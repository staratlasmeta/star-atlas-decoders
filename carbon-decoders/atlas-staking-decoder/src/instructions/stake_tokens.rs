use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x887e5ba228830d7f")]
pub struct StakeTokens {
    pub stake_amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct StakeTokensInstructionAccounts {
    pub user: solana_pubkey::Pubkey,
    pub stake_mint: solana_pubkey::Pubkey,
    pub token_source: solana_pubkey::Pubkey,
    pub registered_stake: solana_pubkey::Pubkey,
    pub staking_account: solana_pubkey::Pubkey,
    pub token_escrow: solana_pubkey::Pubkey,
    pub rent: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
    pub associated_token_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for StakeTokens {
    type ArrangedAccounts = StakeTokensInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let user = next_account(&mut iter)?;
        let stake_mint = next_account(&mut iter)?;
        let token_source = next_account(&mut iter)?;
        let registered_stake = next_account(&mut iter)?;
        let staking_account = next_account(&mut iter)?;
        let token_escrow = next_account(&mut iter)?;
        let rent = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;
        let associated_token_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(StakeTokensInstructionAccounts {
            user,
            stake_mint,
            token_source,
            registered_stake,
            staking_account,
            token_escrow,
            rent,
            token_program,
            associated_token_program,
            system_program,
        })
    }
}
