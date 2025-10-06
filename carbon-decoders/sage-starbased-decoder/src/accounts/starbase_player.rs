use super::super::types::*;

use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct StarbasePlayer {
    pub version: u8,
    pub player_profile: solana_pubkey::Pubkey,
    pub game_id: solana_pubkey::Pubkey,
    pub starbase: solana_pubkey::Pubkey,
    pub sage_player_profile: solana_pubkey::Pubkey,
    pub bump: u8,
    pub ship_escrow_count: u32,
    pub old_total_crew: u32,
    pub new_total_crew: u32,
    pub busy_crew: u64,
    pub update_id: u64,
    pub updated_ship_escrow_count: u32,
    pub ship_escrows: Vec<WrappedShipEscrow>,
}

impl borsh::de::BorshDeserialize for StarbasePlayer
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    u32: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
    WrappedShipEscrow: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> Result<Self, borsh::maybestd::io::Error> {
        let version = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let player_profile = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let game_id = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let starbase = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let sage_player_profile = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let bump = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let ship_escrow_count = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let old_total_crew = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let new_total_crew = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let busy_crew = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let update_id = borsh::BorshDeserialize::deserialize_reader(reader)?;
        let updated_ship_escrow_count = borsh::BorshDeserialize::deserialize_reader(reader)?;

        let mut ship_escrows = Vec::with_capacity(updated_ship_escrow_count as usize);
        for _ in 0..updated_ship_escrow_count {
            ship_escrows.push(borsh::BorshDeserialize::deserialize_reader(reader)?);
        }

        Ok(Self {
            version,
            player_profile,
            game_id,
            starbase,
            sage_player_profile,
            bump,
            ship_escrow_count,
            old_total_crew,
            new_total_crew,
            busy_crew,
            update_id,
            updated_ship_escrow_count,
            ship_escrows,
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for StarbasePlayer {
    const DISCRIMINATOR: &'static [u8] = &[192u8, 234u8, 144u8, 86u8, 72u8, 19u8, 5u8, 99u8];
    fn deserialize(data: &[u8]) -> Option<Self> {
        if data.len() < Self::DISCRIMINATOR.len() {
            return None;
        }
        let (disc, mut rest) = data.split_at(Self::DISCRIMINATOR.len());
        if disc != Self::DISCRIMINATOR {
            return None;
        }
        BorshDeserialize::deserialize(&mut rest).ok()
    }
}
