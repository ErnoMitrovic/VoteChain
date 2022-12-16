#![no_std]
pub mod io;
pub mod state;
pub mod messages;

use gstd::{prelude::*, ActorId};
use crate::{io::*};

#[derive(Debug, Default, Clone, Decode, Encode, TypeInfo)]
pub struct Proposal{
    pub proposer: ActorId,
    pub applicant: ActorId,
    pub shares_requested: u128,
    pub yes_votes: u128,
    pub no_votes: u128,
    pub quorum: u128,
    pub is_membership_proposal: bool,
    pub amount: u128,
    pub processed: bool,
    pub passed: bool,
    pub aborted: bool,
    pub token_tribute: u128,
    pub details: String,
    pub starting_period: u64,
    pub max_total_shares_at_yes_vote: u128,
    pub votes_by_member: BTreeMap<ActorId, Vote>,
}

#[derive(Debug, Default, Clone, Decode, Encode, TypeInfo)]
pub struct Member {
    pub delegate_key: ActorId,
    pub shares: u128,
    pub highest_index_yes_vote: u128,
}
