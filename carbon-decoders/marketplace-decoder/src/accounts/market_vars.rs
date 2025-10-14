use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xff8e86193801db7c")]
pub struct MarketVars {
    pub update_authority_master: solana_pubkey::Pubkey,
    pub bump: u8,
}
