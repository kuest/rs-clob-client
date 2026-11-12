use futures::StreamExt as _;
use kuest_client_sdk::realtime::{Client, Subscription};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::default();
    let stream = client.subscribe(Subscription::crypto("btcusd", Some("chainlink")))?;
    futures::pin_mut!(stream);
    while let Some(frame) = stream.next().await {
        println!("{:?}", frame?);
    }
    Ok(())
}
