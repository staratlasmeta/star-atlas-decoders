
use super::*;

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
pub enum Trigger {
    Account
                {
                    address: solana_pubkey::Pubkey,
                    offset: u64,
                    size: u64,
                }
    ,
    Cron
                {
                    schedule: String,
                    skippable: bool,
                }
    ,
    Now,
    Slot
                {
                    slot: u64,
                }
    ,
    Epoch
                {
                    epoch: u64,
                }
    ,
    Timestamp
                {
                    unix_ts: i64,
                }
    ,
    Pyth
                {
                    price_feed: solana_pubkey::Pubkey,
                    equality: Equality,
                    limit: i64,
                }
    ,
}


