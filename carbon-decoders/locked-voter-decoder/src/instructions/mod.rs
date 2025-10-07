use super::LockedVoterDecoder;
pub mod activate_proposal;
pub mod approve_lock_privilege_event;
pub mod approve_program_lock_privilege;
pub mod cast_vote;
pub mod exit;
pub mod exit_escrow_event;
pub mod lock;
pub mod lock_event;
pub mod lock_with_whitelist_entry;
pub mod locker_set_params_event;
pub mod new_escrow;
pub mod new_escrow_event;
pub mod new_locker;
pub mod new_locker_event;
pub mod revoke_lock_privilege_event;
pub mod revoke_program_lock_privilege;
pub mod set_locker_params;
pub mod set_vote_delegate;
pub mod set_vote_delegate_event;

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
pub enum LockedVoterInstruction {
    NewLocker(new_locker::NewLocker),
    NewEscrow(new_escrow::NewEscrow),
    Lock(lock::Lock),
    LockWithWhitelistEntry(lock_with_whitelist_entry::LockWithWhitelistEntry),
    Exit(exit::Exit),
    ActivateProposal(activate_proposal::ActivateProposal),
    CastVote(cast_vote::CastVote),
    SetVoteDelegate(set_vote_delegate::SetVoteDelegate),
    SetLockerParams(set_locker_params::SetLockerParams),
    ApproveProgramLockPrivilege(approve_program_lock_privilege::ApproveProgramLockPrivilege),
    RevokeProgramLockPrivilege(revoke_program_lock_privilege::RevokeProgramLockPrivilege),
    ApproveLockPrivilegeEvent(approve_lock_privilege_event::ApproveLockPrivilegeEvent),
    ExitEscrowEvent(exit_escrow_event::ExitEscrowEvent),
    LockEvent(lock_event::LockEvent),
    NewEscrowEvent(new_escrow_event::NewEscrowEvent),
    NewLockerEvent(new_locker_event::NewLockerEvent),
    RevokeLockPrivilegeEvent(revoke_lock_privilege_event::RevokeLockPrivilegeEvent),
    LockerSetParamsEvent(locker_set_params_event::LockerSetParamsEvent),
    SetVoteDelegateEvent(set_vote_delegate_event::SetVoteDelegateEvent),
}

impl<'a> carbon_core::instruction::InstructionDecoder<'a> for LockedVoterDecoder {
    type InstructionType = LockedVoterInstruction;

    fn decode_instruction(
        &self,
        instruction: &solana_instruction::Instruction,
    ) -> Option<carbon_core::instruction::DecodedInstruction<Self::InstructionType>> {
        carbon_core::try_decode_instructions!(instruction,
            LockedVoterInstruction::NewLocker => new_locker::NewLocker,
            LockedVoterInstruction::NewEscrow => new_escrow::NewEscrow,
            LockedVoterInstruction::Lock => lock::Lock,
            LockedVoterInstruction::LockWithWhitelistEntry => lock_with_whitelist_entry::LockWithWhitelistEntry,
            LockedVoterInstruction::Exit => exit::Exit,
            LockedVoterInstruction::ActivateProposal => activate_proposal::ActivateProposal,
            LockedVoterInstruction::CastVote => cast_vote::CastVote,
            LockedVoterInstruction::SetVoteDelegate => set_vote_delegate::SetVoteDelegate,
            LockedVoterInstruction::SetLockerParams => set_locker_params::SetLockerParams,
            LockedVoterInstruction::ApproveProgramLockPrivilege => approve_program_lock_privilege::ApproveProgramLockPrivilege,
            LockedVoterInstruction::RevokeProgramLockPrivilege => revoke_program_lock_privilege::RevokeProgramLockPrivilege,
            LockedVoterInstruction::ApproveLockPrivilegeEvent => approve_lock_privilege_event::ApproveLockPrivilegeEvent,
            LockedVoterInstruction::ExitEscrowEvent => exit_escrow_event::ExitEscrowEvent,
            LockedVoterInstruction::LockEvent => lock_event::LockEvent,
            LockedVoterInstruction::NewEscrowEvent => new_escrow_event::NewEscrowEvent,
            LockedVoterInstruction::NewLockerEvent => new_locker_event::NewLockerEvent,
            LockedVoterInstruction::RevokeLockPrivilegeEvent => revoke_lock_privilege_event::RevokeLockPrivilegeEvent,
            LockedVoterInstruction::LockerSetParamsEvent => locker_set_params_event::LockerSetParamsEvent,
            LockedVoterInstruction::SetVoteDelegateEvent => set_vote_delegate_event::SetVoteDelegateEvent,
        )
    }
}
