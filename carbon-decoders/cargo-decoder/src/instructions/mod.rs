use super::CargoDecoder;
pub mod add_cargo;
pub mod close_cargo_pod;
pub mod close_token_account;
pub mod consume_cargo;
pub mod init_cargo_pod;
pub mod init_cargo_type;
pub mod init_cargo_type_for_next_seq_id;
pub mod init_cargo_type_from_old_cargo_type;
pub mod init_definition;
pub mod legitimize_cargo;
pub mod mint_to;
pub mod remove_cargo;
pub mod transfer_authority;
pub mod transfer_cargo;
pub mod update_cargo_pod;
pub mod update_definition;
pub mod update_pod_token_account;

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
pub enum CargoInstruction {
    AddCargo(add_cargo::AddCargo),
    CloseCargoPod(close_cargo_pod::CloseCargoPod),
    CloseTokenAccount(close_token_account::CloseTokenAccount),
    ConsumeCargo(consume_cargo::ConsumeCargo),
    InitCargoPod(init_cargo_pod::InitCargoPod),
    InitCargoType(init_cargo_type::InitCargoType),
    InitCargoTypeForNextSeqId(init_cargo_type_for_next_seq_id::InitCargoTypeForNextSeqId),
    InitCargoTypeFromOldCargoType(
        init_cargo_type_from_old_cargo_type::InitCargoTypeFromOldCargoType,
    ),
    InitDefinition(init_definition::InitDefinition),
    LegitimizeCargo(legitimize_cargo::LegitimizeCargo),
    MintTo(mint_to::MintTo),
    RemoveCargo(remove_cargo::RemoveCargo),
    TransferAuthority(transfer_authority::TransferAuthority),
    TransferCargo(transfer_cargo::TransferCargo),
    UpdateCargoPod(update_cargo_pod::UpdateCargoPod),
    UpdateDefinition(update_definition::UpdateDefinition),
    UpdatePodTokenAccount(update_pod_token_account::UpdatePodTokenAccount),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for CargoDecoder {
    type InstructionType = CargoInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CargoInstruction::AddCargo => add_cargo::AddCargo,
            CargoInstruction::CloseCargoPod => close_cargo_pod::CloseCargoPod,
            CargoInstruction::CloseTokenAccount => close_token_account::CloseTokenAccount,
            CargoInstruction::ConsumeCargo => consume_cargo::ConsumeCargo,
            CargoInstruction::InitCargoPod => init_cargo_pod::InitCargoPod,
            CargoInstruction::InitCargoType => init_cargo_type::InitCargoType,
            CargoInstruction::InitCargoTypeForNextSeqId => init_cargo_type_for_next_seq_id::InitCargoTypeForNextSeqId,
            CargoInstruction::InitCargoTypeFromOldCargoType => init_cargo_type_from_old_cargo_type::InitCargoTypeFromOldCargoType,
            CargoInstruction::InitDefinition => init_definition::InitDefinition,
            CargoInstruction::LegitimizeCargo => legitimize_cargo::LegitimizeCargo,
            CargoInstruction::MintTo => mint_to::MintTo,
            CargoInstruction::RemoveCargo => remove_cargo::RemoveCargo,
            CargoInstruction::TransferAuthority => transfer_authority::TransferAuthority,
            CargoInstruction::TransferCargo => transfer_cargo::TransferCargo,
            CargoInstruction::UpdateCargoPod => update_cargo_pod::UpdateCargoPod,
            CargoInstruction::UpdateDefinition => update_definition::UpdateDefinition,
            CargoInstruction::UpdatePodTokenAccount => update_pod_token_account::UpdatePodTokenAccount,
        )
    }
}
