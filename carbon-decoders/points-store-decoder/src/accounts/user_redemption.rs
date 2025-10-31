use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x40a650064e968ff9")]
pub struct UserRedemption {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub user_points_account: solana_pubkey::Pubkey,
    pub config: solana_pubkey::Pubkey,
    pub points: u64,
    pub day_index: i64,
}
