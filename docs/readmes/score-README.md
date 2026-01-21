# Carbon Score Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-score-decoder">
    <img src="https://img.shields.io/crates/v/carbon-score-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-score-decoder">
    <img src="https://img.shields.io/docsrs/carbon-score-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Score program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `FLEET1qqzpexyaDpqb2DGsSzE2sDCizewCg9WjrA6DBW`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Score program for the original fleet staking system.

## Features

- Decodes all Score account types
- Full instruction parsing support
- Integration with Carbon indexing framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-score-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_score_decoder::{ScoreDecoder, ScoreAccount};
use carbon_core::account::AccountDecoder;

let decoder = ScoreDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        // Handle account types
        _ => println!("Decoded account: {:?}", decoded.data),
    }
}
```

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-score-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
