#![no_std]

use codec::{Decode, Encode};
use gstd::{ActorId, String};
use scale_info::{TypeInfo};

#[derive(Debug, Decode, Encode, TypeInfo)]
pub enum ActorAction{
    SumbitVote{
        proposal_id: u128,
        vote: Vote,
    },
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum ActorEvent{
    SumbitVote{
        account: ActorId,
        proposal_id: u128,
        approved: bool,
    },
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitDao {
    pub approved_token_program_id: ActorId,
}

#[derive(Debug, Encode, Decode, Clone, TypeInfo)]
pub enum Vote {
    Yes,
    No,
}
