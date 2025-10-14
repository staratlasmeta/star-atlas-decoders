use super::ProfileVaultDecoder;
pub mod close_vault;
pub mod create_vault_authority;
pub mod drain_vault;

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
pub enum ProfileVaultInstruction {
    CloseVault(close_vault::CloseVault),
    CreateVaultAuthority(create_vault_authority::CreateVaultAuthority),
    DrainVault(drain_vault::DrainVault),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for ProfileVaultDecoder {
    type InstructionType = ProfileVaultInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            ProfileVaultInstruction::CloseVault => close_vault::CloseVault,
            ProfileVaultInstruction::CreateVaultAuthority => create_vault_authority::CreateVaultAuthority,
            ProfileVaultInstruction::DrainVault => drain_vault::DrainVault,
        )
    }
}
