use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum SizeClass {
    XxSmall,
    XSmall,
    Small,
    Medium,
    Large,
    Capital,
    Commander,
    Titan,
}
