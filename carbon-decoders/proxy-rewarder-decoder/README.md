# Carbon Proxy Rewarder Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-proxy-rewarder-decoder">
    <img src="https://img.shields.io/crates/v/carbon-proxy-rewarder-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-proxy-rewarder-decoder">
    <img src="https://img.shields.io/docsrs/carbon-proxy-rewarder-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Proxy Rewarder program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `gateVwTnKyFrE8nxUUgfzoZTPKgJQZUbLsEidpG4Dp2`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Proxy Rewarder program for reward distribution.

## Features

- Decodes all Proxy Rewarder account types
- Full instruction parsing support
- Integration with Carbon indexing framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-proxy-rewarder-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_proxy_rewarder_decoder::{ProxyRewarderDecoder, ProxyRewarderAccount};
use carbon_core::account::AccountDecoder;

let decoder = ProxyRewarderDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        // Handle account types
        _ => println!("Decoded account: {:?}", decoded.data),
    }
}
```

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-proxy-rewarder-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
