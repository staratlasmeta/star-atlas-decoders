use anyhow::{Context, Result};
use serde::Deserialize;
use solana_pubkey::Pubkey;
use std::str::FromStr;

// Feature-gated default program IDs
#[cfg(feature = "sage-starbased")]
use carbon_sage_starbased_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "sage-holosim")]
use carbon_sage_holosim_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "atlas-staking")]
use carbon_atlas_staking_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "locked-voter")]
use carbon_locked_voter_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "marketplace")]
use carbon_marketplace_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "atlas-fee-payer")]
use carbon_atlas_fee_payer_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "cargo")]
use carbon_cargo_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "crafting")]
use carbon_crafting_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "crew")]
use carbon_crew_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "profile-vault")]
use carbon_profile_vault_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "srsly")]
use carbon_srsly_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "tcomp")]
use carbon_tcomp_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "player-profile")]
use carbon_player_profile_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "points")]
use carbon_points_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "points-store")]
use carbon_points_store_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[cfg(feature = "profile-faction")]
use carbon_profile_faction_decoder::PROGRAM_ID as DEFAULT_PROGRAM_ID;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub rpc_url: String,

    #[serde(default)]
    pub program_id: Option<String>,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        envy::from_env().context("Failed to load configuration from environment variables")
    }

    /// Get the program ID, using feature-specific default if not overridden
    pub fn get_program_id(&self) -> Result<Pubkey> {
        match &self.program_id {
            Some(id) => Pubkey::from_str(id).with_context(|| format!("Invalid program ID: {}", id)),
            None => Ok(DEFAULT_PROGRAM_ID),
        }
    }

    /// Get a redacted version of the RPC URL safe for logging
    pub fn redacted_rpc_url(&self) -> String {
        if let Some(pos) = self.rpc_url.find('?') {
            format!("{}?[REDACTED]", &self.rpc_url[..pos])
        } else {
            self.rpc_url.clone()
        }
    }
}
