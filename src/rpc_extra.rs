use crate::config::Whitelist;
use solana_client::{nonblocking::rpc_client::RpcClient, rpc_response::RpcVoteAccountStatus};
use solana_sdk::clock::Epoch;


pub async fn get_first_block(client: &RpcClient, epoch: Epoch) -> anyhow::Result<Option<u64>> {
    let epoch_schedule = client.get_epoch_schedule().await?;
    let first_slot = epoch_schedule.get_first_slot_in_epoch(epoch);

    // First block in `epoch`.
    let first_block = client.get_blocks_with_limit(first_slot, 1).await?.get(0).cloned();

    if let Some(block) = first_block {
        Ok(Some(block))
    } else {
        Ok(None)
    }
}
/// Maps vote pubkeys to node pubkeys based on the information provided in `vote_accounts`.
pub fn node_pubkeys(vote_pubkeys: &Whitelist, vote_accounts: &RpcVoteAccountStatus) -> Whitelist {
    if vote_pubkeys.0.is_empty() {
        Whitelist::default()
    } else {
        Whitelist(
            vote_accounts
                .current
                .iter()
                .chain(vote_accounts.delinquent.iter())
                .filter(|account| vote_pubkeys.contains(&account.vote_pubkey))
                .map(|account| account.node_pubkey.clone())
                .collect(),
        )
    }
}
