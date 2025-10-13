



use super::AtlasFeePayerDecoder;
pub mod close_fee_payer;
pub mod create_fee_payer;
pub mod create_fee_payer_rates;
pub mod post_transaction;
pub mod post_transaction_no_vault;
pub mod pre_transaction;
pub mod set_fee_payer_rates;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum AtlasFeePayerInstruction {
    CloseFeePayer(close_fee_payer::CloseFeePayer),
    CreateFeePayer(create_fee_payer::CreateFeePayer),
    CreateFeePayerRates(create_fee_payer_rates::CreateFeePayerRates),
    PostTransaction(post_transaction::PostTransaction),
    PostTransactionNoVault(post_transaction_no_vault::PostTransactionNoVault),
    PreTransaction(pre_transaction::PreTransaction),
    SetFeePayerRates(set_fee_payer_rates::SetFeePayerRates),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for AtlasFeePayerDecoder {
    type InstructionType = AtlasFeePayerInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            AtlasFeePayerInstruction::CloseFeePayer => close_fee_payer::CloseFeePayer,
            AtlasFeePayerInstruction::CreateFeePayer => create_fee_payer::CreateFeePayer,
            AtlasFeePayerInstruction::CreateFeePayerRates => create_fee_payer_rates::CreateFeePayerRates,
            AtlasFeePayerInstruction::PostTransaction => post_transaction::PostTransaction,
            AtlasFeePayerInstruction::PostTransactionNoVault => post_transaction_no_vault::PostTransactionNoVault,
            AtlasFeePayerInstruction::PreTransaction => pre_transaction::PreTransaction,
            AtlasFeePayerInstruction::SetFeePayerRates => set_fee_payer_rates::SetFeePayerRates,
        )
    }
}