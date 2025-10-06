use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RegisterSurveyDataUnitTrackerInput {
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
    pub key_index: u16,
}
