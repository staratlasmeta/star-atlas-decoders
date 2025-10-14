use carbon_core::{CarbonDeserialize, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
pub enum TriggerContext {
    Account { data_hash: u64 },
    Cron { started_at: i64 },
    Now,
    Slot { started_at: u64 },
    Epoch { started_at: u64 },
    Timestamp { started_at: i64 },
    Pyth { price: i64 },
}
