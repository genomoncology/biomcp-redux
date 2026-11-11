//! Many searches at once stay complete while a recorded name source is too slow to answer.
use super::{
    configuration::{cached, invoke},
    launch::Environment,
};
use serde_json::{Value, json};
use std::path::Path;

/// As many searches as the agent bench ran side by side.
const AT_ONCE: usize = 25;
const TERM: &str = "Canonical therapy";

pub(super) fn trials() -> Vec<libtest_mimic::Trial> {
    vec![super::transcript::trial(
        "drug-name-search/slow-lookup-stays-complete-at-once",
        || at_once().map_err(Into::into),
    )]
}

fn at_once() -> Result<(), String> {
    let recordings = super::launch::fixtures().join("replies/search/drug-names-budget");
    std::thread::scope(|scope| {
        let searches: Vec<_> = (0..AT_ONCE)
            .map(|index| {
                let recordings = recordings.as_path();
                let entity = if index % 5 == 4 { "article" } else { "trial" };
                scope.spawn(move || search(entity, recordings))
            })
            .collect();
        searches
            .into_iter()
            .try_for_each(|search| search.join().map_err(|_| "a search panicked".to_owned())?)
    })
}

/// One process with its own cache, as each bench agent had.
fn search(entity: &str, recordings: &Path) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    env.config(&json!({"command_timeout_seconds":2,"source_data_max_age_days":1_000_000}))
        .map_err(|e| e.to_string())?;
    let args = cached(
        &["search", entity, TERM],
        &env.root().join("case"),
        Some(recordings),
    );
    let reply = invoke(&env, args, 0)?;
    let unwidened = reply
        .pointer("/sources/0/notices/0/names")
        .is_some_and(|names| names == &json!([TERM]));
    if reply.get("status") != Some(&json!("complete"))
        || reply.pointer("/sources/0/state") != Some(&json!("skipped"))
        || !unwidened
        || reply.pointer("/page/total") != Some(&Value::from(1))
    {
        return Err(format!(
            "search {entity} was not complete and unwidened: {reply}"
        ));
    }
    Ok(())
}
