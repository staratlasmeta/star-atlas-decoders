

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0x1fa8190ed4f057e1")]
pub struct CloseFeePayer{
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct CloseFeePayerInstructionAccounts {
    pub fee_payer: solana_pubkey::Pubkey,
    pub payment_account: solana_pubkey::Pubkey,
    pub recipient: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for CloseFeePayer {
    type ArrangedAccounts = CloseFeePayerInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            fee_payer,
            payment_account,
            recipient,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(CloseFeePayerInstructionAccounts {
            fee_payer: fee_payer.pubkey,
            payment_account: payment_account.pubkey,
            recipient: recipient.pubkey,
            system_program: system_program.pubkey,
        })
    }
}