

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc8faab39eb5f3710")]
pub struct PreTransaction{
    pub signer_count: u8,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct PreTransactionInstructionAccounts {
    pub fee_payer: solana_pubkey::Pubkey,
    pub payment_account: solana_pubkey::Pubkey,
    pub instruction_sysvar: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for PreTransaction {
    type ArrangedAccounts = PreTransactionInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            fee_payer,
            payment_account,
            instruction_sysvar,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(PreTransactionInstructionAccounts {
            fee_payer: fee_payer.pubkey,
            payment_account: payment_account.pubkey,
            instruction_sysvar: instruction_sysvar.pubkey,
        })
    }
}