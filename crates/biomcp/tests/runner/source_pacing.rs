//! Operator pacing configuration through the shipped process boundary.
use super::{configuration::{cached, invoke, root}, launch::Environment};
use libtest_mimic::Trial;
use serde_json::json;

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript::trial("source-pacing/settings-and-ceilings", || {
        check().map_err(Into::into)
    })]
}
fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let case = env.root().join("case");
    let args = cached(&["get", "gene", "BRAF"], &case, Some(&root()));
    let original = invoke(&env, args.clone(), 0)?;
    for rate in [json!({"requests":1,"seconds":2}), json!({"requests":1,"seconds":1})] {
        env.config(&json!({"source_pacing":{"hgnc":{"rate":rate,"in_flight":1}}}))
            .map_err(|e| e.to_string())?;
        if invoke(&env, args.clone(), 0)? != original {
            return Err("pacing changed the reply".into());
        }
    }
    for (source, value) in [
        ("hgnc", json!({"rate":{"requests":2,"seconds":1}})),
        ("hgnc", json!({"rate":{"requests":0,"seconds":1}})),
        ("hgnc", json!({"rate":{"requests":1,"seconds":0}})),
        ("hgnc", json!({"in_flight":0})),
        ("hgnc", json!({"in_flight":2})),
        ("typo", json!({"in_flight":1})),
    ] {
        env.config(&json!({"source_pacing":{source:value}})).map_err(|e| e.to_string())?;
        let rejected = invoke(&env, args.clone(), 5)?;
        if !rejected.to_string().contains(&format!("source_pacing.{source}")) {
            return Err(format!("pacing error did not name {source}: {rejected}"));
        }
    }
    env.config(&json!({"source_pacing":{"hgnc":{"rate":{"requests":1,"seconds":2},"in_flight":1}}}))
        .map_err(|e| e.to_string())?;
    let mut flags = args;
    flags.extend(["--source_pacing".into(), json!({"hgnc":{"rate":{"requests":1,"seconds":1}}}).to_string()]);
    if invoke(&env, flags, 0)? != original {
        return Err("pacing flag changed the reply".into());
    }
    Ok(())
}
