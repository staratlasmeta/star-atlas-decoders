use super::*;

use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub struct RiskZonesData {
    pub mud_security_zone: RiskZoneData,
    pub oni_security_zone: RiskZoneData,
    pub ustur_security_zone: RiskZoneData,
    pub high_risk_zone: RiskZoneData,
    pub medium_risk_zone: RiskZoneData,
}
