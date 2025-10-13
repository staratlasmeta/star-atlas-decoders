



use super::SrslyDecoder;
pub mod accept_rental;
pub mod cancel_rental;
pub mod close_contract;
pub mod close_rental;
pub mod create_contract;
pub mod pay_rental;
pub mod reset_rental;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum SrslyInstruction {
    AcceptRental(accept_rental::AcceptRental),
    CancelRental(cancel_rental::CancelRental),
    CloseContract(close_contract::CloseContract),
    CloseRental(close_rental::CloseRental),
    CreateContract(create_contract::CreateContract),
    PayRental(pay_rental::PayRental),
    ResetRental(reset_rental::ResetRental),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for SrslyDecoder {
    type InstructionType = SrslyInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            SrslyInstruction::AcceptRental => accept_rental::AcceptRental,
            SrslyInstruction::CancelRental => cancel_rental::CancelRental,
            SrslyInstruction::CloseContract => close_contract::CloseContract,
            SrslyInstruction::CloseRental => close_rental::CloseRental,
            SrslyInstruction::CreateContract => create_contract::CreateContract,
            SrslyInstruction::PayRental => pay_rental::PayRental,
            SrslyInstruction::ResetRental => reset_rental::ResetRental,
        )
    }
}