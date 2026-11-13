//! Public live price streams. Each subscription owns a reconnecting socket;
//! dropping its stream closes the socket. Upstream credentials are server-only.

use std::time::Duration;

use bon::Builder;
use futures::{SinkExt as _, Stream, StreamExt as _};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tokio_tungstenite::{connect_async, tungstenite::Message};

use crate::{
    Result,
    error::{Error, Kind},
};

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Builder)]
pub struct Subscription {
    pub channel: String,
    pub filter: Filter,
}

#[non_exhaustive]
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Builder)]
pub struct Filter {
    pub symbol: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub provider: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub window_seconds: Option<u32>,
}

impl Subscription {
    #[must_use]
    pub fn crypto(symbol: &str, provider: Option<&str>) -> Self {
        Self {
            channel: "price.crypto".into(),
            filter: Filter {
                symbol: symbol.into(),
                provider: provider.map(str::to_owned),
                window_seconds: None,
            },
        }
    }

    #[must_use]
    pub fn crypto_twap(symbol: &str) -> Self {
        Self {
            channel: "price.crypto.twap".into(),
            filter: Filter {
                symbol: symbol.into(),
                provider: Some("chainlink".into()),
                window_seconds: Some(60),
            },
        }
    }

    fn validate(&self) -> Result<()> {
        if !matches!(
            self.channel.as_str(),
            "price.crypto" | "price.equity" | "price.crypto.twap" | "price.equity.twap"
        ) {
            return Err(Error::validation("Unsupported price channel"));
        }
        if self.filter.symbol.is_empty()
            || !self
                .filter
                .symbol
                .bytes()
                .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit())
        {
            return Err(Error::validation(
                "Expected a lowercase canonical symbol, e.g. btcusd",
            ));
        }
        if let Some(provider) = &self.filter.provider
            && !matches!(provider.as_str(), "chainlink" | "pyth")
        {
            return Err(Error::validation("Unsupported price provider"));
        }
        if matches!(
            self.channel.as_str(),
            "price.crypto.twap" | "price.equity.twap"
        ) {
            if self.filter.window_seconds != Some(60)
                || self.filter.provider.as_deref() != Some("chainlink")
            {
                return Err(Error::validation(
                    "TWAP requires a 60-second Chainlink feed",
                ));
            }
        } else if self.filter.window_seconds.is_some() {
            return Err(Error::validation("window_seconds is only valid for TWAP"));
        }
        Ok(())
    }
}

/// The payload is retained verbatim, including precise decimal strings,
/// source attribution, market status, and snapshot `data` arrays.
#[non_exhaustive]
#[derive(Clone, Debug, Deserialize)]
pub struct PriceFrame {
    pub v: u32,
    pub channel: String,
    pub seq: u64,
    pub ts: u64,
    #[serde(default)]
    pub snapshot: bool,
    #[serde(default)]
    pub dropped: u64,
    pub payload: Value,
}

#[non_exhaustive]
#[derive(Clone, Debug)]
pub enum Event {
    Price(PriceFrame),
    Control(Value),
}

#[derive(Clone, Debug)]
pub struct Client {
    endpoint: String,
    pub reconnect_delay: Duration,
    pub idle_timeout: Duration,
}

impl Default for Client {
    fn default() -> Self {
        Self::new("wss://ws-live-data.kuest.com").expect("valid default live-data URL")
    }
}

impl Client {
    pub fn new(endpoint: &str) -> Result<Self> {
        let url = url::Url::parse(endpoint)?;
        if !matches!(url.scheme(), "ws" | "wss") {
            return Err(Error::validation(
                "Live data requires a ws:// or wss:// endpoint",
            ));
        }
        Ok(Self {
            endpoint: endpoint.into(),
            reconnect_delay: Duration::from_secs(1),
            idle_timeout: Duration::from_secs(30),
        })
    }

    /// Opens a public price stream. Reconnects after transport errors or gaps,
    /// resubscribing for a fresh snapshot. Errors are visible to the consumer.
    /// A server rejection ends the stream rather than retrying bad parameters.
    pub fn subscribe(
        &self,
        subscription: Subscription,
    ) -> Result<impl Stream<Item = Result<Event>> + use<>> {
        subscription.validate()?;
        let client = self.clone();
        let request = json!({"op":"subscribe","subscriptions":[subscription]}).to_string();
        Ok(async_stream::stream! {
            loop {
                match connect_async(&client.endpoint).await {
                    Err(error) => yield Err(Error::with_source(Kind::WebSocket, error)),
                    Ok((mut socket, _)) => {
                        if let Err(error) = socket.send(Message::Text(request.clone().into())).await {
                            yield Err(Error::with_source(Kind::WebSocket, error));
                        } else {
                            let mut last_seq = None;
                            let mut ping = tokio::time::interval(Duration::from_secs(5));
                            let mut last_message = tokio::time::Instant::now();
                            loop {
                                tokio::select! {
                                    _ = ping.tick() => {
                                        if last_message.elapsed() >= client.idle_timeout {
                                            yield Err(Error::validation("Live price stream timed out; reconnecting"));
                                            break;
                                        }
                                        if let Err(error) = socket.send(Message::Text(r#"{"op":"ping"}"#.into())).await {
                                            yield Err(Error::with_source(Kind::WebSocket, error));
                                            break;
                                        }
                                    }
                                    message = socket.next() => {
                                        last_message = tokio::time::Instant::now();
                                        match message {
                                            Some(Ok(Message::Text(text))) => {
                                                let value: Value = match serde_json::from_str(&text) {
                                                    Ok(value) => value,
                                                    Err(error) => { yield Err(error.into()); break; }
                                                };
                                                if value.get("op").and_then(Value::as_str) == Some("error") {
                                                    yield Err(Error::validation(format!("Live price subscription rejected: {value}")));
                                                    if matches!(value.get("code").and_then(Value::as_str), Some("upstream_gap" | "upstream_unavailable")) {
                                                        break;
                                                    }
                                                    return;
                                                }
                                                if value.get("channel").is_some() && value.get("seq").is_some() {
                                                    let frame: PriceFrame = match serde_json::from_value(value) {
                                                        Ok(frame) => frame,
                                                        Err(error) => { yield Err(error.into()); break; }
                                                    };
                                                    if frame.v != 1 || frame.channel != subscription.channel {
                                                        yield Err(Error::validation("Unexpected live price envelope"));
                                                        return;
                                                    }
                                                    if let Err(error) = validate_source(&subscription, &frame.payload) {
                                                        yield Err(error);
                                                        return;
                                                    }
                                                    if frame.dropped > 0 || (!frame.snapshot && last_seq.is_some_and(|previous: u64| previous.checked_add(1) != Some(frame.seq))) {
                                                        yield Err(Error::validation("Live price sequence gap; reconnecting for snapshot"));
                                                        break;
                                                    }
                                                    last_seq = Some(frame.seq);
                                                    yield Ok(Event::Price(frame));
                                                } else {
                                                    if value.get("op").and_then(Value::as_str) == Some("subscribed") {
                                                        let filter = &value["filter"];
                                                        if value["channel"] != subscription.channel || filter["symbol"] != subscription.filter.symbol || subscription.filter.provider.as_ref().is_some_and(|provider| filter["provider"] != *provider) || subscription.filter.window_seconds.is_some_and(|window| filter["window_seconds"] != window) {
                                                            yield Err(Error::validation("Live price subscription acknowledgement changed the requested filter"));
                                                            return;
                                                        }
                                                    }
                                                    yield Ok(Event::Control(value));
                                                }
                                            }
                                            Some(Ok(Message::Ping(data))) => {
                                                if let Err(error) = socket.send(Message::Pong(data)).await {
                                                    yield Err(Error::with_source(Kind::WebSocket, error)); break;
                                                }
                                            }
                                            Some(Ok(Message::Close(_))) | None => {
                                                yield Err(Error::validation("Live price connection closed; reconnecting")); break;
                                            }
                                            Some(Err(error)) => { yield Err(Error::with_source(Kind::WebSocket, error)); break; }
                                            _ => {}
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                tokio::time::sleep(client.reconnect_delay).await;
            }
        })
    }
}

fn validate_source(subscription: &Subscription, payload: &Value) -> Result<()> {
    let symbol = &subscription.filter.symbol;
    if let Some(points) = payload["data"].as_array() {
        if payload["symbol"] != *symbol
            || points
                .iter()
                .any(|point| point.get("symbol").is_some_and(|value| value != symbol))
        {
            return Err(Error::validation("Live price symbol mismatch"));
        }
    } else if payload["symbol"] != *symbol {
        return Err(Error::validation("Live price symbol mismatch"));
    }
    if subscription
        .filter
        .provider
        .as_ref()
        .is_some_and(|provider| payload["source"] != *provider)
    {
        return Err(Error::validation("Live price source mismatch"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn reconnects_after_gap_and_resubscribes_anonymously() {
        let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address = listener.local_addr().unwrap();
        let server = tokio::spawn(async move {
            for round in 0..2 {
                let (tcp, _) = listener.accept().await.unwrap();
                let mut socket = tokio_tungstenite::accept_async(tcp).await.unwrap();
                let request = socket.next().await.unwrap().unwrap().into_text().unwrap();
                assert_eq!(
                    serde_json::from_str::<Value>(&request).unwrap(),
                    json!({"op":"subscribe","subscriptions":[{"channel":"price.crypto","filter":{"symbol":"btcusd","provider":"chainlink"}}]})
                );
                for seq in if round == 0 { vec![1, 3] } else { vec![1] } {
                    let frame = json!({"v":1,"channel":"price.crypto","seq":seq,"ts":1000,"snapshot":seq==1,"payload":{"symbol":"btcusd","source":"chainlink","full_accuracy_value":"123.000000000001"}});
                    socket
                        .send(Message::Text(frame.to_string().into()))
                        .await
                        .unwrap();
                }
                while let Some(Ok(message)) = socket.next().await {
                    if let Message::Text(text) = message
                        && serde_json::from_str::<Value>(&text).unwrap()["op"] == "ping"
                    {
                        socket
                            .send(Message::Text(json!({"op":"pong"}).to_string().into()))
                            .await
                            .unwrap();
                    }
                }
            }
        });
        let mut client = Client::new(&format!("ws://{address}")).unwrap();
        client.reconnect_delay = Duration::from_millis(1);
        let stream = client
            .subscribe(Subscription::crypto("btcusd", Some("chainlink")))
            .unwrap();
        futures::pin_mut!(stream);
        tokio::time::timeout(Duration::from_secs(5), async {
            assert!(matches!(
                stream.next().await.unwrap().unwrap(),
                Event::Price(_)
            ));
            stream.next().await.unwrap().unwrap_err();
            assert!(matches!(
                stream.next().await.unwrap().unwrap(),
                Event::Price(_)
            ));
        })
        .await
        .unwrap();
        server.abort();
    }

    #[test]
    fn source_mismatch_is_not_relabelled() {
        assert!(
            validate_source(
                &Subscription::crypto("btcusd", Some("chainlink")),
                &json!({"symbol":"btcusd","source":"pyth"})
            )
            .is_err()
        );
    }

    #[test]
    fn subscribe_has_object_filter_without_credentials() {
        let sub = Subscription::crypto_twap("btcusd");
        sub.validate().unwrap();
        assert_eq!(
            serde_json::to_value(sub).unwrap(),
            json!({"channel":"price.crypto.twap","filter":{"symbol":"btcusd","provider":"chainlink","window_seconds":60}})
        );
    }
    #[test]
    fn rejects_obsolete_window_and_source() {
        let mut sub = Subscription::crypto_twap("btcusd");
        sub.filter.window_seconds = Some(30);
        assert!(sub.validate().is_err());
        sub.filter.window_seconds = Some(60);
        sub.filter.provider = None;
        assert!(sub.validate().is_err());
        sub.filter.provider = Some("binance".into());
        assert!(sub.validate().is_err());
    }
    #[test]
    fn preserves_snapshot_precision_and_source() {
        let frame: PriceFrame = serde_json::from_value(json!({"v":1,"channel":"price.crypto","seq":3,"ts":1000,"snapshot":true,"payload":{"symbol":"btcusd","source":"chainlink","data":[{"timestamp":1000,"full_accuracy_value":"100000.123456789123456789"}]}})).unwrap();
        assert_eq!(
            frame.payload["data"][0]["full_accuracy_value"],
            "100000.123456789123456789"
        );
        assert_eq!(frame.payload["source"], "chainlink");
        assert!(frame.snapshot);
        let subscription = Subscription::crypto("btcusd", Some("chainlink"));
        validate_source(&subscription, &frame.payload).unwrap();
        let mut missing_symbol = frame.payload.clone();
        missing_symbol.as_object_mut().unwrap().remove("symbol");
        assert!(validate_source(&subscription, &missing_symbol).is_err());
        let mut wrong_batch = frame.payload.clone();
        wrong_batch["symbol"] = json!("ethusd");
        assert!(validate_source(&subscription, &wrong_batch).is_err());
        let empty = json!({"symbol":"btcusd","source":"chainlink","data":[]});
        validate_source(&subscription, &empty).unwrap();
        let mut mismatched = frame.payload;
        mismatched["data"][0]["symbol"] = json!("ethusd");
        assert!(validate_source(&subscription, &mismatched).is_err());
    }
}
