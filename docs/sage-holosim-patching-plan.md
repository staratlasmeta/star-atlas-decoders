# Sage Holosim Decoder - Comprehensive Patching Plan

## Overview
This document outlines the complete patching strategy for expanding composite accounts in the sage-holosim-decoder. The decoder has 124 instruction files that need composite account expansions to match the actual Solana program structure. Sage-holosim includes additional combat features not present in sage-starbased.

## Progress Summary
- **Total instruction files**: 124
- **Completed patches**: 3 (covering 3 instruction files + 2 account files)
- **Files needing composite expansions**: ~92 remaining
- **Remaining patches**: ~14
- **Estimated total patches**: ~17

## Completed Patches (01-03)

### Patch 01: Disable Combat Log Events
- **Files**: 2 (combat_log_event.rs, combat_resolved_event.rs) + mod.rs updates
- **Purpose**: Comments out problematic combat log event instructions
- **Status**: ✅ Complete

### Patch 02: Accounts
- **Files**: 2 accounts + mod.rs (fleet.rs, starbase_player.rs)
- **Purpose**: Custom deserialize for accounts with remaining data fields
  - Fleet: `fleet_state` field (FleetState enum with remaining data)
  - StarbasePlayer: `ship_escrows` dynamic array field
- **Status**: ✅ Complete

### Patch 03: Scanning & Discovery
- **Files**: 3 (2 edited + 1 no changes needed)
  - scan_for_survey_data_units.rs
  - discover_sector.rs
  - fleet_state_handler.rs (no changes - only has direct `fleet` field)
- **Composite accounts**:
  - `game_accounts_fleet_and_owner` → GameAndGameStateAndFleetAndOwnerMut (6 accounts): key, owning_profile, owning_profile_faction, fleet, game_id, game_state
  - `data_running_xp_accounts` → PointsModificationAccounts (3 accounts): data_running_user_points_account, data_running_points_category, data_running_points_modifier_account
  - `council_rank_xp_accounts` → PointsModificationAccounts (3 accounts): council_rank_user_points_account, council_rank_points_category, council_rank_points_modifier_account
- **Complexity**: Medium-High (includes multiple XP account expansions with unique prefixes)
- **Priority**: 🔴 High - Core exploration mechanic
- **Status**: ✅ Complete (150 lines)

---

## Remaining Patches (04-17)

### Priority 1: Core Gameplay - Mining, Movement (High Priority)

#### Patch 04: Mining Operations
- **Files**: 2
  - start_mining_asteroid.rs
  - stop_mining_asteroid.rs
- **Composite accounts**:
  - `game_accounts_fleet_and_owner` → GameAndGameStateAndFleetAndOwnerMut (6 accounts)
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts): starbase, starbase_player
- **Complexity**: Medium
- **Priority**: 🔴 High - Core resource gathering mechanic
- **Status**: 🔲 Pending

#### Patch 05: Movement Instructions
- **Files**: 5
  - start_subwarp.rs
  - stop_subwarp.rs
  - warp_lane.rs
  - warp_to_coordinate.rs
  - mine_asteroid_to_respawn.rs
- **Composite accounts**:
  - `game_accounts_fleet_and_owner` → GameAndGameStateAndFleetAndOwnerMut (6 accounts)
- **Complexity**: Medium
- **Priority**: 🔴 High - Core movement mechanics
- **Status**: 🔲 Pending

---

### Priority 2: Frequently Used Operations (Medium-High Priority)

#### Patch 06: Starbase Operations
- **Files**: ~9
  - deposit_starbase_upkeep_resource.rs
  - start_starbase_upgrade.rs
  - complete_starbase_upgrade.rs
  - submit_starbase_upgrade_resource.rs
  - create_starbase_upgrade_resource_process.rs
  - close_upgrade_process.rs
  - sync_starbase_upgrade_ingredients.rs
  - repair_starbase.rs
  - create_cargo_pod.rs
- **Composite accounts**:
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts)
  - `game_accounts_and_profile` → GameAndGameStateAndProfile (5 accounts): key, profile, profile_faction, game_id, game_state
  - `loyalty_points_accounts` → PointsModificationAccounts (3 accounts) - for upkeep only
- **Complexity**: Medium-High
- **Priority**: 🟡 Medium-High - Frequently used starbase management
- **Status**: 🔲 Pending

#### Patch 07: Crafting Instructions
- **Files**: ~10
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
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts)
  - `game_accounts_and_profile` → GameAndGameStateAndProfile (5 accounts)
  - PointsModificationAccounts in close_crafting_process.rs
- **Complexity**: Medium-High
- **Priority**: 🟡 Medium-High - Core crafting system
- **Status**: 🔲 Pending

#### Patch 08: Fleet Management & State Transitions
- **Files**: ~15
  - create_fleet.rs
  - add_ship_to_fleet.rs
  - disband_fleet.rs
  - disbanded_fleet_to_escrow.rs
  - force_disband_fleet.rs
  - idle_to_loading_bay.rs
  - loading_bay_to_idle.rs
  - respawn_to_loading_bay.rs
  - idle_to_respawn.rs
  - load_fleet_crew.rs
  - unload_fleet_crew.rs
  - close_fleet_cargo_pod_token_account.rs
  - close_disbanded_fleet.rs
  - update_ship_in_fleet.rs
  - set_next_ship.rs
- **Composite accounts**:
  - `game_accounts_fleet_and_owner` → GameAndGameStateAndFleetAndOwnerMut (6 accounts)
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts)
  - `game_accounts_and_profile` → GameAndGameStateAndProfile (5 accounts)
  - `game_and_profile` → GameAndProfile (3 accounts): key, profile, game_id
  - `game_accounts` → GameAndGameState (2 accounts): game_id, game_state
- **Complexity**: High (complex state machine with many transitions)
- **Priority**: 🟡 Medium-High - Core fleet functionality
- **Status**: 🔲 Pending

#### Patch 09: Fleet Cargo Operations
- **Files**: 4
  - deposit_cargo_to_fleet.rs
  - withdraw_cargo_from_fleet.rs
  - transfer_cargo_within_fleet.rs
  - force_drop_fleet_cargo.rs
- **Composite accounts**:
  - `game_accounts_fleet_and_owner` → GameAndGameStateAndFleetAndOwnerMut (6 accounts)
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts) - deposit/withdraw only
- **Complexity**: Medium
- **Priority**: 🟡 Medium-High - Frequently used cargo operations
- **Status**: 🔲 Pending

---

### Priority 3: Combat & Player Management (Medium Priority)

#### Patch 10: Combat Operations (Holosim-Specific)
- **Files**: ~6
  - attack_fleet.rs
  - attack_starbase.rs
  - repair_docked_fleet.rs
  - repair_idle_fleet.rs
  - reload_fleet_ability_power.rs
  - retrieve_loot.rs
- **Composite accounts**:
  - `game_and_fleet_and_owner` → GameAndFleetAndOwner variant (4-6 accounts)
  - `game_accounts_fleet_and_owner` → GameAndGameStateAndFleetAndOwnerMut (6 accounts)
  - Multiple XP account expansions in combat (attacker/defender XP)
- **Complexity**: High (combat-specific logic with XP rewards)
- **Priority**: 🟠 Medium - Combat-specific features
- **Status**: 🔲 Pending
- **Note**: Unique to holosim, not in starbased

#### Patch 11: Ship & Crew Management
- **Files**: ~8
  - add_ship_escrow.rs
  - remove_ship_escrow.rs
  - update_ship_escrow.rs
  - remove_invalid_ship_escrow.rs
  - update_ship.rs
  - invalidate_ship.rs
  - add_crew_to_game.rs
  - remove_crew_from_game.rs
  - dev_add_crew_to_game.rs
  - mint_crew_to_game.rs
  - close_player_crew_record.rs
- **Composite accounts**:
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts)
  - `game_accounts_and_profile` → GameAndGameStateAndProfile (5 accounts)
  - `game_and_profile_and_faction` → GameAndProfileAndFaction (3 accounts)
  - `game_and_profile` → GameAndProfile (3 accounts)
- **Complexity**: Medium
- **Priority**: 🟠 Medium - Ship and crew operations
- **Status**: 🔲 Pending

#### Patch 12: Starbase Player & Cargo Operations
- **Files**: ~8
  - register_starbase_player.rs
  - sync_starbase_player.rs
  - deposit_cargo_to_game.rs
  - withdraw_cargo_from_game.rs
  - dev_deposit_cargo_to_game.rs
  - transfer_cargo_at_starbase.rs
  - close_starbase_cargo_token_account.rs
  - remove_cargo_pod.rs
- **Composite accounts**:
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts)
  - `game_accounts_and_profile` → GameAndGameStateAndProfile (5 accounts)
  - `game_and_profile_and_faction` → GameAndProfileAndFaction (3 accounts)
  - `game_accounts` → GameAndGameState (2 accounts)
- **Complexity**: Medium
- **Priority**: 🟠 Medium - Starbase player management
- **Status**: 🔲 Pending

---

### Priority 4: Admin & Configuration (Lower Priority)

#### Patch 13: Rental System
- **Files**: 3
  - add_rental.rs
  - change_rental.rs
  - invalidate_rental.rs
- **Composite accounts**:
  - `starbase_and_starbase_player` → StarbaseMutAndStarbasePlayer (2 accounts)
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low-Medium - Rental management
- **Status**: 🔲 Pending

#### Patch 14: Admin - Game & Combat Config
- **Files**: ~15
- **Functional areas**:
  - Game: init_game, init_game_state, update_game, update_game_state, activate_game_state, copy_game_state
  - Ship: register_ship
  - Crew: register_sage_crew_config
  - Player: register_sage_player_profile, register_sage_point_modifier
  - Progression: register_progression_config, update_progression_config, deregister_progression_config
  - Combat: register_combat_config, update_combat_config, deregister_combat_config (holosim-specific)
- **Composite accounts**:
  - `game_and_profile` → GameAndProfile (3 accounts)
  - `game_accounts` → GameAndGameState (2 accounts)
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low - Admin functions
- **Status**: 🔲 Pending

#### Patch 15: Admin - Starbase & Sector Config
- **Files**: ~10
- **Functional areas**:
  - Starbase: register_starbase, update_starbase, deregister_starbase
  - Sector: register_sector, add_connection, remove_connection
  - Celestial: register_planet, update_planet, register_star, update_star
- **Composite accounts**:
  - `game_and_profile` → GameAndProfile (3 accounts)
  - Various game account expansions
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low - Admin functions
- **Status**: 🔲 Pending

#### Patch 16: Admin - Resources & Mining Config
- **Files**: ~11
- **Functional areas**:
  - Resources: register_resource, update_resource, deregister_resource
  - Mine items: register_mine_item, update_mine_item, deregister_mine_item, drain_mine_item_bank
  - Survey units: register_survey_data_unit_tracker, update_survey_data_unit_tracker, deregister_survey_data_unit_tracker, drain_survey_data_units_bank
- **Composite accounts**:
  - `game_and_profile` → GameAndProfile (3 accounts)
  - `game_accounts` → GameAndGameState (2 accounts)
- **Complexity**: Low-Medium
- **Priority**: 🟢 Low - Admin functions
- **Status**: 🔲 Pending

#### Patch 17: Certificates & Events
- **Files**: ~8
  - create_certificate_mint.rs
  - mint_certificate.rs
  - redeem_certificate.rs
  - battle_log_event.rs
  - combat_initiated_event.rs
  - combat_loot_drop_event.rs
  - combat_participant_event.rs
  - starbase_combat_event.rs
- **Composite accounts**:
  - `game_and_profile` → GameAndProfile (3 accounts) - certificates
  - Various game account expansions for events
- **Complexity**: Low
- **Priority**: 🟢 Low - Certificates and events
- **Status**: 🔲 Pending
- **Note**: Some event instructions may not need expansions

---

## Common Composite Account Patterns

### Pattern 1: GameAndGameStateAndFleetAndOwnerMut
**Field name**: `game_accounts_fleet_and_owner`
**Expands to** (6 accounts):
- `key` - Game account/signer
- `owning_profile` - Profile owning the fleet
- `owning_profile_faction` - Faction of owning profile
- `fleet` - Fleet account
- `game_id` - Game ID
- `game_state` - Game state account

### Pattern 2: StarbaseMutAndStarbasePlayer
**Field name**: `starbase_and_starbase_player`
**Expands to** (2 accounts):
- `starbase` - Starbase account
- `starbase_player` - Starbase player account

### Pattern 3: GameAndGameStateAndProfile
**Field name**: `game_accounts_and_profile`
**Expands to** (5 accounts):
- `key` - Game account/signer
- `profile` - Player profile
- `profile_faction` - Profile faction
- `game_id` - Game ID
- `game_state` - Game state account

### Pattern 4: GameAndGameState
**Field name**: `game_accounts`
**Expands to** (2 accounts):
- `game_id` - Game ID
- `game_state` - Game state account

### Pattern 5: GameAndProfile
**Field name**: `game_and_profile`
**Expands to** (3 accounts):
- `key` - Game account/signer
- `profile` - Player profile
- `game_id` - Game ID

### Pattern 6: GameAndProfileAndFaction
**Field name**: `game_and_profile_and_faction`
**Expands to** (3 accounts):
- `key` - Game account/signer
- `profile` - Player profile
- `profile_faction` - Profile faction

### Pattern 7: GameAndFleetAndOwner (Combat Variant)
**Field name**: `game_and_fleet_and_owner`
**Expands to** (4-6 accounts):
- Similar to Pattern 1 but may exclude game_state in some contexts
- Used in combat operations

### Pattern 8: PointsModificationAccounts (XP/Loyalty)
**Expands to** (3 accounts per instance):
- `user_points_account` - User's points account
- `points_category` - Points category account
- `points_modifier_account` - Points modifier account

**Important**: When multiple PointsModificationAccounts exist in one instruction, use unique prefixes:
- First instance: `data_running_user_points_account`, `data_running_points_category`, `data_running_points_modifier_account`
- Second instance: `council_rank_user_points_account`, `council_rank_points_category`, `council_rank_points_modifier_account`

---

## Execution Strategy

### Recommended Order (Following User Preference)
1. ~~**Patch 01** - Disable Combat Log Events~~ ✅ Complete
2. ~~**Patch 02** - Accounts~~ ✅ Complete
3. ~~**Patch 03** - Scanning & Discovery~~ ✅ Complete
4. **Patch 04** - Mining Operations - **NEXT**
5. **Patch 05** - Movement Instructions
6. **Patch 06** - Starbase Operations
7. **Patch 07** - Crafting Instructions
8. **Patch 08** - Fleet Management
9. **Patch 09** - Fleet Cargo Operations
10. **Patch 10** - Combat Operations
11. **Patch 11** - Ship & Crew Management
12. **Patch 12** - Starbase Player & Cargo
13. **Patch 13** - Rental System
14. **Patch 14** - Game & Combat Config (admin)
15. **Patch 15** - Starbase & Sector Config (admin)
16. **Patch 16** - Resources & Mining Config (admin)
17. **Patch 17** - Certificates & Events

### Workflow Per Patch
1. `just build-sage-holosim` - Clean build using local IDL
2. `just apply-patches-sage-holosim` - Apply existing patches
3. `cd dist/sage-holosim && git add -A && git commit -m "Apply existing patches"` - Commit to isolate new changes
4. Edit instruction files with composite account expansions
5. `cargo check -p sage-holosim-decoder` - Test compilation
6. `just create-patch-sage-holosim XX-descriptive-name` - Create patch
7. `just publish-sage-holosim` - Publish to workspace

---

## Key Differences from Sage-Starbased

1. **Total Files**: 124 files (vs 106 in starbased) - 18 more files
2. **Combat Features**: Holosim includes combat operations not in starbased:
   - attack_fleet.rs, attack_starbase.rs
   - repair operations
   - Combat events and logging
   - Loot retrieval
3. **Composite Naming**: Slightly different conventions:
   - `game_accounts_fleet_and_owner` vs `game_accounts_fleet_and_owner_mut`
   - `starbase_and_starbase_player` vs `starbase_mut_and_starbase_player`
4. **IDL Source**: Local IDL file vs mainnet fetch
5. **Starting Point**: Already has 2 patches vs starbased having 11

---

## Files Not Requiring Patches
These files appear to use only direct accounts (no composite expansions needed):
- combat_log_event.rs (disabled in Patch 01)
- combat_resolved_event.rs (disabled in Patch 01)
- init_game.rs (simple direct accounts)
- Some event instructions may not need expansions

**Total**: ~30 files (events, disabled instructions, simple admin)

---

## Notes
- All patches follow the established comment pattern
- Comment format: `// CompositeAccountName expansion` followed by individual accounts
- Account order must match the Solana program's account ordering exactly
- XP account expansions (PointsModificationAccounts) follow the same 3-field pattern
- Combat operations include unique XP patterns (attacker/defender)
- Multiple instances of same composite type need unique field name prefixes
- Inline expansion pattern: composite accounts expanded at their original position, not grouped at top
