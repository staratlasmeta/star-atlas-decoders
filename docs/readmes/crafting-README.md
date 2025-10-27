# Carbon Crafting Decoder

<p align="center">
  <a href="https://crates.io/crates/carbon-crafting-decoder">
    <img src="https://img.shields.io/crates/v/carbon-crafting-decoder?logo=rust" />
  </a>
  <a href="https://docs.rs/carbon-crafting-decoder">
    <img src="https://img.shields.io/docsrs/carbon-crafting-decoder?logo=docsdotrs" />
  </a>
  <a href="https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE">
    <img src="https://img.shields.io/badge/license-Apache%202.0-blue" />
  </a>
</p>

Rust decoder for the Star Atlas Crafting program on Solana, generated using [Carbon CLI](https://github.com/sevenlabs-hq/carbon).

## Program Information

- **Program ID**: `CRAFT2RPXPJWCEix4WpJST3E7NLf79GTqZUL75wngXo5`
- **Network**: Solana Mainnet
- **Description**: Star Atlas Crafting program for managing recipes, crafting facilities, and crafting processes. Enables players to create items using consumable and non-consumable inputs at specialized facilities.

## Features

- Decodes all Crafting account types
- Full instruction parsing support
- Integration with Carbon indexing framework
- Type-safe enum support for process/recipe status and location types
- Support for recipes with dynamic inputs/outputs

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
carbon-crafting-decoder = "0.10.0"
```

### Decoding Accounts

```rust
use carbon_crafting_decoder::{CraftingDecoder, CraftingAccount};
use carbon_core::account::AccountDecoder;

let decoder = CraftingDecoder;
let decoded_account = decoder.decode_account(&account);

if let Some(decoded) = decoded_account {
    match decoded.data {
        CraftingAccount::Recipe(recipe) => {
            println!("Recipe: {:?}", recipe);
            println!("Duration: {}", recipe.duration);
            println!("Status: {:?}", recipe.status);
            println!("Consumables: {}", recipe.consumables_count);
            println!("Outputs: {}", recipe.outputs_count);
            println!("Total items: {}", recipe.recipe_items.len());
            // Access recipe inputs/outputs from remaining data
            for (i, item) in recipe.recipe_items.iter().enumerate() {
                println!("Item {}: amount={}, mint={}", i, item.amount, item.mint);
            }
        }
        CraftingAccount::CraftingFacility(facility) => {
            println!("Crafting Facility: {:?}", facility);
            println!("Location: {}", facility.location);
            println!("Location Type: {:?}", facility.location_type);
            println!("Efficiency: {}", facility.efficiency);
            println!("Max Concurrent Processes: {}", facility.max_concurrent_processes);
            println!("Recipe categories: {}", facility.recipe_categories.len());
            // Access recipe category pubkeys from remaining data
            for (i, category) in facility.recipe_categories.iter().enumerate() {
                println!("Category {}: {}", i, category);
            }
        }
        CraftingAccount::CraftingProcess(process) => {
            println!("Crafting Process: {:?}", process);
            println!("Recipe: {}", process.recipe);
            println!("Status: {:?}", process.status);
            println!("Start Time: {}", process.start_time);
            println!("End Time: {}", process.end_time);
        }
        CraftingAccount::CraftableItem(item) => {
            println!("Craftable Item: {:?}", item);
            println!("Mint: {}", item.mint);
        }
        CraftingAccount::Domain(domain) => {
            println!("Crafting Domain: {:?}", domain);
            println!("Authority Profile: {}", domain.authority_profile);
        }
        CraftingAccount::RecipeCategory(category) => {
            println!("Recipe Category: {:?}", category);
        }
    }
}
```

### Working with Status Enums

The decoder provides type-safe enums for status and type fields:

```rust
use carbon_crafting_decoder::{ProcessStatus, RecipeStatus, LocationType};

// Check process status
match process.status {
    ProcessStatus::Initialized => println!("Process initialized"),
    ProcessStatus::Started => println!("Process in progress"),
    ProcessStatus::Completed => println!("Process complete"),
}

// Check recipe status
match recipe.status {
    RecipeStatus::Initializing => println!("Recipe being set up"),
    RecipeStatus::Active => println!("Recipe available for use"),
    RecipeStatus::Deactivated => println!("Recipe disabled"),
}

// Check facility location type
match facility.location_type {
    LocationType::Starbase => println!("Located at a starbase"),
}
```

### Account Types

This decoder supports all Crafting account types:
- `Recipe` - Blueprint defining inputs, outputs, duration, and requirements for crafting. Includes `recipe_items` (Vec<RecipeInputsOutputs>) containing all consumable/non-consumable inputs and outputs with amounts and mint addresses.
- `CraftingFacility` - Physical location where crafting occurs with efficiency modifiers. Includes `recipe_categories` (Vec<Pubkey>) listing all recipe categories available at this facility.
- `CraftingProcess` - Active crafting operation tracking progress and status
- `CraftableItem` - Item that can be produced through crafting
- `Domain` - Administrative domain controlling crafting permissions
- `RecipeCategory` - Classification system for organizing recipes

### Status Enums

#### ProcessStatus
- `Initialized` - Process created but not yet started
- `Started` - Crafting in progress
- `Completed` - Crafting finished, ready to claim outputs

#### RecipeStatus
- `Initializing` - Recipe being configured
- `Active` - Recipe available for use
- `Deactivated` - Recipe disabled and unavailable

#### LocationType
- `Starbase` - Crafting facility located at a starbase

## Documentation

Full documentation is available at [docs.rs](https://docs.rs/carbon-crafting-decoder).

## Repository

See the [main repository](https://github.com/staratlasmeta/star-atlas-decoders) for build instructions and contribution guidelines.

## License

Licensed under the [Apache-2.0](https://github.com/staratlasmeta/star-atlas-decoders/blob/main/LICENSE) license.
