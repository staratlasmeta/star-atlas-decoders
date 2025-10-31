# Carbon Points Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-points-decoder">
    <img src="https://img.shields.io/crates/v/carbon-points-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-points-decoder">
    <img src="https://img.shields.io/docsrs/carbon-points-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Points program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `Point2iBvz7j5TMVef8nEgpmz4pDr7tU7v3RjAfkQbM`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Points program for managing player progression systems with categories, levels, and point tracking. Supports daily point limits, level upgrades, token-gated progression, and spendable points mechanics.

## Features

- Decodes all Points account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Support for point categories with configurable levels
- Token-gated level progression with license types
- Daily point tracking and limits

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-points-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_points_decoder::{PointsDecoder, PointsAccount};
use carbon_core::account::AccountDecoder;

let decoder = PointsDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        PointsAccount::PointCategory(category) => {
            println!("Point Category: {:?}", category);
            println!("Profile: {}", category.profile);
            println!("Point Limit: {}", category.point_limit);
            println!("Is Spendable: {}", category.is_spendable != 0);
            println!("Token Required: {}", category.token_required != 0);
            if category.token_required != 0 {
                println!("Token Mint: {}", category.token_mint);
                println!("Token Qty: {}", category.token_qty);
            }
        }
        PointsAccount::UserPointsAccount(user_points) => {
            println!("User Points Account: {:?}", user_points);
            println!("Profile: {}", user_points.profile);
            println!("Point Category: {}", user_points.point_category);
            println!("Earned Points: {}", user_points.earned_points);
            println!("Spent Points: {}", user_points.spent_points);
            println!("Current Level: {}", user_points.level);
            println!("Daily Earned: {}", user_points.daily_earned_points);
            println!("Last Earned Timestamp: {}", user_points.last_earned_points_timestamp);
        }
        PointsAccount::PointsModifier(modifier) => {
            println!("Points Modifier: {:?}", modifier);
            println!("Point Category: {}", modifier.point_category);
            println!("Can Increment: {}", modifier.can_increment != 0);
            println!("Can Decrement: {}", modifier.can_decrement != 0);
        }
    }
}
```

### Working with License Types

The Points program uses license types to control level progression requirements:

```rust
use carbon_points_decoder::{LicenseType, PointsLevelLicenseType};

// LicenseType is used for level upgrade requirements with quantity
match license_type {
    LicenseType::None => {
        println!("No license required for this level");
    }
    LicenseType::Burn { quantity } => {
        println!("Requires burning {} tokens to unlock this level", quantity);
    }
    LicenseType::Vault { quantity } => {
        println!("Requires {} tokens transferred to vault for this level", quantity);
    }
}

// PointsLevelLicenseType is a simpler enum without quantities
match license {
    PointsLevelLicenseType::None => println!("No license"),
    PointsLevelLicenseType::Burn => println!("Burn license"),
    PointsLevelLicenseType::Vault => println!("Vault license"),
}
```

### Working with Boolean Flags

The Points program uses u8 fields for boolean flags. Check them like this:

```rust
// Check if a point category requires tokens
if category.token_required != 0 {
    println!("This category requires token: {}", category.token_mint);
}

// Check if points are spendable
if category.is_spendable != 0 {
    println!("Points can be spent in this category");
}

// Check if tokens should be transferred to vault
if category.transfer_tokens_to_vault != 0 {
    println!("Tokens will be transferred to vault: {}", category.token_vault);
}

// Check modifier permissions
if modifier.can_increment != 0 {
    println!("This modifier can increment points");
}

if modifier.can_decrement != 0 {
    println!("This modifier can decrement points");
}
```

### Account Types

This decoder supports all Points account types:

- **`PointCategory`** - Defines a category of points with levels, limits, and token requirements
  - Contains point limits, spendability settings, and token-gating configuration
  - Supports dynamic levels array (stored as RemainingData in the on-chain account)
  - Can require tokens for point earning or level upgrades

- **`UserPointsAccount`** - Tracks a user's points and level for a specific category
  - Stores earned points, spent points, and current level
  - Includes daily point tracking with timestamps
  - Links to a profile and point category

- **`PointsModifier`** - Grants authority to modify points for a specific category
  - Controls increment and decrement permissions
  - Used to delegate point management authority

### License Types

The Points program uses two license type enums:

#### LicenseType
An enum with associated data for level upgrade requirements:
- `None` - No license or tokens required
- `Burn { quantity: u64 }` - Requires burning the specified quantity of tokens
- `Vault { quantity: u64 }` - Requires transferring tokens to a vault

#### PointsLevelLicenseType
A simpler enum without quantities:
- `None` - No license required
- `Burn` - Burn-type license
- `Vault` - Vault-type license

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-points-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
