# Carbon ATLAS Fee Payer Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-atlas-fee-payer-decoder">
    <img src="https://img.shields.io/crates/v/carbon-atlas-fee-payer-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-atlas-fee-payer-decoder">
    <img src="https://img.shields.io/docsrs/carbon-atlas-fee-payer-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas ATLAS fee payer program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `APR1MEny25pKupwn72oVqMH4qpDouArsX8zX4VwwfoXD`
- **Network**: Solana Mainnet
- **Description**: Star Atlas ATLAS fee payer program for managing transaction fees and fee payer accounts in the Star Atlas ecosystem.

## Features

- Decodes all ATLAS fee payer account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for fee payer management and rate configuration

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-atlas-fee-payer-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_atlas_fee_payer_decoder::AtlasFeePayerDecoder;
use carbon_core::account::AccountDecoder;

let decoder = AtlasFeePayerDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        AtlasFeePayerAccount::FeePayer(payer) => {
            println!("Fee Payer: {:?}", payer);
        }
        AtlasFeePayerAccount::FeePayerRates(rates) => {
            println!("Fee Payer Rates: {:?}", rates);
        }
    }
}
```

### Account Types

This decoder supports all ATLAS fee payer account types:
- `FeePayer` - Fee payer account configuration
- `FeePayerRates` - Fee rate configuration for transactions

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-atlas-fee-payer-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
