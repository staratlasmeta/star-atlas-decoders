mod analysis;
mod config;
mod decoder;
mod rpc;

use analysis::AnalysisContext;
use anyhow::{Context, Result};
use decoder::{impl_type_identifier, run_decoder_analysis};
use tracing_subscriber::EnvFilter;

// ============================================================
// Feature-gated decoder imports
// ============================================================

#[cfg(feature = "sage-starbased")]
use carbon_sage_starbased_decoder::{SageDecoder, accounts::SageAccount};

#[cfg(feature = "sage-holosim")]
use carbon_sage_holosim_decoder::{
    SageDecoder as SageHolosimDecoder, accounts::SageAccount as SageHolosimAccount,
};

#[cfg(feature = "atlas-staking")]
use carbon_atlas_staking_decoder::{AtlasStakingDecoder, accounts::AtlasStakingAccount};

#[cfg(feature = "locked-voter")]
use carbon_locked_voter_decoder::{LockedVoterDecoder, accounts::LockedVoterAccount};

#[cfg(feature = "marketplace")]
use carbon_marketplace_decoder::{MarketplaceDecoder, accounts::MarketplaceAccount};

#[cfg(feature = "atlas-fee-payer")]
use carbon_atlas_fee_payer_decoder::{AtlasFeePayerDecoder, accounts::AtlasFeePayerAccount};

#[cfg(feature = "cargo")]
use carbon_cargo_decoder::{CargoDecoder, accounts::CargoAccount};

#[cfg(feature = "crafting")]
use carbon_crafting_decoder::{CraftingDecoder, accounts::CraftingAccount};

#[cfg(feature = "crew")]
use carbon_crew_decoder::{CrewDecoder, accounts::CrewAccount};

#[cfg(feature = "profile-vault")]
use carbon_profile_vault_decoder::{ProfileVaultDecoder, accounts::ProfileVaultAccount};

#[cfg(feature = "srsly")]
use carbon_srsly_decoder::{SrslyDecoder, accounts::SrslyAccount};

#[cfg(feature = "tcomp")]
use carbon_tcomp_decoder::{TcompDecoder, accounts::TcompAccount};

#[cfg(feature = "player-profile")]
use carbon_player_profile_decoder::{PlayerProfileDecoder, accounts::PlayerProfileAccount};

#[cfg(feature = "points")]
use carbon_points_decoder::{PointsDecoder, accounts::PointsAccount};

#[cfg(feature = "points-store")]
use carbon_points_store_decoder::{PointsStoreDecoder, accounts::PointsStoreAccount};

#[cfg(feature = "profile-faction")]
use carbon_profile_faction_decoder::{ProfileFactionDecoder, accounts::ProfileFactionAccount};

#[cfg(feature = "score")]
use carbon_score_decoder::{ScoreDecoder, accounts::ScoreAccount};

#[cfg(feature = "claim-stake")]
use carbon_claim_stake_decoder::{ClaimStakeDecoder, accounts::ClaimStakeAccount};

#[cfg(feature = "proxy-rewarder")]
use carbon_proxy_rewarder_decoder::{ProxyRewarderDecoder, accounts::ProxyRewarderAccount};

// ============================================================
// AccountTypeIdentifier implementations
// ============================================================

#[cfg(feature = "sage-starbased")]
impl_type_identifier!(SageAccount =>
    CraftingInstance, DisbandedFleet, Fleet, FleetShips, Game, GameState,
    MineItem, Planet, PlayerCrewRecord, ProgressionConfig, Resource,
    SageCrewConfig, SagePlayerProfile, Sector, Ship, Star, Starbase,
    StarbasePlayer, SurveyDataUnitTracker
);

#[cfg(feature = "sage-holosim")]
impl_type_identifier!(SageHolosimAccount =>
    CombatConfig, CraftingInstance, DisbandedFleet, Fleet, FleetShips,
    Game, GameState, Loot, MineItem, Planet, PlayerCrewRecord,
    ProgressionConfig, Resource, SageCrewConfig, SagePlayerProfile,
    Sector, Ship, Star, Starbase, StarbasePlayer, SurveyDataUnitTracker
);

#[cfg(feature = "atlas-staking")]
impl_type_identifier!(AtlasStakingAccount => RegisteredStake, StakingAccount, StakingVars);

#[cfg(feature = "locked-voter")]
impl_type_identifier!(LockedVoterAccount => Escrow, Locker, LockerWhitelistEntry);

#[cfg(feature = "marketplace")]
impl_type_identifier!(MarketplaceAccount =>
    AtlasRateAccount, FeeReduction, MarketVars, OpenOrdersCounter,
    OrderAccount, RegisteredCurrency
);

#[cfg(feature = "atlas-fee-payer")]
impl_type_identifier!(AtlasFeePayerAccount => FeePayer, FeePayerRates);

#[cfg(feature = "cargo")]
impl_type_identifier!(CargoAccount => CargoPod, CargoStatsDefinition, CargoType);

#[cfg(feature = "crafting")]
impl_type_identifier!(CraftingAccount =>
    CraftableItem, CraftingFacility, CraftingProcess, Domain, Recipe, RecipeCategory
);

#[cfg(feature = "crew")]
impl_type_identifier!(CrewAccount => CrewConfig, PackTiers, PackType, SftRedemption, UserRedemption);

#[cfg(feature = "profile-vault")]
impl_type_identifier!(ProfileVaultAccount => VaultAuthority);

#[cfg(feature = "srsly")]
impl_type_identifier!(SrslyAccount => ContractState, Fleet, RentalState, Thread);

#[cfg(feature = "tcomp")]
impl_type_identifier!(TcompAccount => BidState, ListState);

#[cfg(feature = "player-profile")]
impl_type_identifier!(PlayerProfileAccount => PlayerName, Profile, ProfileRoleMembership, Role);

#[cfg(feature = "points")]
impl_type_identifier!(PointsAccount => PointCategory, PointsModifier, UserPointsAccount);

#[cfg(feature = "points-store")]
impl_type_identifier!(PointsStoreAccount => PointsStore, RedemptionConfig, UserRedemption);

#[cfg(feature = "profile-faction")]
impl_type_identifier!(ProfileFactionAccount => ProfileFactionAccount);

#[cfg(feature = "score")]
impl_type_identifier!(ScoreAccount => ScoreVars, ScoreVarsShip, ShipStaking);

#[cfg(feature = "claim-stake")]
impl_type_identifier!(ClaimStakeAccount => ClaimStakeVar, ClaimStaking, GlobalVars);

#[cfg(feature = "proxy-rewarder")]
impl_type_identifier!(ProxyRewarderAccount => Proxy, ProxyEscrow, RegisteredLocker, TreasuryAuthority);

// ============================================================
// Main entry point
// ============================================================

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
        )
        .init();

    // Load .env file (non-fatal if missing)
    let _ = dotenvy::dotenv();

    let config = config::Config::from_env()
        .context("Failed to load configuration - ensure RPC_URL is set")?;

    let program_id = config.get_program_id()?;

    tracing::info!("=== Decoder Explorer ===");
    tracing::info!("Program ID: {}", program_id);
    tracing::info!("RPC URL: {}", config.redacted_rpc_url());

    let client = rpc::init_rpc_client(&config.rpc_url)?;
    let accounts = rpc::get_program_accounts(&client, &program_id)?;

    if accounts.is_empty() {
        tracing::warn!("No accounts found for program {}", program_id);
        return Ok(());
    }

    let ctx = AnalysisContext::new(&accounts);

    run_analysis(&ctx)?;

    tracing::info!("\n=== Analysis Complete ===");
    Ok(())
}

fn run_analysis(ctx: &AnalysisContext) -> Result<()> {
    // Size distribution (decoder-agnostic)
    analysis::size_distribution::analyze(ctx)?;

    // Type classification (decoder-specific)
    run_decoder_analysis!(ctx, "sage-starbased", SageDecoder, SageAccount);
    run_decoder_analysis!(ctx, "sage-holosim", SageHolosimDecoder, SageHolosimAccount);
    run_decoder_analysis!(
        ctx,
        "atlas-staking",
        AtlasStakingDecoder,
        AtlasStakingAccount
    );
    run_decoder_analysis!(ctx, "locked-voter", LockedVoterDecoder, LockedVoterAccount);
    run_decoder_analysis!(ctx, "marketplace", MarketplaceDecoder, MarketplaceAccount);
    run_decoder_analysis!(
        ctx,
        "atlas-fee-payer",
        AtlasFeePayerDecoder,
        AtlasFeePayerAccount
    );
    run_decoder_analysis!(ctx, "cargo", CargoDecoder, CargoAccount);
    run_decoder_analysis!(ctx, "crafting", CraftingDecoder, CraftingAccount);
    run_decoder_analysis!(ctx, "crew", CrewDecoder, CrewAccount);
    run_decoder_analysis!(
        ctx,
        "profile-vault",
        ProfileVaultDecoder,
        ProfileVaultAccount
    );
    run_decoder_analysis!(ctx, "srsly", SrslyDecoder, SrslyAccount);
    run_decoder_analysis!(ctx, "tcomp", TcompDecoder, TcompAccount);
    run_decoder_analysis!(
        ctx,
        "player-profile",
        PlayerProfileDecoder,
        PlayerProfileAccount
    );
    run_decoder_analysis!(ctx, "points", PointsDecoder, PointsAccount);
    run_decoder_analysis!(ctx, "points-store", PointsStoreDecoder, PointsStoreAccount);
    run_decoder_analysis!(
        ctx,
        "profile-faction",
        ProfileFactionDecoder,
        ProfileFactionAccount
    );
    run_decoder_analysis!(ctx, "score", ScoreDecoder, ScoreAccount);
    run_decoder_analysis!(ctx, "claim-stake", ClaimStakeDecoder, ClaimStakeAccount);
    run_decoder_analysis!(ctx, "proxy-rewarder", ProxyRewarderDecoder, ProxyRewarderAccount);

    Ok(())
}
