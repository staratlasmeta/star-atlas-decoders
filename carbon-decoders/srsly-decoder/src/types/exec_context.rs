
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub struct ExecContext {
    pub exec_index: u64,
    pub execs_since_reimbursement: u64,
    pub execs_since_slot: u64,
    pub last_exec_at: u64,
    pub last_exec_timestamp: i64,
    pub trigger_context: TriggerContext,
}
