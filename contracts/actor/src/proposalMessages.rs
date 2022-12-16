use proposal_io::{ProposalAction, ProposalEvent};
use gstd::{msg, ActorId};

pub async fn transfer_from_tokens(token_id: &ActorId, from: &ActorId, to: &ActorId, amount: u128){
    let _transfer_response = msg::send_for_reply(
        *token_id, 
        ProposalAction::Transfer { from: *from, to: *to, amount },
        0).unwrap().await.expect("Error in transaction");
}

pub async fn balance(token_id: &ActorId, account: &ActorId){
    let balance_response: ProposalEvent = 
    msg::send_for_reply(*token_id, ProposalAction::Balance(*account), 0)
    .unwrap()
    .await;
}