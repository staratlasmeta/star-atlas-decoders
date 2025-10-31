use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xbfe3d9b23acd2c27")]
pub struct PointsModifier {
    pub version: u8,
    pub point_category: solana_pubkey::Pubkey,
    pub can_increment: u8,
    pub can_decrement: u8,
}
