#![no_std]

use codec::{Decode, Encode};
use gstd::{ActorId, String};
use scale_info::TypeInfo;

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SuperActorAction{
    Deposit{
        amount: u128,
    },
    SubmitFundingProposal{
        applicant: ActorId,
        amount: u128,
        quorum: u128,
        details: String, 
    },
    ProcessProposal{
        proposal_id: u128,
    },
    RageQuit{
        amount: u128,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum SuperActorEvent{
    Deposit {
        member: ActorId,
        share: u128,
    },
    SubmitFundingProposal{
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    RageQuit{
        member: u128,
        amount: u128,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitSuperActor {
    pub approved_token_program_id: ActorId,
    pub voting_period_length: u64,
    pub period_duration: u64,
    pub grace_period_length: u64,
}