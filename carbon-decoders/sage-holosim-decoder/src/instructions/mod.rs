use super::SageDecoder;
pub mod activate_game_state;
pub mod add_connection;
pub mod add_crew_to_game;
pub mod add_rental;
pub mod add_ship_escrow;
pub mod add_ship_to_fleet;
pub mod attack_fleet;
pub mod attack_starbase;
pub mod battle_log_event;
pub mod burn_crafting_consumables;
pub mod cancel_crafting_process;
pub mod change_rental;
pub mod claim_crafting_non_consumables;
pub mod claim_crafting_outputs;
pub mod close_crafting_process;
pub mod close_disbanded_fleet;
pub mod close_fleet_cargo_pod_token_account;
pub mod close_player_crew_record;
pub mod close_starbase_cargo_token_account;
pub mod close_upgrade_process;
pub mod combat_initiated_event;
// pub mod combat_log_event;
pub mod combat_loot_drop_event;
pub mod combat_participant_event;
pub mod combat_resolved_event;
pub mod complete_starbase_upgrade;
pub mod copy_game_state;
pub mod create_cargo_pod;
pub mod create_certificate_mint;
pub mod create_crafting_process;
pub mod create_fleet;
pub mod create_starbase_upgrade_resource_process;
pub mod deposit_cargo_to_fleet;
pub mod deposit_cargo_to_game;
pub mod deposit_crafting_ingredient;
pub mod deposit_starbase_upkeep_resource;
pub mod deregister_combat_config;
pub mod deregister_mine_item;
pub mod deregister_progression_config;
pub mod deregister_resource;
pub mod deregister_starbase;
pub mod deregister_survey_data_unit_tracker;
pub mod dev_add_crew_to_game;
pub mod dev_deposit_cargo_to_game;
pub mod disband_fleet;
pub mod disbanded_fleet_to_escrow;
pub mod discover_sector;
pub mod drain_mine_item_bank;
pub mod drain_survey_data_units_bank;
pub mod fleet_state_handler;
pub mod force_disband_fleet;
pub mod force_drop_fleet_cargo;
pub mod idle_to_loading_bay;
pub mod idle_to_respawn;
pub mod init_game;
pub mod init_game_state;
pub mod invalidate_rental;
pub mod invalidate_ship;
pub mod load_fleet_crew;
pub mod loading_bay_to_idle;
pub mod mine_asteroid_to_respawn;
pub mod mint_certificate;
pub mod mint_crew_to_game;
pub mod redeem_certificate;
pub mod register_combat_config;
pub mod register_mine_item;
pub mod register_planet;
pub mod register_progression_config;
pub mod register_resource;
pub mod register_sage_crew_config;
pub mod register_sage_player_profile;
pub mod register_sage_point_modifier;
pub mod register_sector;
pub mod register_ship;
pub mod register_star;
pub mod register_starbase;
pub mod register_starbase_player;
pub mod register_survey_data_unit_tracker;
pub mod reload_fleet_ability_power;
pub mod remove_cargo_pod;
pub mod remove_connection;
pub mod remove_crew_from_game;
pub mod remove_invalid_ship_escrow;
pub mod remove_ship_escrow;
pub mod repair_docked_fleet;
pub mod repair_idle_fleet;
pub mod repair_starbase;
pub mod respawn_to_loading_bay;
pub mod retrieve_loot;
pub mod scan_for_survey_data_units;
pub mod set_next_ship;
pub mod starbase_combat_event;
pub mod start_crafting_process;
pub mod start_mining_asteroid;
pub mod start_starbase_upgrade;
pub mod start_subwarp;
pub mod stop_crafting_process;
pub mod stop_mining_asteroid;
pub mod stop_subwarp;
pub mod submit_starbase_upgrade_resource;
pub mod sync_starbase_player;
pub mod sync_starbase_upgrade_ingredients;
pub mod transfer_cargo_at_starbase;
pub mod transfer_cargo_within_fleet;
pub mod unload_fleet_crew;
pub mod update_combat_config;
pub mod update_game;
pub mod update_game_state;
pub mod update_mine_item;
pub mod update_planet;
pub mod update_progression_config;
pub mod update_resource;
pub mod update_ship;
pub mod update_ship_escrow;
pub mod update_ship_in_fleet;
pub mod update_star;
pub mod update_starbase;
pub mod update_survey_data_unit_tracker;
pub mod warp_lane;
pub mod warp_to_coordinate;
pub mod withdraw_cargo_from_fleet;
pub mod withdraw_cargo_from_game;
pub mod withdraw_crafting_ingredient;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum SageInstruction {
    ActivateGameState(activate_game_state::ActivateGameState),
    AddConnection(add_connection::AddConnection),
    AddCrewToGame(add_crew_to_game::AddCrewToGame),
    AddRental(add_rental::AddRental),
    AddShipEscrow(add_ship_escrow::AddShipEscrow),
    AddShipToFleet(add_ship_to_fleet::AddShipToFleet),
    AttackFleet(attack_fleet::AttackFleet),
    AttackStarbase(attack_starbase::AttackStarbase),
    BurnCraftingConsumables(burn_crafting_consumables::BurnCraftingConsumables),
    CancelCraftingProcess(cancel_crafting_process::CancelCraftingProcess),
    ChangeRental(change_rental::ChangeRental),
    ClaimCraftingNonConsumables(claim_crafting_non_consumables::ClaimCraftingNonConsumables),
    ClaimCraftingOutputs(claim_crafting_outputs::ClaimCraftingOutputs),
    CloseCraftingProcess(close_crafting_process::CloseCraftingProcess),
    CloseDisbandedFleet(close_disbanded_fleet::CloseDisbandedFleet),
    CloseFleetCargoPodTokenAccount(
        close_fleet_cargo_pod_token_account::CloseFleetCargoPodTokenAccount,
    ),
    ClosePlayerCrewRecord(close_player_crew_record::ClosePlayerCrewRecord),
    CloseStarbaseCargoTokenAccount(
        close_starbase_cargo_token_account::CloseStarbaseCargoTokenAccount,
    ),
    CloseUpgradeProcess(close_upgrade_process::CloseUpgradeProcess),
    CompleteStarbaseUpgrade(complete_starbase_upgrade::CompleteStarbaseUpgrade),
    CopyGameState(copy_game_state::CopyGameState),
    CreateCargoPod(create_cargo_pod::CreateCargoPod),
    CreateCertificateMint(create_certificate_mint::CreateCertificateMint),
    CreateCraftingProcess(create_crafting_process::CreateCraftingProcess),
    CreateFleet(create_fleet::CreateFleet),
    CreateStarbaseUpgradeResourceProcess(
        create_starbase_upgrade_resource_process::CreateStarbaseUpgradeResourceProcess,
    ),
    DepositCargoToFleet(deposit_cargo_to_fleet::DepositCargoToFleet),
    DepositCargoToGame(deposit_cargo_to_game::DepositCargoToGame),
    DepositCraftingIngredient(deposit_crafting_ingredient::DepositCraftingIngredient),
    DepositStarbaseUpkeepResource(deposit_starbase_upkeep_resource::DepositStarbaseUpkeepResource),
    DeregisterCombatConfig(deregister_combat_config::DeregisterCombatConfig),
    DeregisterMineItem(deregister_mine_item::DeregisterMineItem),
    DeregisterProgressionConfig(deregister_progression_config::DeregisterProgressionConfig),
    DeregisterResource(deregister_resource::DeregisterResource),
    DeregisterStarbase(deregister_starbase::DeregisterStarbase),
    DeregisterSurveyDataUnitTracker(
        deregister_survey_data_unit_tracker::DeregisterSurveyDataUnitTracker,
    ),
    DevAddCrewToGame(dev_add_crew_to_game::DevAddCrewToGame),
    DevDepositCargoToGame(dev_deposit_cargo_to_game::DevDepositCargoToGame),
    DisbandFleet(disband_fleet::DisbandFleet),
    DisbandedFleetToEscrow(disbanded_fleet_to_escrow::DisbandedFleetToEscrow),
    DiscoverSector(discover_sector::DiscoverSector),
    DrainMineItemBank(drain_mine_item_bank::DrainMineItemBank),
    DrainSurveyDataUnitsBank(drain_survey_data_units_bank::DrainSurveyDataUnitsBank),
    FleetStateHandler(fleet_state_handler::FleetStateHandler),
    ForceDisbandFleet(force_disband_fleet::ForceDisbandFleet),
    ForceDropFleetCargo(force_drop_fleet_cargo::ForceDropFleetCargo),
    IdleToLoadingBay(idle_to_loading_bay::IdleToLoadingBay),
    IdleToRespawn(idle_to_respawn::IdleToRespawn),
    InitGame(init_game::InitGame),
    InitGameState(init_game_state::InitGameState),
    InvalidateRental(invalidate_rental::InvalidateRental),
    InvalidateShip(invalidate_ship::InvalidateShip),
    LoadFleetCrew(load_fleet_crew::LoadFleetCrew),
    LoadingBayToIdle(loading_bay_to_idle::LoadingBayToIdle),
    MineAsteroidToRespawn(mine_asteroid_to_respawn::MineAsteroidToRespawn),
    MintCertificate(mint_certificate::MintCertificate),
    MintCrewToGame(mint_crew_to_game::MintCrewToGame),
    RedeemCertificate(redeem_certificate::RedeemCertificate),
    RegisterCombatConfig(register_combat_config::RegisterCombatConfig),
    RegisterMineItem(register_mine_item::RegisterMineItem),
    RegisterPlanet(register_planet::RegisterPlanet),
    RegisterProgressionConfig(register_progression_config::RegisterProgressionConfig),
    RegisterResource(register_resource::RegisterResource),
    RegisterSageCrewConfig(register_sage_crew_config::RegisterSageCrewConfig),
    RegisterSagePlayerProfile(register_sage_player_profile::RegisterSagePlayerProfile),
    RegisterSagePointModifier(register_sage_point_modifier::RegisterSagePointModifier),
    RegisterSector(register_sector::RegisterSector),
    RegisterShip(register_ship::RegisterShip),
    RegisterStar(register_star::RegisterStar),
    RegisterStarbase(register_starbase::RegisterStarbase),
    RegisterStarbasePlayer(register_starbase_player::RegisterStarbasePlayer),
    RegisterSurveyDataUnitTracker(register_survey_data_unit_tracker::RegisterSurveyDataUnitTracker),
    ReloadFleetAbilityPower(reload_fleet_ability_power::ReloadFleetAbilityPower),
    RemoveCargoPod(remove_cargo_pod::RemoveCargoPod),
    RemoveConnection(remove_connection::RemoveConnection),
    RemoveCrewFromGame(remove_crew_from_game::RemoveCrewFromGame),
    RemoveInvalidShipEscrow(remove_invalid_ship_escrow::RemoveInvalidShipEscrow),
    RemoveShipEscrow(remove_ship_escrow::RemoveShipEscrow),
    RepairDockedFleet(repair_docked_fleet::RepairDockedFleet),
    RepairIdleFleet(repair_idle_fleet::RepairIdleFleet),
    RepairStarbase(repair_starbase::RepairStarbase),
    RespawnToLoadingBay(respawn_to_loading_bay::RespawnToLoadingBay),
    RetrieveLoot(retrieve_loot::RetrieveLoot),
    ScanForSurveyDataUnits(scan_for_survey_data_units::ScanForSurveyDataUnits),
    SetNextShip(set_next_ship::SetNextShip),
    StartCraftingProcess(start_crafting_process::StartCraftingProcess),
    StartMiningAsteroid(start_mining_asteroid::StartMiningAsteroid),
    StartStarbaseUpgrade(start_starbase_upgrade::StartStarbaseUpgrade),
    StartSubwarp(start_subwarp::StartSubwarp),
    StopCraftingProcess(stop_crafting_process::StopCraftingProcess),
    StopMiningAsteroid(stop_mining_asteroid::StopMiningAsteroid),
    StopSubwarp(stop_subwarp::StopSubwarp),
    SubmitStarbaseUpgradeResource(submit_starbase_upgrade_resource::SubmitStarbaseUpgradeResource),
    SyncStarbasePlayer(sync_starbase_player::SyncStarbasePlayer),
    SyncStarbaseUpgradeIngredients(
        sync_starbase_upgrade_ingredients::SyncStarbaseUpgradeIngredients,
    ),
    TransferCargoAtStarbase(transfer_cargo_at_starbase::TransferCargoAtStarbase),
    TransferCargoWithinFleet(transfer_cargo_within_fleet::TransferCargoWithinFleet),
    UnloadFleetCrew(unload_fleet_crew::UnloadFleetCrew),
    UpdateCombatConfig(update_combat_config::UpdateCombatConfig),
    UpdateGame(update_game::UpdateGame),
    UpdateGameState(update_game_state::UpdateGameState),
    UpdateMineItem(update_mine_item::UpdateMineItem),
    UpdatePlanet(update_planet::UpdatePlanet),
    UpdateProgressionConfig(update_progression_config::UpdateProgressionConfig),
    UpdateResource(update_resource::UpdateResource),
    UpdateShip(update_ship::UpdateShip),
    UpdateShipEscrow(update_ship_escrow::UpdateShipEscrow),
    UpdateShipInFleet(update_ship_in_fleet::UpdateShipInFleet),
    UpdateStar(update_star::UpdateStar),
    UpdateStarbase(update_starbase::UpdateStarbase),
    UpdateSurveyDataUnitTracker(update_survey_data_unit_tracker::UpdateSurveyDataUnitTracker),
    WarpLane(warp_lane::WarpLane),
    WarpToCoordinate(warp_to_coordinate::WarpToCoordinate),
    WithdrawCargoFromFleet(withdraw_cargo_from_fleet::WithdrawCargoFromFleet),
    WithdrawCargoFromGame(withdraw_cargo_from_game::WithdrawCargoFromGame),
    WithdrawCraftingIngredient(withdraw_crafting_ingredient::WithdrawCraftingIngredient),
    BattleLogEvent(battle_log_event::BattleLogEvent),
    CombatInitiatedEvent(combat_initiated_event::CombatInitiatedEvent),
    // CombatLogEvent(combat_log_event::CombatLogEvent),
    CombatLootDropEvent(combat_loot_drop_event::CombatLootDropEvent),
    CombatParticipantEvent(combat_participant_event::CombatParticipantEvent),
    // CombatResolvedEvent(combat_resolved_event::CombatResolvedEvent),
    StarbaseCombatEvent(starbase_combat_event::StarbaseCombatEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for SageDecoder {
    type InstructionType = SageInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            SageInstruction::ActivateGameState => activate_game_state::ActivateGameState,
            SageInstruction::AddConnection => add_connection::AddConnection,
            SageInstruction::AddCrewToGame => add_crew_to_game::AddCrewToGame,
            SageInstruction::AddRental => add_rental::AddRental,
            SageInstruction::AddShipEscrow => add_ship_escrow::AddShipEscrow,
            SageInstruction::AddShipToFleet => add_ship_to_fleet::AddShipToFleet,
            SageInstruction::AttackFleet => attack_fleet::AttackFleet,
            SageInstruction::AttackStarbase => attack_starbase::AttackStarbase,
            SageInstruction::BurnCraftingConsumables => burn_crafting_consumables::BurnCraftingConsumables,
            SageInstruction::CancelCraftingProcess => cancel_crafting_process::CancelCraftingProcess,
            SageInstruction::ChangeRental => change_rental::ChangeRental,
            SageInstruction::ClaimCraftingNonConsumables => claim_crafting_non_consumables::ClaimCraftingNonConsumables,
            SageInstruction::ClaimCraftingOutputs => claim_crafting_outputs::ClaimCraftingOutputs,
            SageInstruction::CloseCraftingProcess => close_crafting_process::CloseCraftingProcess,
            SageInstruction::CloseDisbandedFleet => close_disbanded_fleet::CloseDisbandedFleet,
            SageInstruction::CloseFleetCargoPodTokenAccount => close_fleet_cargo_pod_token_account::CloseFleetCargoPodTokenAccount,
            SageInstruction::ClosePlayerCrewRecord => close_player_crew_record::ClosePlayerCrewRecord,
            SageInstruction::CloseStarbaseCargoTokenAccount => close_starbase_cargo_token_account::CloseStarbaseCargoTokenAccount,
            SageInstruction::CloseUpgradeProcess => close_upgrade_process::CloseUpgradeProcess,
            SageInstruction::CompleteStarbaseUpgrade => complete_starbase_upgrade::CompleteStarbaseUpgrade,
            SageInstruction::CopyGameState => copy_game_state::CopyGameState,
            SageInstruction::CreateCargoPod => create_cargo_pod::CreateCargoPod,
            SageInstruction::CreateCertificateMint => create_certificate_mint::CreateCertificateMint,
            SageInstruction::CreateCraftingProcess => create_crafting_process::CreateCraftingProcess,
            SageInstruction::CreateFleet => create_fleet::CreateFleet,
            SageInstruction::CreateStarbaseUpgradeResourceProcess => create_starbase_upgrade_resource_process::CreateStarbaseUpgradeResourceProcess,
            SageInstruction::DepositCargoToFleet => deposit_cargo_to_fleet::DepositCargoToFleet,
            SageInstruction::DepositCargoToGame => deposit_cargo_to_game::DepositCargoToGame,
            SageInstruction::DepositCraftingIngredient => deposit_crafting_ingredient::DepositCraftingIngredient,
            SageInstruction::DepositStarbaseUpkeepResource => deposit_starbase_upkeep_resource::DepositStarbaseUpkeepResource,
            SageInstruction::DeregisterCombatConfig => deregister_combat_config::DeregisterCombatConfig,
            SageInstruction::DeregisterMineItem => deregister_mine_item::DeregisterMineItem,
            SageInstruction::DeregisterProgressionConfig => deregister_progression_config::DeregisterProgressionConfig,
            SageInstruction::DeregisterResource => deregister_resource::DeregisterResource,
            SageInstruction::DeregisterStarbase => deregister_starbase::DeregisterStarbase,
            SageInstruction::DeregisterSurveyDataUnitTracker => deregister_survey_data_unit_tracker::DeregisterSurveyDataUnitTracker,
            SageInstruction::DevAddCrewToGame => dev_add_crew_to_game::DevAddCrewToGame,
            SageInstruction::DevDepositCargoToGame => dev_deposit_cargo_to_game::DevDepositCargoToGame,
            SageInstruction::DisbandFleet => disband_fleet::DisbandFleet,
            SageInstruction::DisbandedFleetToEscrow => disbanded_fleet_to_escrow::DisbandedFleetToEscrow,
            SageInstruction::DiscoverSector => discover_sector::DiscoverSector,
            SageInstruction::DrainMineItemBank => drain_mine_item_bank::DrainMineItemBank,
            SageInstruction::DrainSurveyDataUnitsBank => drain_survey_data_units_bank::DrainSurveyDataUnitsBank,
            SageInstruction::FleetStateHandler => fleet_state_handler::FleetStateHandler,
            SageInstruction::ForceDisbandFleet => force_disband_fleet::ForceDisbandFleet,
            SageInstruction::ForceDropFleetCargo => force_drop_fleet_cargo::ForceDropFleetCargo,
            SageInstruction::IdleToLoadingBay => idle_to_loading_bay::IdleToLoadingBay,
            SageInstruction::IdleToRespawn => idle_to_respawn::IdleToRespawn,
            SageInstruction::InitGame => init_game::InitGame,
            SageInstruction::InitGameState => init_game_state::InitGameState,
            SageInstruction::InvalidateRental => invalidate_rental::InvalidateRental,
            SageInstruction::InvalidateShip => invalidate_ship::InvalidateShip,
            SageInstruction::LoadFleetCrew => load_fleet_crew::LoadFleetCrew,
            SageInstruction::LoadingBayToIdle => loading_bay_to_idle::LoadingBayToIdle,
            SageInstruction::MineAsteroidToRespawn => mine_asteroid_to_respawn::MineAsteroidToRespawn,
            SageInstruction::MintCertificate => mint_certificate::MintCertificate,
            SageInstruction::MintCrewToGame => mint_crew_to_game::MintCrewToGame,
            SageInstruction::RedeemCertificate => redeem_certificate::RedeemCertificate,
            SageInstruction::RegisterCombatConfig => register_combat_config::RegisterCombatConfig,
            SageInstruction::RegisterMineItem => register_mine_item::RegisterMineItem,
            SageInstruction::RegisterPlanet => register_planet::RegisterPlanet,
            SageInstruction::RegisterProgressionConfig => register_progression_config::RegisterProgressionConfig,
            SageInstruction::RegisterResource => register_resource::RegisterResource,
            SageInstruction::RegisterSageCrewConfig => register_sage_crew_config::RegisterSageCrewConfig,
            SageInstruction::RegisterSagePlayerProfile => register_sage_player_profile::RegisterSagePlayerProfile,
            SageInstruction::RegisterSagePointModifier => register_sage_point_modifier::RegisterSagePointModifier,
            SageInstruction::RegisterSector => register_sector::RegisterSector,
            SageInstruction::RegisterShip => register_ship::RegisterShip,
            SageInstruction::RegisterStar => register_star::RegisterStar,
            SageInstruction::RegisterStarbase => register_starbase::RegisterStarbase,
            SageInstruction::RegisterStarbasePlayer => register_starbase_player::RegisterStarbasePlayer,
            SageInstruction::RegisterSurveyDataUnitTracker => register_survey_data_unit_tracker::RegisterSurveyDataUnitTracker,
            SageInstruction::ReloadFleetAbilityPower => reload_fleet_ability_power::ReloadFleetAbilityPower,
            SageInstruction::RemoveCargoPod => remove_cargo_pod::RemoveCargoPod,
            SageInstruction::RemoveConnection => remove_connection::RemoveConnection,
            SageInstruction::RemoveCrewFromGame => remove_crew_from_game::RemoveCrewFromGame,
            SageInstruction::RemoveInvalidShipEscrow => remove_invalid_ship_escrow::RemoveInvalidShipEscrow,
            SageInstruction::RemoveShipEscrow => remove_ship_escrow::RemoveShipEscrow,
            SageInstruction::RepairDockedFleet => repair_docked_fleet::RepairDockedFleet,
            SageInstruction::RepairIdleFleet => repair_idle_fleet::RepairIdleFleet,
            SageInstruction::RepairStarbase => repair_starbase::RepairStarbase,
            SageInstruction::RespawnToLoadingBay => respawn_to_loading_bay::RespawnToLoadingBay,
            SageInstruction::RetrieveLoot => retrieve_loot::RetrieveLoot,
            SageInstruction::ScanForSurveyDataUnits => scan_for_survey_data_units::ScanForSurveyDataUnits,
            SageInstruction::SetNextShip => set_next_ship::SetNextShip,
            SageInstruction::StartCraftingProcess => start_crafting_process::StartCraftingProcess,
            SageInstruction::StartMiningAsteroid => start_mining_asteroid::StartMiningAsteroid,
            SageInstruction::StartStarbaseUpgrade => start_starbase_upgrade::StartStarbaseUpgrade,
            SageInstruction::StartSubwarp => start_subwarp::StartSubwarp,
            SageInstruction::StopCraftingProcess => stop_crafting_process::StopCraftingProcess,
            SageInstruction::StopMiningAsteroid => stop_mining_asteroid::StopMiningAsteroid,
            SageInstruction::StopSubwarp => stop_subwarp::StopSubwarp,
            SageInstruction::SubmitStarbaseUpgradeResource => submit_starbase_upgrade_resource::SubmitStarbaseUpgradeResource,
            SageInstruction::SyncStarbasePlayer => sync_starbase_player::SyncStarbasePlayer,
            SageInstruction::SyncStarbaseUpgradeIngredients => sync_starbase_upgrade_ingredients::SyncStarbaseUpgradeIngredients,
            SageInstruction::TransferCargoAtStarbase => transfer_cargo_at_starbase::TransferCargoAtStarbase,
            SageInstruction::TransferCargoWithinFleet => transfer_cargo_within_fleet::TransferCargoWithinFleet,
            SageInstruction::UnloadFleetCrew => unload_fleet_crew::UnloadFleetCrew,
            SageInstruction::UpdateCombatConfig => update_combat_config::UpdateCombatConfig,
            SageInstruction::UpdateGame => update_game::UpdateGame,
            SageInstruction::UpdateGameState => update_game_state::UpdateGameState,
            SageInstruction::UpdateMineItem => update_mine_item::UpdateMineItem,
            SageInstruction::UpdatePlanet => update_planet::UpdatePlanet,
            SageInstruction::UpdateProgressionConfig => update_progression_config::UpdateProgressionConfig,
            SageInstruction::UpdateResource => update_resource::UpdateResource,
            SageInstruction::UpdateShip => update_ship::UpdateShip,
            SageInstruction::UpdateShipEscrow => update_ship_escrow::UpdateShipEscrow,
            SageInstruction::UpdateShipInFleet => update_ship_in_fleet::UpdateShipInFleet,
            SageInstruction::UpdateStar => update_star::UpdateStar,
            SageInstruction::UpdateStarbase => update_starbase::UpdateStarbase,
            SageInstruction::UpdateSurveyDataUnitTracker => update_survey_data_unit_tracker::UpdateSurveyDataUnitTracker,
            SageInstruction::WarpLane => warp_lane::WarpLane,
            SageInstruction::WarpToCoordinate => warp_to_coordinate::WarpToCoordinate,
            SageInstruction::WithdrawCargoFromFleet => withdraw_cargo_from_fleet::WithdrawCargoFromFleet,
            SageInstruction::WithdrawCargoFromGame => withdraw_cargo_from_game::WithdrawCargoFromGame,
            SageInstruction::WithdrawCraftingIngredient => withdraw_crafting_ingredient::WithdrawCraftingIngredient,
            SageInstruction::BattleLogEvent => battle_log_event::BattleLogEvent,
            SageInstruction::CombatInitiatedEvent => combat_initiated_event::CombatInitiatedEvent,
            // SageInstruction::CombatLogEvent => combat_log_event::CombatLogEvent,
            SageInstruction::CombatLootDropEvent => combat_loot_drop_event::CombatLootDropEvent,
            SageInstruction::CombatParticipantEvent => combat_participant_event::CombatParticipantEvent,
            // SageInstruction::CombatResolvedEvent => combat_resolved_event::CombatResolvedEvent,
            SageInstruction::StarbaseCombatEvent => starbase_combat_event::StarbaseCombatEvent,
        )
    }
}
