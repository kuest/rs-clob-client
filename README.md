<h1 align="center">
  <img src="https://github.com/user-attachments/assets/0cc687fb-89c4-43fa-a056-d89c307215ad" alt="Kuest" height="96" /><br/>
  Kuest Rust CLOB Client
</h1>

[![Crates.io](https://img.shields.io/crates/v/kuest-client-sdk.svg)](https://crates.io/crates/kuest-client-sdk)
[![PR](https://github.com/kuest/rs-clob-client/actions/workflows/pr.yml/badge.svg)](https://github.com/kuest/rs-clob-client/actions/workflows/pr.yml)

## Read-Only Client

```rust,ignore
use kuest_client_sdk::clob::{Client, Config};

# async fn run() -> kuest_client_sdk::Result<()> {
    let client = Client::new("https://clob.kuest.com", Config::default())?;
    let ok = client.ok().await?;
    println!("{ok}");
    # Ok(())
# }
```

## Wallet-Only Trading

Kuest trading uses Deposit Wallet orders only. Public order builders use `SignatureType::DepositWallet` (`3`) and require a Deposit Wallet funder address.

```rust,ignore
use std::str::FromStr;

use alloy::signers::Signer as _;
use alloy::signers::local::LocalSigner;
use kuest_client_sdk::POLYGON;
use kuest_client_sdk::clob::{Client, Config};
use kuest_client_sdk::clob::types::SignatureType;
use kuest_client_sdk::types::Address;

# async fn run() -> anyhow::Result<()> {
let signer = LocalSigner::from_str("<owner-private-key>")?.with_chain_id(Some(POLYGON));
let deposit_wallet = Address::from_str("<deposit-wallet-address>")?;

let client = Client::new("https://clob.kuest.com", Config::default())?
    .authentication_builder(&signer)
    .signature_type(SignatureType::DepositWallet)
    .funder(deposit_wallet)
    .authenticate()
    .await?;
# Ok(())
# }
```

## Live prices

Enable `realtime` and use `realtime::Client` with `Subscription::crypto("btcusd", Some("chainlink"))` or `Subscription::crypto_twap("btcusd")`. Prices use `op/channel/filter` requests and versioned `channel/seq/ts/payload` frames at `wss://ws-live-data.kuest.com`; operator URLs can be passed to `Client::new`. No client credentials are required for public prices.

`Event::Price` preserves snapshots, full-precision decimal strings, source and market status in its payload. Each subscription owns its socket; dropping the stream closes it. Transport failures and sequence gaps are returned as errors before reconnecting for a fresh snapshot. Invalid filters/source mismatches end the stream. TWAP accepts 60 seconds, not 30. Local comments/activity remain in the separate `rtds` community module; its old price helpers have been removed.
