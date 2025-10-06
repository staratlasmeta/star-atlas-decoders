use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct UpdateSurveyDataUnitTrackerInput {
    pub coordinates_range: Option<[i64; 2]>,
    pub css_coordinates: Option<[[i64; 2]; 3]>,
    pub origin_coordinates: Option<[i64; 2]>,
    pub css_max_distance: Option<u32>,
    pub origin_max_distance: Option<u32>,
    pub distance_weighting: Option<u32>,
    pub t_max: Option<i64>,
    pub x_mul: Option<u32>,
    pub y_mul: Option<u32>,
    pub z_mul: Option<u32>,
    pub sdu_max_per_sector: Option<u32>,
    pub scan_chance_regen_period: Option<i16>,
    pub key_index: u16,
}
