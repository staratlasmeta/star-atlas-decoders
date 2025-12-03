# Carbon SAGE Holosim Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-sage-holosim-decoder">
    <img src="https://img.shields.io/crates/v/carbon-sage-holosim-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-sage-holosim-decoder">
    <img src="https://img.shields.io/docsrs/carbon-sage-holosim-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas SAGE Holosim program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `SAgEeT8u14TE69JXtanGSgNkEdoPUcLabeyZD2uw8x9`
- **Network**: ATMTA Atlasnet
- **Description**: Star Atlas SAGE (Starbase and Galactic Expansion) Holosim program for simulated space operations, fleet activities, and resource management.

## Features

- Decodes all SAGE Holosim account types
- Custom deserialization for complex accounts (Fleet, StarbasePlayer)
- Full instruction parsing support
- Integration with Carbon indexing framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-sage-holosim-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_sage_holosim_decoder::SageDecoder;
use carbon_core::account::AccountDecoder;

let decoder = SageDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        SageAccount::Fleet(fleet) => {
            println!("Fleet: {:?}", fleet);
        }
        SageAccount::GameState(game_state) => {
            println!("Game State: {:?}", game_state);
        }
        // ... handle other account types
        _ => {}
    }
}
```

### Account Types

This decoder supports all SAGE Holosim account types including:
- `Fleet` - Fleet management with custom state deserialization
- `StarbasePlayer` - Player starbase interactions with dynamic escrows
- `Game` - Game configuration and state
- `GameState` - Current game state
- `Starbase` - Starbase definitions
- `MineItem` - Mining resources
- `Planet` - Planetary bodies
- `Sector` - Space sectors
- And more...

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-sage-holosim-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions, patch development workflow, and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
