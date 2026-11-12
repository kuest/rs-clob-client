use std::io::{self, Write as _};

use futures::StreamExt as _;
use kuest_client_sdk::realtime::{Client, Subscription};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();
    let stream = client.subscribe(Subscription::crypto("btcusd", Some("chainlink")))?;
    futures::pin_mut!(stream);
    while let Some(frame) = stream.next().await {
        match frame {
            Ok(event) => writeln!(io::stdout(), "{event:?}")?,
            Err(error) => writeln!(io::stderr(), "{error}")?,
        }
    }
    anyhow::bail!("Live price stream terminated")
}
