use anyhow::Ok;
use ream_consensus_misc::constants::beacon::SYNC_COMMITTEE_SIZE;
use ream_light_client::finality_update::LightClientFinalityUpdate;
use ream_storage::cache::CachedDB;

use crate::gossipsub::validate::result::ValidationResult;

pub async fn validate_light_client_finality_update(
    update: &LightClientFinalityUpdate,
    cached_db: &CachedDB,
) -> anyhow::Result<ValidationResult> {
    // [IGNORE] The finalized header is greater than that of all previously forwarded finality updates
    // or it matches the highest previously forwarded slot and also has a supermajority participation
    // while previously forwarded slot did not indicate supermajority
    let new_slot = update.finalized_header.beacon.slot;
    let participation_count = update
        .sync_aggregate
        .sync_committee_bits
        .iter()
        .filter(|b| *b)
        .count() as u64;

    let has_supermajority = participation_count * 3 > SYNC_COMMITTEE_SIZE * 2;

    let mut last_forwarded_update = cached_db.last_forwarded_finality_update_slot.write().await;
    if let Some(ref mut info) = *last_forwarded_update {
        if new_slot < info.0 {
            return Ok(ValidationResult::Ignore(
                "Finality update slot is less than the last forwarded update slot".into(),
            ));
        }

        if new_slot == info.0 {
            if info.1 == true {
                return Ok(ValidationResult::Ignore(
                    "Finality update already gossiped".into(),
                ));
            } else {
                if !has_supermajority {
                    return Ok(ValidationResult::Ignore(
                        "Worse than previous update".into(),
                    ));
                }
            }
        }
    }
    *last_forwarded_update = Some((new_slot, has_supermajority));

    Ok(ValidationResult::Accept)
}
