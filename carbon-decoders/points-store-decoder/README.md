# Carbon Points Store Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-points-store-decoder">
    <img src="https://img.shields.io/crates/v/carbon-points-store-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-points-store-decoder">
    <img src="https://img.shields.io/docsrs/carbon-points-store-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Points Store program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `PsToRxhEPScGt1Bxpm7zNDRzaMk31t8Aox7fyewoVse`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Points Store program for managing token purchases with points and epoch-based token redemptions. Enables players to spend points to buy tokens directly or participate in time-locked redemption pools with dynamic pricing based on participation.

## Features

- Decodes all Points Store account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for direct token purchases and epoch-based redemptions
- Faction-based redemption configurations

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-points-store-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_points_store_decoder::{PointsStoreDecoder, PointsStoreAccount};
use carbon_core::account::AccountDecoder;

let decoder = PointsStoreDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        PointsStoreAccount::PointsStore(store) => {
            println!("Points Store: {:?}", store);
            println!("Point Category: {}", store.point_category);
            println!("Profile: {}", store.profile);
            println!("Bank: {}", store.bank);
            println!("Price (points per token): {}", store.price);
            println!("Signer Bump: {}", store.signer_bump);
        }
        PointsStoreAccount::RedemptionConfig(config) => {
            println!("Redemption Config: {:?}", config);
            println!("Point Category: {}", config.point_category);
            println!("Profile: {}", config.profile);
            println!("Faction: {}", config.faction);
            println!("Bank: {}", config.bank);
            println!("Allow Only Current Epoch: {}", config.allow_only_current_epoch != 0);
        }
        PointsStoreAccount::UserRedemption(redemption) => {
            println!("User Redemption: {:?}", redemption);
            println!("Profile: {}", redemption.profile);
            println!("Point Category: {}", redemption.point_category);
            println!("User Points Account: {}", redemption.user_points_account);
            println!("Config: {}", redemption.config);
            println!("Points: {}", redemption.points);
            println!("Day Index: {}", redemption.day_index);
        }
    }
}
```

### Working with Faction Values

The `RedemptionConfig.faction` field is stored as a u8. Map values to factions:

```rust
// Faction values (from profile-faction program)
const FACTION_UNALIGNED: u8 = 0;
const FACTION_MUD: u8 = 1;
const FACTION_ONI: u8 = 2;
const FACTION_USTUR: u8 = 3;

let faction_name = match config.faction {
    0 => "Unaligned",
    1 => "MUD",
    2 => "ONI",
    3 => "Ustur",
    _ => "Unknown",
};

println!("Redemption config is for faction: {}", faction_name);
```

### Working with Boolean Flags

The Points Store program uses u8 fields for boolean flags. Check them like this:

```rust
// Check if redemptions are limited to current epoch only
if config.allow_only_current_epoch != 0 {
    println!("Only the current epoch can be redeemed");
} else {
    println!("Past epochs can also be redeemed");
}
```

### Account Types

This decoder supports all Points Store account types:

- **`PointsStore`** - A store where tokens can be purchased directly with points at a fixed price
  - Stores the point category, managing profile, token bank, and price per token
  - Includes a signer bump for PDA-based token transfers
  - Simple direct exchange: spend X points, receive Y tokens

- **`RedemptionConfig`** - Configuration for epoch-based token redemptions
  - Enables time-locked redemption pools where users contribute points
  - Tokens are distributed proportionally based on each user's contribution
  - Faction-specific configurations allow different redemption rules per faction
  - Supports dynamic redemption epochs (stored as RemainingData in on-chain account)
  - Can restrict redemptions to only the current epoch

- **`UserRedemption`** - Tracks a user's points contribution for a specific redemption epoch
  - Links to the user's profile and points account
  - Stores the points submitted for redemption
  - Tied to a specific day index (24-hour epoch)
  - Used to calculate proportional token distribution when claiming

### Store vs Redemption

The Points Store program provides two mechanisms for exchanging points for tokens:

#### Direct Store Purchase
- **Fixed pricing**: Set price per token
- **Immediate exchange**: Spend points, receive tokens instantly
- **Simple model**: Like buying items from a store

#### Epoch-Based Redemption
- **Pool-based**: Users contribute points to a daily pool
- **Proportional distribution**: Tokens distributed based on contribution percentage
- **Time-locked**: Must wait for epoch to complete
- **Dynamic pricing**: Effective rate depends on total participation
- **Faction-specific**: Different configs for different factions

Example: If 1000 tokens are available for an epoch, and you contributed 100 points out of a total 1000 points submitted, you can claim 100 tokens (10% of the pool).

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-points-store-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
