use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf6abe890daec21a1")]
pub struct AtlasRateAccount {
    pub atlas_rate: u64,
}
