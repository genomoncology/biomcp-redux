//! Feature list-paging: a list walked by offset alone, each call with a new store, gives the
//! rows its big pages give, in order, with no empty page before the end, and one page past
//! the end is empty with the total (1061).
use super::{launch::Environment, trial_get::command};
use libtest_mimic::Trial;
use serde::Deserialize;
use serde_json::Value;

#[derive(Deserialize)]
struct Walk {
    name: String,
    args: Vec<String>,
    replay: String,
    small: u32,
    big: u32,
    items: String,
    page: String,
}

pub(super) fn trials() -> Vec<Trial> {
    let walks: Vec<Walk> =
        serde_json::from_str(include_str!("offset_walks.json")).unwrap_or_default();
    walks
        .into_iter()
        .map(|walk| {
            super::transcript_trial(
                format!("list-paging/offset-walk/{}", walk.name),
                move || check(&walk).map_err(Into::into),
            )
        })
        .collect()
}

fn check(walk: &Walk) -> Result<(), String> {
    let owner = Environment::new().map_err(|e| e.to_string())?;
    let replay = owner.root().join("recordings");
    super::response_cases::copy_capture(&walk.replay, &replay, |_, _| {})
        .map_err(|e| e.to_string())?;
    let big = rows(walk, &replay, walk.big)?;
    let small = rows(walk, &replay, walk.small)?;
    if big.len() <= walk.small as usize || big != small {
        return Err(format!(
            "{}: {} rows by pages of {} differ from {} rows by pages of {}",
            walk.name,
            small.len(),
            walk.small,
            big.len(),
            walk.big
        ));
    }
    Ok(())
}

/// Walk the list by offset alone, each call in a new store, then ask one page past the end.
fn rows(walk: &Walk, replay: &std::path::Path, limit: u32) -> Result<Vec<Value>, String> {
    let mut rows = Vec::new();
    let mut offset = 0;
    loop {
        let (page, items) = call(walk, replay, limit, offset)?;
        if page.get("offset").and_then(Value::as_u64) != Some(u64::from(offset))
            || page.get("returned").and_then(Value::as_u64) != Some(items.len() as u64)
        {
            return Err(format!(
                "{}: page at {offset} misstates itself: {page}",
                walk.name
            ));
        }
        if items.is_empty() {
            let total = page.get("total").and_then(Value::as_u64);
            if total.is_none_or(|total| total > u64::from(offset)) {
                return Err(format!(
                    "{}: empty page at {offset} before the end: {page}",
                    walk.name
                ));
            }
            let (past, items) = call(walk, replay, limit, offset + 3 * limit)?;
            if !items.is_empty() || past.get("total").and_then(Value::as_u64) != total {
                return Err(format!(
                    "{}: a page past the end is not empty with the total: {past}",
                    walk.name
                ));
            }
            return Ok(rows);
        }
        rows.extend(items);
        offset += limit;
    }
}

fn call(
    walk: &Walk,
    replay: &std::path::Path,
    limit: u32,
    offset: u32,
) -> Result<(Value, Vec<Value>), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let mut args = walk.args.clone();
    args.extend([
        "--limit".into(),
        limit.to_string(),
        "--offset".into(),
        offset.to_string(),
    ]);
    let value = command(&env, &args, &env.globals(Some(replay), false), 0)?;
    let page = value.pointer(&walk.page).cloned().unwrap_or_default();
    let items = value
        .pointer(&walk.items)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    Ok((page, items))
}
