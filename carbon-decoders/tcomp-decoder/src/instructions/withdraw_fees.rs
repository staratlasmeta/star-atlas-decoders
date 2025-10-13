

use carbon_core::{CarbonDeserialize, borsh};


#[derive(CarbonDeserialize, Debug, serde::Serialize, serde::Deserialize, PartialEq, Eq, Clone, Hash)]
#[carbon(discriminator = "0xc6d4ab6d90d7ae59")]
pub struct WithdrawFees{
    pub amount: u64,
}

#[derive(Debug, PartialEq, Eq, Clone, Hash, serde::Serialize, serde::Deserialize)]
pub struct WithdrawFeesInstructionAccounts {
    pub tswap: solana_pubkey::Pubkey,
    pub tcomp: solana_pubkey::Pubkey,
    pub cosigner: solana_pubkey::Pubkey,
    pub owner: solana_pubkey::Pubkey,
    pub destination: solana_pubkey::Pubkey,
    pub system_program: solana_pubkey::Pubkey,
}

impl carbon_core::deserialize::ArrangeAccounts for WithdrawFees {
    type ArrangedAccounts = WithdrawFeesInstructionAccounts;

    fn arrange_accounts(accounts: &[solana_instruction::AccountMeta]) -> Option<Self::ArrangedAccounts> {
        let [
            tswap,
            tcomp,
            cosigner,
            owner,
            destination,
            system_program,
            _remaining @ ..
        ] = accounts else {
            return None;
        };
       

        Some(WithdrawFeesInstructionAccounts {
            tswap: tswap.pubkey,
            tcomp: tcomp.pubkey,
            cosigner: cosigner.pubkey,
            owner: owner.pubkey,
            destination: destination.pubkey,
            system_program: system_program.pubkey,
        })
    }
}