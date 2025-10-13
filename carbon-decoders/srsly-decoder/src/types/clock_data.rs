

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ClockData {
    pub slot: u64,
    pub epoch: u64,
    pub unix_timestamp: i64,
}
