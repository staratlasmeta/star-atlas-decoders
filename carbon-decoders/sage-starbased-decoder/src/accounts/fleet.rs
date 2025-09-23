use super::super::types::*;

use carbon_core::borsh::{self, BorshDeserialize};

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Fleet {
    pub version: u8,
    pub game_id: solana_pubkey::Pubkey,
    pub owner_profile: solana_pubkey::Pubkey,
    pub fleet_ships: solana_pubkey::Pubkey,
    pub sub_profile: OptionalNonSystemPubkey,
    pub sub_profile_invalidator: solana_pubkey::Pubkey,
    pub faction: u8,
    pub fleet_label: [u8; 32],
    pub ship_counts: ShipCounts,
    pub warp_cooldown_expires_at: i64,
    pub scan_cooldown_expires_at: i64,
    pub stats: ShipStats,
    pub cargo_hold: solana_pubkey::Pubkey,
    pub fuel_tank: solana_pubkey::Pubkey,
    pub ammo_bank: solana_pubkey::Pubkey,
    pub update_id: u64,
    pub bump: u8,
    pub fleet_state: FleetState,
}

#[derive(Debug, PartialEq, serde::Serialize, serde::Deserialize, BorshDeserialize)]
pub enum FleetState {
    StarbaseLoadingBay(StarbaseLoadingBay),
    Idle(Idle),
    MineAsteroid(MineAsteroid),
    MoveWarp(MoveWarp),
    MoveSubwarp(MoveSubwarp),
    Respawn(Respawn),
}

impl borsh::de::BorshDeserialize for Fleet
where
    u8: borsh::BorshDeserialize,
    solana_pubkey::Pubkey: borsh::BorshDeserialize,
    OptionalNonSystemPubkey: borsh::BorshDeserialize,
    [u8; 32]: borsh::BorshDeserialize,
    ShipCounts: borsh::BorshDeserialize,
    i64: borsh::BorshDeserialize,
    ShipStats: borsh::BorshDeserialize,
    u64: borsh::BorshDeserialize,
{
    fn deserialize_reader<R: borsh::maybestd::io::Read>(
        reader: &mut R,
    ) -> Result<Self, borsh::maybestd::io::Error> {
        Ok(Self {
            version: borsh::BorshDeserialize::deserialize_reader(reader)?,
            game_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            owner_profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fleet_ships: borsh::BorshDeserialize::deserialize_reader(reader)?,
            sub_profile: borsh::BorshDeserialize::deserialize_reader(reader)?,
            sub_profile_invalidator: borsh::BorshDeserialize::deserialize_reader(reader)?,
            faction: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fleet_label: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ship_counts: borsh::BorshDeserialize::deserialize_reader(reader)?,
            warp_cooldown_expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            scan_cooldown_expires_at: borsh::BorshDeserialize::deserialize_reader(reader)?,
            stats: borsh::BorshDeserialize::deserialize_reader(reader)?,
            cargo_hold: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fuel_tank: borsh::BorshDeserialize::deserialize_reader(reader)?,
            ammo_bank: borsh::BorshDeserialize::deserialize_reader(reader)?,
            update_id: borsh::BorshDeserialize::deserialize_reader(reader)?,
            bump: borsh::BorshDeserialize::deserialize_reader(reader)?,
            fleet_state: borsh::BorshDeserialize::deserialize_reader(reader)?,
        })
    }
}

#[automatically_derived]
impl carbon_core::deserialize::CarbonDeserialize for Fleet {
    const DISCRIMINATOR: &'static [u8] = &[109u8, 207u8, 251u8, 48u8, 106u8, 2u8, 136u8, 163u8];
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
