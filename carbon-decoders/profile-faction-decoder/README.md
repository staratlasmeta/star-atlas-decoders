# Carbon Profile Faction Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-profile-faction-decoder">
    <img src="https://img.shields.io/crates/v/carbon-profile-faction-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-profile-faction-decoder">
    <img src="https://img.shields.io/docsrs/carbon-profile-faction-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Profile Faction program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `pFACSRuobDmvfMKq1bAzwj27t6d2GJhSCHb1VcfnRmq`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Profile Faction program for managing player faction affiliations within the Star Atlas universe.

## Features

- Decodes all Profile Faction account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Type-safe Faction enum for faction selection

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-profile-faction-decoder = "0.12.0"
```

### Decoding Accounts

```rust
use carbon_profile_faction_decoder::{ProfileFactionDecoder, ProfileFactionAccount};
use carbon_core::account::AccountDecoder;

let decoder = ProfileFactionDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        ProfileFactionAccount::ProfileFactionAccount(profile_faction) => {
            println!("Profile: {:?}", profile_faction.profile);
            println!("Faction: {:?}", profile_faction.faction);
            println!("Version: {}", profile_faction.version);
        }
    }
}
```

### Working with Factions

The decoder includes a type-safe `Faction` enum:

```rust
use carbon_profile_faction_decoder::{ProfileFactionAccount, Faction};

let profile_faction: ProfileFactionAccount = /* ... */;

// Pattern match on faction
match profile_faction.faction {
    Faction::Unaligned => println!("Player has not chosen a faction"),
    Faction::MUD => println!("Player is aligned with MUD"),
    Faction::ONI => println!("Player is aligned with ONI"),
    Faction::Ustur => println!("Player is aligned with Ustur"),
}

// Check specific faction
if profile_faction.faction == Faction::MUD {
    println!("MUD faction member detected");
}
```

### Account Types

This decoder supports the Profile Faction account type:
- `ProfileFactionAccount` - Stores a profile's enlisted faction on-chain
  - `version: u8` - Account data version
  - `profile: Pubkey` - The profile this faction enlistment is for
  - `faction: Faction` - The faction of the profile (type-safe enum)
  - `bump: u8` - PDA bump seed

### Faction Enum

The `Faction` enum represents the available factions in Star Atlas:

- `Unaligned` - Faction is not selected yet (default)
- `MUD` - The MUD faction
- `ONI` - The ONI faction
- `Ustur` - The Ustur faction

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-profile-faction-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
