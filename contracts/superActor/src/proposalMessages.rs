use proposal_io::*;
use gstd::{msg, ActorId};

pub async fn transfer_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128){
    let transfer_response: ProposalEvent = msg::send_and_wait_for_reply(
        *token_id,
        ProposalAction::Transfer{
            from: *from,
            to: *to,
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in transfer");
}

pub async fn approve_tokens(token_id: &ActorId, to: &ActorId, amount: u128){
    let approve_response: ProposalEvent = msg::send_and_wait_for_reply(
        *token_id, 
        ProposalAction::Approve {
            to: *to, 
            amount,
        },
        0,
    )
    .unwrap()
    .await
    .expect("Error in approve tokens");
}
