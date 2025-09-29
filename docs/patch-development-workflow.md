# Decoder Patch Development Workflow

This document describes how to create and maintain patches for generated decoders.

## Overview

When carbon-cli generates decoders from Solana IDLs, sometimes we need custom implementations for complex account deserialization. Rather than maintaining full copies of modified files, we use git patches with a git-based workflow.

## Improved Git-Based Workflow

### 1. Build Clean Decoder
```bash
just build-sage-starbased
```
This:
- Generates decoder from mainnet IDL
- Prepares it (fixes arrays, removes serde attributes)
- Initializes git repo and commits clean state

### 2. Apply Existing Patches
```bash
just apply-patches-sage-starbased
```
This applies all patches using `git apply`

### 3. Edit and Test in dist/
```bash
# Make edits directly in dist/sage-starbased/
cd dist/sage-starbased
# Edit files as needed
vim src/accounts/fleet.rs

# Test compilation
cargo check
```

### 4. Create New Patches
```bash
# From dist/sage-starbased with uncommitted changes:
cd dist/sage-starbased
git diff src/accounts/fleet.rs > ../../patches/sage-starbased-fleet.patch

# Or use the just command (from project root):
just create-patch-sage-starbased my-change
```

> **⚠️ Patch Naming Convention**
> When multiple patches need to be applied in a specific order, use a numbered prefix:
> - `sage-starbased-01-accounts.patch` (applied first)
> - `sage-starbased-02-instructions.patch` (applied second)
> - `sage-starbased-03-custom-deserialize.patch` (applied third)
>
> Patches are applied alphabetically, so the numbering ensures correct order.

> **⚠️ CRITICAL: Creating Sequential Patches**
> When creating a new patch that builds on existing patches, you MUST commit the applied patches first:
> ```bash
> # After applying patches 01 and 02
> cd dist/sage-starbased
> git add -A
> git commit -m "Apply existing patches"
>
> # NOW make your new changes
> # Edit files...
>
> # Create patch 03 (will only include new changes)
> cd ../..
> just create-patch-sage-starbased 03-my-new-feature
> ```
> If you don't commit first, the new patch will include ALL changes from previous patches!

### 5. Publish to Workspace
```bash
just publish-sage-starbased
```
This removes the .git directory and moves to carbon-decoders/

### Complete Pipeline
```bash
just all-sage-starbased
```
Runs: clean → build → apply-patches → publish

## Key Improvements

1. **Clean git history**: Each build starts with committed clean state
2. **Easy patch testing**: Edit directly in dist/ before publishing
3. **Simplified patch paths**: Patches use relative paths (no -p3 needed)

## Common Customizations

**Variable-length "remaining data" fields:**
Some accounts have variable-length data after fixed fields. Example: Fleet's `fleet_state`.

**Dynamic arrays not in IDL:**
Some accounts have dynamic lists that aren't in the IDL. Example: StarbasePlayer's `ship_escrows`.

**Custom deserialize implementations needed when:**
- Account has "remaining data" after defined fields
- Account has dynamic-length fields
- Account has complex nested structures
- Standard borsh deserialization doesn't handle the account layout

## Creating Sequential Patches: Complete Example

This example shows the full workflow for creating patch 03 that builds on patches 01 and 02:

```bash
# 1. Build clean decoder
just build-sage-starbased

# 2. Apply existing patches (01 and 02)
just apply-patches-sage-starbased

# 3. COMMIT the applied patches (critical step!)
cd dist/sage-starbased
git add -A
git commit -m "Apply patches 01 and 02"

# 4. Make your new changes for patch 03
# Edit the files you need to change
vim src/instructions/start_subwarp.rs
vim src/instructions/stop_subwarp.rs
# ... etc

# 5. Test your changes
cargo check

# 6. Create patch 03 (will only contain new changes)
cd ../..
just create-patch-sage-starbased 03-instructions-movement

# 7. Verify patch size is reasonable
# Should only show your new changes, not patches 01+02
cat patches/sage-starbased-03-instructions-movement.patch | wc -l

# 8. Test the full pipeline
just all-sage-starbased
```

## Comment Style Conventions

When creating patches for instruction account expansions, follow these commenting conventions for consistency:

**For composite account expansions:**
```rust
pub struct MyInstructionAccounts {
    // GameAndGameStateAndFleetAndOwnerMut expansion
    pub key: solana_pubkey::Pubkey,
    pub owning_profile: solana_pubkey::Pubkey,
    // ... other expanded accounts

    // Direct accounts
    pub fuel_tank: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    // ... other non-composite accounts
}
```

**In the arrange_accounts implementation:**
```rust
fn arrange_accounts(...) -> Option<Self::ArrangedAccounts> {
    let mut iter = accounts.iter();

    // GameAndGameStateAndFleetAndOwnerMut expansion
    let key = next_account(&mut iter)?;
    let owning_profile = next_account(&mut iter)?;
    // ... other expanded accounts

    // Direct accounts
    let fuel_tank = next_account(&mut iter)?;
    // ... other accounts
}
```

This commenting style:
- Clearly indicates which accounts come from composite expansions
- Shows where direct (non-composite) accounts begin
- Makes patches easier to review and understand
- Matches the style used in existing patches (e.g., mining instructions)

## Example: Fleet Account Patch

The Fleet account needs custom deserialization for `fleet_state` field (remaining data):

1. **Add field to struct:**
   ```rust
   pub struct Fleet {
       // ... existing fields ...
       pub bump: u8,
       pub fleet_state: FleetState,  // Add this
   }
   ```

2. **Define FleetState enum:**
   ```rust
   pub enum FleetState {
       StarbaseLoadingBay(StarbaseLoadingBay),
       Idle(Idle),
       MineAsteroid(MineAsteroid),
       MoveWarp(MoveWarp),
       MoveSubwarp(MoveSubwarp),
       Respawn(Respawn),
   }
   ```

3. **Implement custom BorshDeserialize:**
   ```rust
   impl borsh::de::BorshDeserialize for Fleet {
       fn deserialize_reader<R: borsh::maybestd::io::Read>(
           reader: &mut R,
       ) -> Result<Self, borsh::maybestd::io::Error> {
           // Deserialize fixed fields
           // Then deserialize fleet_state from remaining data
       }
   }
   ```

4. **Implement custom CarbonDeserialize:**
   Override the default to use the custom BorshDeserialize.

## Adding New Decoders

When adding patches for a new decoder:

1. Create a new patch file: `patches/[decoder-name]-[patch-type].patch`
   - Use numbered prefixes if order matters: `[decoder-name]-01-[description].patch`
2. Add new just commands in the justfile:
   - `generate-[decoder-name]`
   - `patch-[decoder-name]`
   - `apply-patches-[decoder-name]`
   - `publish-[decoder-name]`
3. Follow the same workflow as above

## Troubleshooting

**Patch fails to apply:**
- Regenerate the base decoder and recreate the patch
- Check if the generated decoder structure changed

**Compilation errors after patching:**
- Verify all necessary imports are included
- Check that types referenced exist in the types module
- Ensure the custom implementations match the expected interfaces

**Missing types:**
- The generated types module should have all component types
- Import them as needed: `use super::super::types::*;`
