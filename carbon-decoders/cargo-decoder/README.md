# Carbon Cargo Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-cargo-decoder">
    <img src="https://img.shields.io/crates/v/carbon-cargo-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-cargo-decoder">
    <img src="https://img.shields.io/docsrs/carbon-cargo-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Cargo program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `Cargo2VNTPPTi9c1vq1Jw5d3BWUNr18MjRtSupAghKEk`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Cargo program for managing resource containers (pods) with dynamic stat tracking, cargo types, and token-based resource management.

## Features

- Decodes all Cargo account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Permission bitflags support for cargo operations
- Support for dynamic cargo stats and pod management

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-cargo-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_cargo_decoder::{CargoDecoder, CargoAccount};
use carbon_core::account::AccountDecoder;

let decoder = CargoDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        CargoAccount::CargoPod(pod) => {
            println!("Cargo Pod: {:?}", pod);
            println!("Authority: {}", pod.authority);
            println!("Open Token Accounts: {}", pod.open_token_accounts);
            println!("Cargo Contents: {:?}", pod.cargo_contents);
        }
        CargoAccount::CargoType(cargo_type) => {
            println!("Cargo Type: {:?}", cargo_type);
            println!("Mint: {}", cargo_type.mint);
            println!("Stats Count: {}", cargo_type.stats_count);
            println!("Cargo Stats: {:?}", cargo_type.cargo_stats);
        }
        CargoAccount::CargoStatsDefinition(definition) => {
            println!("Stats Definition: {:?}", definition);
            println!("Authority: {}", definition.authority);
            println!("Sequence ID: {}", definition.seq_id);
        }
    }
}
```

### Working with Permissions

The decoder includes ergonomic permission handling with bitflags:

```rust
use carbon_cargo_decoder::CargoPermissions;

// Create permissions from u64 or bytes
let perms = CargoPermissions::from_u64(0b111);

// Check individual permissions
if perms.contains(CargoPermissions::MANAGE_DEFINITION) {
    println!("Can manage cargo definitions");
}

if perms.contains(CargoPermissions::CREATE_CARGO_TYPE) {
    println!("Can create new cargo types");
}

if perms.contains(CargoPermissions::MANAGE_CARGO_TYPE) {
    println!("Can update existing cargo types");
}

// Combine permissions
let admin_perms = CargoPermissions::MANAGE_DEFINITION
    | CargoPermissions::CREATE_CARGO_TYPE
    | CargoPermissions::MANAGE_CARGO_TYPE;

// Convert to/from bytes for storage
let bytes = admin_perms.to_le_bytes();
let restored = CargoPermissions::from_le_bytes(bytes);
```

### Account Types

This decoder supports all Cargo account types:
- `CargoPod` - Container for resources with dynamic stat tracking
  - Contains `cargo_contents: Vec<u64>` with the pod's cargo values
- `CargoType` - Definition of a specific cargo type with associated stats
  - Contains `cargo_stats: Vec<u64>` with stat values (length = `stats_count`)
- `CargoStatsDefinition` - Global configuration for cargo stat definitions

### Permission Flags

The `CargoPermissions` bitflags type includes:
- `MANAGE_DEFINITION` - Can initialize and update cargo definitions
- `CREATE_CARGO_TYPE` - Can create new cargo types
- `MANAGE_CARGO_TYPE` - Can update existing cargo types

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-cargo-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
