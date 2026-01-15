# Carbon Crew Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-crew-decoder">
    <img src="https://img.shields.io/crates/v/carbon-crew-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-crew-decoder">
    <img src="https://img.shields.io/docsrs/carbon-crew-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Crew management program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `CREWiq8qbxvo4SKkAFpVnc6t7CRQC4tAAscsNAENXgrJ`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Crew management program for managing crew members, pack types, and redemptions in the Star Atlas ecosystem.

## Features

- Decodes all Crew account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for crew configuration, pack management, and SFT redemptions

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-crew-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_crew_decoder::CrewDecoder;
use carbon_core::account::AccountDecoder;

let decoder = CrewDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        CrewAccount::CrewConfig(config) => {
            println!("Crew Config: {:?}", config);
        }
        CrewAccount::PackTiers(tiers) => {
            println!("Pack Tiers: {:?}", tiers);
        }
        CrewAccount::PackType(pack_type) => {
            println!("Pack Type: {:?}", pack_type);
        }
        CrewAccount::SftRedemption(redemption) => {
            println!("SFT Redemption: {:?}", redemption);
        }
        CrewAccount::UserRedemption(user_redemption) => {
            println!("User Redemption: {:?}", user_redemption);
        }
    }
}
```

### Account Types

This decoder supports all Crew account types:
- `CrewConfig` - Crew configuration and metadata
- `PackTiers` - Pack tier definitions
- `PackType` - Pack type configuration
- `SftRedemption` - SFT redemption tracking
- `UserRedemption` - User redemption records

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-crew-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
