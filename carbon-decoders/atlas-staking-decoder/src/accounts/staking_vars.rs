use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x82ae3ac279829990")]
pub struct StakingVars {
    pub authority: solana_pubkey::Pubkey,
    pub bump: u8,
}
