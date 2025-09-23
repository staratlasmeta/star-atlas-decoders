use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xea7fe35a9041556f")]
pub struct SurveyDataUnitTracker {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub sdu_mint: solana_pubkey::Pubkey,
    pub resource_mint: solana_pubkey::Pubkey,
    pub signer: solana_pubkey::Pubkey,
    pub signer_bump: u8,
    pub coordinates_range: [i64; 2],
    pub css_coordinates: [[i64; 2]; 3],
    pub origin_coordinates: [i64; 2],
    pub css_max_distance: u32,
    pub origin_max_distance: u32,
    pub distance_weighting: u32,
    pub t_max: i64,
    pub x_mul: u32,
    pub y_mul: u32,
    pub z_mul: u32,
    pub sdu_max_per_sector: u32,
    pub scan_chance_regen_period: i16,
}
