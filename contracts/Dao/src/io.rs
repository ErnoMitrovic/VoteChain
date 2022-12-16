use gstd::{prelude::*, ActorId};

#[derive(Debug, Decode, Encode, TypeInfo, Clone)]
pub enum DaoAction {
    AddToWhiteList(
        ActorId,
    ),
    // Just the designated of the white list 
    SubmitMembershipProposal {
        applicant: ActorId,
        token_tribute: u128,
        shares_requested: u128,
        quorum: u128,
        details: String,
    },
    // Just the designated of the white list 
    SubmitFundProposal {
        applicant: ActorId,
        amount: u128,
        quorum: u128,
        details: String,
    },
    ProcessProposal (u128),
    SubmitVote{
        proposal_id: u128,
        vote: Vote,
    },
    RageQuit(u128),
    Abort(u128),
    // Just by admin
    UpdateDelegateKey(ActorId),
    // Just by admin
    SetAdmin(ActorId),
    // In case of failure
    Continue(u128),
}

#[derive(Debug, Encode, Decode, Clone, TypeInfo)]
pub enum Vote{
    Yes, No,
}

#[derive(Debug, Encode, Decode, TypeInfo)]
pub enum DaoEvent{
    MemberAddedToWhiteList(ActorId),
    SubmitMembershipProposal{
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        token_tribute: u128,
    },
    SubmitFundProposal{
        proposer: ActorId,
        applicant: ActorId,
        proposal_id: u128,
        amount: u128,
    },
    SubmitVote{
        origin: ActorId,
        proposal_id: u128,
        vote: Vote,
    },
    ProcessProposal{
        proposal_id: u128,
        approved: bool,
    },
    RageQuit{
        member: ActorId,
        amount: u128,
    },
    Abort(u128),
    AdminUpdated(ActorId),
    DelegateKeyUpdated{
        member: ActorId,
        delegate: ActorId,
    },
    TransactionFailed(u64),
}

#[derive(Debug, Decode, Encode, TypeInfo)]
pub struct InitDao {
    pub admin: ActorId,
    pub approved_token_program_id: ActorId,
    pub period_duration: u64,
    pub voting_period_length: u64,
    pub grace_period_length: u64,
    pub dilution_bound: u8,
    pub abort_window: u64,
}
