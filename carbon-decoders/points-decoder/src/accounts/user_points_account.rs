use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x25ec06153219f954")]
pub struct UserPointsAccount {
    pub version: u8,
    pub profile: solana_pubkey::Pubkey,
    pub point_category: solana_pubkey::Pubkey,
    pub earned_points: u64,
    pub spent_points: u64,
    pub level: u16,
    pub daily_earned_points: u64,
    pub last_earned_points_timestamp: i64,
    pub bump: u8,
}
