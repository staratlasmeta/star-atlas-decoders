use solana_sdk::{account::Account, pubkey::Pubkey};

pub mod sample_accounts;
pub mod size_distribution;
pub mod type_classification;

pub use crate::decoder::AccountTypeIdentifier;

/// Generic analysis context holding fetched accounts
pub struct AnalysisContext<'a> {
    pub accounts: &'a [(Pubkey, Account)],
}

impl<'a> AnalysisContext<'a> {
    pub fn new(accounts: &'a [(Pubkey, Account)]) -> Self {
        Self { accounts }
    }

    pub fn total_accounts(&self) -> usize {
        self.accounts.len()
    }

    pub fn total_data_size(&self) -> usize {
        self.accounts
            .iter()
            .map(|(_, account)| account.data.len())
            .sum()
    }
}
