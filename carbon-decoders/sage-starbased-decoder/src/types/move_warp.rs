use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MoveWarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub warp_start: i64,
    pub warp_finish: i64,
}
