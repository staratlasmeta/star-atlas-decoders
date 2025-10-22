use super::ProfileFactionDecoder;
pub mod choose_faction;

#[derive(
    carbon_core::InstructionType,
    serde::Serialize,
    serde::Deserialize,
    PartialEq,
    Eq,
    Debug,
    Clone,
    Hash,
)]
pub enum ProfileFactionInstruction {
    ChooseFaction(choose_faction::ChooseFaction),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for ProfileFactionDecoder {
    type InstructionType = ProfileFactionInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            ProfileFactionInstruction::ChooseFaction => choose_faction::ChooseFaction,
        )
    }
}
