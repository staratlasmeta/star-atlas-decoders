use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct MiscStatsUnpacked {
    pub required_crew: u16,
    pub passenger_capacity: u16,
    pub crew_count: u16,
    pub rented_crew: u16,
    pub respawn_time: u16,
    pub scan_cool_down: u16,
    pub sdu_per_scan: u32,
    pub scan_cost: u32,
    pub placeholder: u32,
    pub placeholder2: u32,
    pub placeholder3: u32,
}
