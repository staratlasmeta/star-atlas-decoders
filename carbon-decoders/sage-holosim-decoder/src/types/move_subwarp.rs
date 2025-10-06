use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MoveSubwarp {
    pub from_sector: [i64; 2],
    pub to_sector: [i64; 2],
    pub current_sector: [i64; 2],
    pub departure_time: i64,
    pub arrival_time: i64,
    pub fuel_expenditure: u64,
    pub last_update: i64,
}
