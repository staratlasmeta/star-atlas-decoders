# Carbon Claim Stake Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-claim-stake-decoder">
    <img src="https://img.shields.io/crates/v/carbon-claim-stake-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-claim-stake-decoder">
    <img src="https://img.shields.io/docsrs/carbon-claim-stake-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Claim Stake program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `STAKEr4Bh8sbBMoAVmTDBRqouPzgdocVrvtjmhJhd65`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Claim Stake program for staking functionality.

## Features

- Decodes all Claim Stake account types
- Full instruction parsing support
- Integration with Carbon indexing framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-claim-stake-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_claim_stake_decoder::{ClaimStakeDecoder, ClaimStakeAccount};
use carbon_core::account::AccountDecoder;

let decoder = ClaimStakeDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        // Handle account types
        _ => println!("Decoded account: {:?}", decoded.data),
    }
}
```

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-claim-stake-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
