# Carbon Snapshots Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-snapshots-decoder">
    <img src="https://img.shields.io/crates/v/carbon-snapshots-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-snapshots-decoder">
    <img src="https://img.shields.io/docsrs/carbon-snapshots-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Snapshots program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `snapNQkxsiqDWdbNfz8KVB7e3NPzLwtHHA6WV8kKgUc`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Snapshots program for escrow and locker history tracking.

## Features

- Decodes all Snapshots account types
- Full instruction parsing support
- Integration with Carbon indexing framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-snapshots-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_snapshots_decoder::{SnapshotsDecoder, SnapshotsAccount};
use carbon_core::account::AccountDecoder;

let decoder = SnapshotsDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        // Handle account types
        _ => println!("Decoded account: {:?}", decoded.data),
    }
}
```

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-snapshots-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
