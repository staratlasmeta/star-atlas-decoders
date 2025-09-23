use carbon_core::{CarbonDeserialize, account_utils::next_account, borsh};

#[derive(
    CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash,
)]
#[carbon(discriminator = "0xadbe2d74226d2259")]
pub struct CreateCertificateMint {}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CreateCertificateMintInstructionAccounts {
    pub funder: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub cargo_mint: solana_pubkey::Pubkey,
    pub certificate_mint: solana_pubkey::Pubkey,
    pub cargo_type: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub transfer_hook_extra_account_meta_list: solana_pubkey::Pubkey,
    pub transfer_hook_program: solana_pubkey::Pubkey,
    pub token2022_program: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CreateCertificateMint {
    type ArrangedAccounts = CreateCertificateMintInstructionAccounts;

    fn arrange_accounts(
        accounts: &[solana_instruction::AccountMeta],
    ) -> Option<Self::ArrangedAccounts> {
        let mut iter = accounts.iter();
        let funder = next_account(&mut iter)?;
        let starbase = next_account(&mut iter)?;
        let cargo_mint = next_account(&mut iter)?;
        let certificate_mint = next_account(&mut iter)?;
        let cargo_type = next_account(&mut iter)?;
        let game_id = next_account(&mut iter)?;
        let transfer_hook_extra_account_meta_list = next_account(&mut iter)?;
        let transfer_hook_program = next_account(&mut iter)?;
        let token2022_program = next_account(&mut iter)?;
        let system_program = next_account(&mut iter)?;

        Some(CreateCertificateMintInstructionAccounts {
            funder,
            starbase,
            cargo_mint,
            certificate_mint,
            cargo_type,
            game_id,
            transfer_hook_extra_account_meta_list,
            transfer_hook_program,
            token2022_program,
            system_program,
        })
    }
}
