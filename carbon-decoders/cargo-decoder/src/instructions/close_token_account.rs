use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0x84ac183c649c8761")]
pub struct CloseTokenAccount {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseTokenAccountInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub authority: solana_pubkey::Pubkey,
    pub cargo_pod: solana_pubkey::Pubkey,
    pub cargo_token_account: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub mint: solana_pubkey::Pubkey,
    pub token_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseTokenAccount {
    type ArrangedAccounts = CloseTokenAccountInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let authority = next_account(&mut iter)?;
        let cargo_pod = next_account(&mut iter)?;
        let cargo_token_account = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let mint = next_account(&mut iter)?;
        let token_program = next_account(&mut iter)?;

        Some(CloseTokenAccountInstructionAccounts {
            funder,
            authority,
            cargo_pod,
            cargo_token_account,
            cargo_type,
            mint,
            token_program,
        })
    }
}
