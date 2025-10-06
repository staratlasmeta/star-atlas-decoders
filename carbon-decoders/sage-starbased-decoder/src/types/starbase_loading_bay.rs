use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct StarbaseLoadingBay {
    pub starbase: solana_pubkey::Pubkey,
    pub last_update: i64,
}
