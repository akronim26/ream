use anyhow::{Ok, anyhow};
use ream_chain_beacon::beacon_chain::BeaconChain;
use ream_consensus_beacon::electra::beacon_state::BeaconState;
use ream_consensus_misc::constants::beacon::SYNC_COMMITTEE_SIZE;
use ream_light_client::finality_update::LightClientFinalityUpdate;
use ream_storage::{cache::CachedDB, tables::table::REDBTable};

use crate::gossipsub::validate::result::ValidationResult;

pub async fn validate_light_client_finality_update(
    update: &LightClientFinalityUpdate,
    beacon_chain: &BeaconChain,
    cached_db: &CachedDB,
) -> anyhow::Result<ValidationResult> {
    let store = beacon_chain.store.lock().await;

    let head_root = store.get_head()?;
    let mut state: BeaconState = store
        .db
        .state_provider()
        .get(head_root)?
        .ok_or_else(|| anyhow!("No beacon state found for head root: {head_root}"))?;

    // [IGNORE] The finalized header is greater than that of all previously forwarded finality updates
    // or it matches the highest previously forwarded slot and also has a supermajority participation
    // while previously forwarded slot did not indicate supermajority
    let new_slot = update.finalized_header.beacon.slot;
    let participation_count = update.sync_aggregate.sync_committee_bits.iter().filter(|b| *b).count() as u64;

    let has_supermajority = participation_count * 3 > SYNC_COMMITTEE_SIZE * 2;

    let highest_forwarded_update = cached_db.last_forwarded_finality_update_slot.write().await;
    let highest_forwarded_slot;
    let highest_forwarded_Slot_has_supermajority
    if let 
    
}
