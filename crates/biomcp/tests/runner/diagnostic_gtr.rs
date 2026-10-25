//! Feature diagnostic-gtr: local reads require an explicit complete synchronization.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "diagnostic-gtr/read-does-not-synchronize",
        || missing().map_err(Into::into),
    )]
}
fn missing() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("empty");
    std::fs::create_dir(&replay).map_err(|e| e.to_string())?;
    for tail in [
        vec!["search", "diagnostic", "--source", "gtr", "--gene", "BRCA1"],
        vec!["get", "diagnostic", "GTR000006692.3", "--source", "gtr"],
    ] {
        let mut args = env.globals(Some(&replay), false);
        args.extend(tail.into_iter().map(str::to_owned));
        let value = invoke(&env, args, 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("syncing"))
            || value.pointer("/next/0/args") != Some(&json!(["sync", "gtr"]))
        {
            return Err(format!(
                "read did not request explicit synchronization: {value}"
            ));
        }
    }
    Ok(())
}
