//! Old trial snapshot owner keys must refuse before decoding incompatible payloads.
use super::{launch::Environment, response_cases::copy_capture, trial_get::command};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    vec![
        super::transcript::trial("trial-sites/sites/old-snapshot-owner-expires", || {
            old_owner().map_err(|e| e.to_string().into())
        }),
        super::transcript::trial("trial-sites/sites/page-past-end-is-empty", || {
            past_end().map_err(|e| e.to_string().into())
        }),
    ]
}
const LOCATIONS: &str = "/result/items/0/sections/locations/data";
/// A full page of 100 sites reports the total; an offset past the last site is an empty page.
fn past_end() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    copy_capture("trial/get/NCT03734029", &replay, |_, _| {})?;
    let args = ["get", "trial", "NCT03734029", "locations", "--limit", "100"].map(str::to_owned);
    let first = command(&env, &args, &env.globals(Some(&replay), false), 0)?;
    let page = first.pointer(LOCATIONS).ok_or("first locations page")?;
    let returned = page
        .get("items")
        .and_then(serde_json::Value::as_array)
        .map_or(0, Vec::len);
    let total = page.get("total").and_then(serde_json::Value::as_u64);
    if returned != 100 || total != Some(274) || page.get("continuation").is_none() {
        return Err(format!("first page: {returned} sites, total {total:?}").into());
    }
    let mut next = super::action(&first, "page")?;
    let at = next
        .iter()
        .position(|a| a == "--offset")
        .ok_or("page action has no offset")?;
    *next.get_mut(at + 1).ok_or("offset value")? = "400".into();
    let after = command(&env, &next, &env.globals(Some(&replay), false), 0)?;
    let page = after.pointer(LOCATIONS).ok_or("past-end locations page")?;
    let number = |name: &str| page.get(name).and_then(serde_json::Value::as_u64);
    if after.get("status").and_then(serde_json::Value::as_str) != Some("complete")
        || number("total") != Some(274)
        || number("offset") != Some(400)
        || page
            .get("items")
            .and_then(serde_json::Value::as_array)
            .is_some_and(|items| !items.is_empty())
        || page.get("continuation").is_some_and(|c| !c.is_null())
        || super::action(&after, "page").is_ok()
    {
        return Err(format!("past-end page: {after}").into());
    }
    Ok(())
}
fn old_owner() -> Check {
    for version in [1, 2, 3, 4] {
        old_version(version)?;
    }
    Ok(())
}
fn old_version(version: u32) -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    copy_capture("trial/get/NCT03734029", &replay, |_, _| {})?;
    let args = ["get", "trial", "NCT03734029", "locations"].map(str::to_owned);
    let first = command(&env, &args, &env.globals(Some(&replay), false), 0)?;
    let next = super::action(&first, "page")?;
    // Reclassify this row as an old owner's incompatible payload. The key lookup
    // must reject it before decode, even though the bytes are intentionally invalid.
    let database = env.root().join("case-cache").join("replay.sqlite");
    let fetch = biomcp_fetch::Fetch::open(biomcp_fetch::FetchConfig {
        cache: Some(database),
        cache_owners: vec![biomcp_query::store::OWNER],
        mode: biomcp_fetch::Mode::Offline,
        ..Default::default()
    })?;
    let db = fetch.cache_access()?.connect()?;
    use sha2::{Digest, Sha256};
    let old_key = if version == 4 {
        // This source snapshot predates the posted-results projection, not the source data.
        serde_json::to_vec(&("biomcp.source_snapshot/1", "trial", "NCT03734029"))?
    } else {
        Sha256::digest(format!("biomcp.trial_snapshot/{version}\0NCT03734029").as_bytes()).to_vec()
    };
    db.execute(
        "UPDATE trial_snapshot SET request_key=?1,payload='[\"old positional snapshot\"]'",
        [old_key],
    )?;
    let expired = command(&env, &next, &env.globals(None, true), 3)?;
    if expired.get("status").and_then(serde_json::Value::as_str) != Some("expired")
        || super::action(&expired, "restart").is_err()
    {
        return Err("old snapshot did not expire with restart".into());
    }
    Ok(())
}
