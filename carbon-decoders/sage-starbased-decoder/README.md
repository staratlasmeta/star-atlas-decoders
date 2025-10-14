# Carbon SAGE Starbased Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-sage-starbased-decoder">
    <img src="https://img.shields.io/crates/v/carbon-sage-starbased-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-sage-starbased-decoder">
    <img src="https://img.shields.io/docsrs/carbon-sage-starbased-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas SAGE Starbased program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `SAGE2HAwep459SNq61LHvjxPk4pLPEJLoMETef7f7EE`
- **Network**: Solana Mainnet
- **Description**: Star Atlas SAGE (Starbase and Galactic Expansion) Starbased program for managing starbase operations, fleet management, and resource processing.

## Features

- Decodes all SAGE Starbased account types
- Custom deserialization for complex accounts (Fleet, StarbasePlayer)
- Full instruction parsing support
- Integration with Carbon indexing framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-sage-starbased-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_sage_starbased_decoder::SageDecoder;
use carbon_core::account::AccountDecoder;

let decoder = SageDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        SageAccount::Fleet(fleet) => {
            println!("Fleet: {:?}", fleet);
        }
        SageAccount::Starbase(starbase) => {
            println!("Starbase: {:?}", starbase);
        }
        // ... handle other account types
        _ => {}
    }
}
```

### Account Types

This decoder supports all SAGE Starbased account types including:
- `Fleet` - Fleet management with custom state deserialization
- `StarbasePlayer` - Player starbase interactions with dynamic escrows
- `Game` - Game configuration and state
- `GameState` - Current game state
- `Starbase` - Starbase definitions
- `Ship` - Ship configurations
- `CraftingInstance` - Crafting operations
- And more...

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-sage-starbased-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions, patch development workflow, and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
