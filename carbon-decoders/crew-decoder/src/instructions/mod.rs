



use super::CrewDecoder;
pub mod mint_crew_member;
pub mod redeem_crew_pack;
pub mod register_crew_config;
pub mod register_pack_tiers;
pub mod register_pack_type;
pub mod register_sft_redemption;
pub mod update_pack_tiers;

#[derive(carbon_core::InstructionType, serde::Serialize, serde::Deserialize, PartialEq, Eq, Debug, Clone, Hash)]
pub enum CrewInstruction {
    MintCrewMember(mint_crew_member::MintCrewMember),
    RedeemCrewPack(redeem_crew_pack::RedeemCrewPack),
    RegisterCrewConfig(register_crew_config::RegisterCrewConfig),
    RegisterPackTiers(register_pack_tiers::RegisterPackTiers),
    RegisterPackType(register_pack_type::RegisterPackType),
    RegisterSftRedemption(register_sft_redemption::RegisterSftRedemption),
    UpdatePackTiers(update_pack_tiers::UpdatePackTiers),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for CrewDecoder {
    type InstructionType = CrewInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            CrewInstruction::MintCrewMember => mint_crew_member::MintCrewMember,
            CrewInstruction::RedeemCrewPack => redeem_crew_pack::RedeemCrewPack,
            CrewInstruction::RegisterCrewConfig => register_crew_config::RegisterCrewConfig,
            CrewInstruction::RegisterPackTiers => register_pack_tiers::RegisterPackTiers,
            CrewInstruction::RegisterPackType => register_pack_type::RegisterPackType,
            CrewInstruction::RegisterSftRedemption => register_sft_redemption::RegisterSftRedemption,
            CrewInstruction::UpdatePackTiers => update_pack_tiers::UpdatePackTiers,
        )
    }
}