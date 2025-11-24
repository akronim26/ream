use anyhow::anyhow;
use ream_storage::cache::CachedDB;
use ream_chain_beacon::beacon_chain::BeaconChain;
use ream_light_client::finality_update::LightClientFinalityUpdate;

use crate::gossipsub::validate::result::ValidationResult;

pub async fn validate_light_client_finality_update(
    update: &LightClientFinalityUpdate,
    beacon_chain: &BeaconChain,
    cached_db: &CachedDB
) ->  anyhow::Result<ValidationResult> {
    let store = beacon_chain.store.lock().await;

    let head_root = store.get_head()?;
    let mut state: BeaconState = store
        .db
        .beacon_state_provider()
        .get(head_root)?
        .ok_or_else(|| anyhow!("No beacon state found for head root: {head_root}"))?;
}