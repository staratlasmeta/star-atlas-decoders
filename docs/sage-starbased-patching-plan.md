# Sage Starbased Decoder - Comprehensive Patching Plan

## Overview
This document outlines the complete patching strategy for expanding composite accounts in the sage-starbased-decoder. The decoder has 106 instruction files that need composite account expansions to match the actual Solana program structure.

## Progress Summary
- **Total instruction files**: 106
- **Completed patches**: 11 (covering 65 files)
- **Remaining patches**: 5 (covering 41 files)
- **Estimated total patches**: 16

## Completed Patches (01-11)

### Patch 01: Accounts
- **Files**: 1 (fleet.rs)
- **Purpose**: Custom deserialize for Fleet account with remaining data field
- **Status**: ✅ Complete

### Patch 02: Mining Instructions
- **Files**: 2
  - start_mining_asteroid.rs
  - stop_mining_asteroid.rs
- **Composite accounts**: `GameAndGameStateAndFleetAndOwnerMut`, `StarbaseMutAndStarbasePlayer`
- **Status**: ✅ Complete

### Patch 03: Movement Instructions
- **Files**: 4
  - start_subwarp.rs
  - stop_subwarp.rs
  - warp_lane.rs
  - warp_to_coordinate.rs
- **Composite accounts**: `GameAndGameStateAndFleetAndOwnerMut`
- **Status**: ✅ Complete

### Patch 04: Starbase Operations
- **Files**: 7
  - deposit_starbase_upkeep_resource.rs
  - start_starbase_upgrade.rs
  - complete_starbase_upgrade.rs
  - submit_starbase_upgrade_resource.rs
  - create_starbase_upgrade_resource_process.rs
  - close_upgrade_process.rs
  - sync_starbase_upgrade_ingredients.rs
- **Composite accounts**:
  - `StarbaseMutAndStarbasePlayer` → starbase, starbase_player
  - `GameAndGameStateAndProfile` → key, profile, profile_faction, game_id, game_state
  - `PointsModificationAccounts` → user_points_account, points_category, points_modifier_account
- **Status**: ✅ Complete

### Patch 05: Crafting Instructions
- **Files**: 10
  - deposit_crafting_ingredient.rs
  - withdraw_crafting_ingredient.rs
  - start_crafting_process.rs
  - stop_crafting_process.rs
  - create_crafting_process.rs
  - cancel_crafting_process.rs
  - close_crafting_process.rs
  - claim_crafting_outputs.rs
  - burn_crafting_consumables.rs
  - claim_crafting_non_consumables.rs
- **Composite accounts**:
  - `StarbaseMutAndStarbasePlayer` → starbase, starbase_player
  - `GameAndGameStateAndProfile` → key, profile, profile_faction, game_id, game_state (7 files)
  - `GameAndGameState` → game_id, game_state (3 claim/burn files)
  - `PointsModificationAccounts` (in close_crafting_process.rs)
- **Status**: ✅ Complete

### Patch 06: Fleet Operations & State Transitions
- **Files**: 15
  - create_fleet.rs
  - add_ship_to_fleet.rs
  - disband_fleet.rs
  - disbanded_fleet_to_escrow.rs
  - force_disband_fleet.rs
  - idle_to_loading_bay.rs
  - loading_bay_to_idle.rs
  - respawn_to_loading_bay.rs
  - idle_to_respawn.rs
  - mine_asteroid_to_respawn.rs
  - load_fleet_crew.rs
  - unload_fleet_crew.rs
  - close_fleet_cargo_pod_token_account.rs
  - update_ship_in_fleet.rs
- **Composite accounts**:
  - `GameAndGameStateAndFleetAndOwnerMut` → key, owning_profile, owning_profile_faction, fleet, game_id, game_state
  - `FleetAndOwner` → key, owning_profile, owning_profile_faction, fleet
  - `GameAndProfile` → key, profile, game_id
  - `StarbaseMutAndStarbasePlayer` → starbase, starbase_player
  - `GameAndGameState` → game_id, game_state
- **Complexity**: Medium-High (complex state machine)
- **Priority**: 🔴 High - Core fleet functionality
- **Status**: ✅ Complete (805 lines)

### Patch 07: Fleet Cargo Operations
- **Files**: 4
  - deposit_cargo_to_fleet.rs
  - withdraw_cargo_from_fleet.rs
  - transfer_cargo_within_fleet.rs
  - set_next_ship.rs
- **Composite accounts**:
  - `GameAndGameStateAndFleetAndOwnerMut` → key, owning_profile, owning_profile_faction, fleet, game_id, game_state
  - `StarbaseAndStarbasePlayer` → starbase, starbase_player (deposit & withdraw only)
  - `ActiveOrInactiveGameAndProfile` → key, profile, game_id (set_next_ship only)
- **Complexity**: Medium
- **Priority**: 🔴 High - Frequently used operations
- **Status**: ✅ Complete (226 lines)

### Patch 08: Starbase Cargo & Player Operations
- **Files**: 9
  - create_cargo_pod.rs
  - remove_cargo_pod.rs
  - close_starbase_cargo_token_account.rs
  - deposit_cargo_to_game.rs
  - withdraw_cargo_from_game.rs
  - dev_deposit_cargo_to_game.rs
  - transfer_cargo_at_starbase.rs
  - register_starbase_player.rs
  - sync_starbase_player.rs
- **Composite accounts**:
  - `StarbaseAndStarbasePlayer` → starbase, starbase_player (7 files)
  - `GameAndGameStateAndProfile` → key, profile, profile_faction, game_id, game_state (6 files)
  - `GameAndProfileAndFaction` → key, profile, profile_faction, game_id (dev_deposit_cargo_to_game only)
  - `GameAndGameState` → game_id, game_state (register/sync starbase player)
- **Complexity**: Medium
- **Priority**: 🟡 Medium - Frequently used starbase operations
- **Status**: ✅ Complete (527 lines)

### Patch 09: Crew & Player Management
- **Files**: 5
  - add_crew_to_game.rs
  - remove_crew_from_game.rs
  - dev_add_crew_to_game.rs
  - mint_crew_to_game.rs
  - close_player_crew_record.rs
- **Composite accounts**:
  - `StarbaseAndStarbasePlayerMut` → starbase, starbase_player
  - `GameAndProfileAndFaction` → key, profile, profile_faction, game_id
  - `GameAndProfile` → key, profile, game_id (dev_add_crew_to_game only)
- **Complexity**: Medium
- **Priority**: 🟡 Medium - Crew management operations
- **Status**: ✅ Complete (242 lines)

### Patch 10: Ship Escrow & Management
- **Files**: 6
  - add_ship_escrow.rs
  - remove_ship_escrow.rs
  - update_ship_escrow.rs
  - remove_invalid_ship_escrow.rs
  - update_ship.rs
  - invalidate_ship.rs
- **Composite accounts**:
  - `StarbaseAndStarbasePlayerMut` → starbase, starbase_player
  - `GameAndGameStateAndProfile` → key, profile, profile_faction, game_id, game_state
  - `GameAndGameState` → game_id, game_state (update_ship_escrow only)
  - `ActiveOrInactiveGameAndProfile` → key, profile, game_id (update_ship & invalidate_ship)
- **Complexity**: Medium
- **Priority**: 🟡 Medium - Ship escrow management operations
- **Status**: ✅ Complete (323 lines)

### Patch 11: Scanning & Discovery
- **Files**: 3
  - scan_for_survey_data_units.rs
  - discover_sector.rs
  - fleet_state_handler.rs (no changes needed)
- **Composite accounts**:
  - `GameAndGameStateAndFleetAndOwnerMut` → key, owning_profile, owning_profile_faction, fleet, game_id, game_state
  - `PointsModificationAccounts` (2 instances in scan_for_survey_data_units.rs with unique prefixes)
    - data_running: data_running_user_points_account, data_running_points_category, data_running_points_modifier_account
    - council_rank: council_rank_user_points_account, council_rank_points_category, council_rank_points_modifier_account
- **Complexity**: Medium-High (includes multiple XP account expansions with prefixes)
- **Priority**: 🔴 High - Core gameplay mechanic
- **Status**: ✅ Complete (150 lines)

---

## Remaining Patches (12-16)

### Priority 1: Frequently Used Operations (Medium Priority)

#### Patch 12: Rental System
- **Files**: ~3
  - add_rental.rs
  - change_rental.rs
  - invalidate_rental.rs
- **Composite accounts**:
  - `StarbaseMutAndStarbasePlayer`
- **Complexity**: Low-Medium
- **Priority**: 🟡 Medium
- **Status**: 🔲 Pending

---

### Priority 3: Admin & Configuration (Lower Priority)

#### Patch 13: Admin - Game Registration & Config
- **Files**: ~13
- **Functional areas**:
  - Game: init_game, init_game_state, update_game, update_game_state, activate_game_state, copy_game_state
  - Ship: register_ship
  - Crew: register_sage_crew_config
  - Player: register_sage_player_profile, register_sage_point_modifier
  - Progression: register_progression_config, update_progression_config, deregister_progression_config
- **Composite accounts**:
  - `GameAndProfile`
  - `GameAccounts`
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low - Admin functions, less frequently called
- **Status**: 🔲 Pending

#### Patch 14: Admin - Starbase & Sector Config
- **Files**: ~10
- **Functional areas**:
  - Starbase: register_starbase, update_starbase, deregister_starbase
  - Sector: register_sector, add_connection, remove_connection
  - Celestial: register_planet, update_planet, register_star, update_star
- **Composite accounts**:
  - `GameAndProfile`
  - `GameStateAndProfile`
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low - Admin functions
- **Status**: 🔲 Pending

#### Patch 15: Admin - Resources & Mining Config
- **Files**: ~11
- **Functional areas**:
  - Resources: register_resource, update_resource, deregister_resource
  - Mine items: register_mine_item, update_mine_item, deregister_mine_item, drain_mine_item_bank
  - Survey units: register_survey_data_unit_tracker, update_survey_data_unit_tracker, deregister_survey_data_unit_tracker, drain_survey_data_units_bank
- **Composite accounts**:
  - `GameAndProfile`
  - `GameAccounts`
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low - Admin functions
- **Status**: 🔲 Pending

#### Patch 16: Certificates & Miscellaneous
- **Files**: ~3
  - create_certificate_mint.rs
  - mint_certificate.rs
  - redeem_certificate.rs
- **Composite accounts**:
  - `GameAndProfile`
- **Complexity**: Low
- **Priority**: 🟢 Low
- **Status**: 🔲 Pending

---

## Common Composite Account Patterns

### Pattern 1: GameAndGameStateAndFleetAndOwnerMut
**Expands to** (5 accounts):
- `fleet_and_owner` → fleet, fleet_owner_profile
- `game_id`
- `game_state`
- `game_account` (sometimes called `key`)

### Pattern 2: StarbaseMutAndStarbasePlayer
**Expands to** (2 accounts):
- `starbase`
- `starbase_player`

### Pattern 3: GameAndGameStateAndProfile
**Expands to** (5 accounts):
- `key`
- `profile`
- `profile_faction`
- `game_id`
- `game_state`

### Pattern 4: GameAndGameState
**Expands to** (2 accounts):
- `game_id`
- `game_state`

### Pattern 5: PointsModificationAccounts
**Expands to** (3 accounts per instance):
- `user_points_account`
- `points_category`
- `points_modifier_account`

---

## Execution Strategy

### Recommended Order
1. ~~**Patch 06** - Fleet Operations (core gameplay)~~ ✅ Complete
2. ~~**Patch 07** - Fleet Cargo (frequently used)~~ ✅ Complete
3. ~~**Patch 08** - Starbase Cargo (frequently used)~~ ✅ Complete
4. ~~**Patch 09** - Crew Management~~ ✅ Complete
5. ~~**Patch 10** - Ship Escrow~~ ✅ Complete
6. ~~**Patch 11** - Scanning & Discovery (core gameplay with XP)~~ ✅ Complete
7. **Patch 12** - Rental System - **NEXT**
8. **Patch 13** - Game Config (admin)
9. **Patch 14** - Starbase Config (admin)
10. **Patch 15** - Resources Config (admin)
11. **Patch 16** - Certificates

### Workflow Per Patch
1. `just build-sage-starbased` - Clean build
2. `just apply-patches-sage-starbased` - Apply existing patches
3. `cd dist/sage-starbased && git add -A && git commit -m "Apply existing patches"` - Commit to isolate new changes
4. Edit instruction files with composite account expansions
5. `cargo check` - Test compilation
6. `just create-patch-sage-starbased XX-descriptive-name` - Create patch
7. `just publish-sage-starbased` - Publish to workspace (optional)

---

## Files Not Requiring Patches
These files appear to use only direct accounts (no composite expansions needed):
- close_disbanded_fleet.rs
- close_player_crew_record.rs
- force_drop_fleet_cargo.rs
- init_game.rs
- register_sage_player_profile.rs

**Total**: ~5 files

---

## Notes
- All patches follow the established comment pattern from patches 02-05
- Comment format: `// CompositeAccountName expansion` followed by individual accounts
- Direct accounts are marked with `// Direct accounts` comment
- Account order must match the Solana program's account ordering exactly
- XP account expansions (PointsModificationAccounts) follow the same 3-field pattern