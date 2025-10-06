use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RiskZoneDataUnpacked {
    pub center: [i64; 2],
    pub radius: u64,
}
