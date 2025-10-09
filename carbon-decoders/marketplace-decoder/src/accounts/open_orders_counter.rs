use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Deserialize, serde::Serialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xf57031812e21b749")]
pub struct OpenOrdersCounter {
    pub open_order_count: u64,
    pub bump: u8,
}
