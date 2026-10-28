//! Feature adverse-event-search: real command policy precedes cache, replay and HTTP.
use super::{Check, args, both, environment, expression, setup};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("adverse-event-search/component-admission-before-io", || {
            admission().map_err(Into::into)
        }),
        transcript_trial("adverse-event-search/synthetic-invalid-before-io", || {
            invalid().map_err(Into::into)
        }),
    ]
}
fn admission() -> Check {
    let (env, stub) = setup(expression(), 5)?;
    let record = env.root().join("recording");
    let mut recorded = vec!["--record".into(), record.to_string_lossy().into()];
    recorded.extend(args());
    super::invoke(&env, recorded, 0)?;
    for switch in ["meddra", "restricted"] {
        let mut allowed = vec!["--disable-sources".into(), switch.into()];
        allowed.extend(args());
        both(&env, allowed, 0)?;
        for replay in [false, true] {
            let mut blocked = vec!["--disable-sources".into(), switch.into()];
            if replay {
                blocked.extend(["--replay".into(), record.to_string_lossy().into()]);
            }
            blocked.extend(args());
            blocked.extend(["--reaction".into(), "Synthetic term".into()]);
            let value = both(&env, blocked, 4)?;
            if value.pointer("/sources/0/reason") != Some(&json!("disabled"))
                || value.get("result").is_some()
            {
                return Err(format!("disabled terminology entered report path: {value}"));
            }
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled or warm production route performed HTTP".into());
    }
    // A reaction-filtered reply first enters the real cache and recorder, then a new policy blocks both.
    let (env, stub) = setup(
        &format!(
            "{} AND patient.reaction.reactionmeddrapt:\"Synthetic term\"",
            expression()
        ),
        5,
    )?;
    let record = env.root().join("filtered-recording");
    let mut recorded = vec!["--record".into(), record.to_string_lossy().into()];
    recorded.extend(args());
    recorded.extend(["--reaction".into(), "Synthetic term".into()]);
    super::invoke(&env, recorded, 0)?;
    for switch in ["meddra", "restricted", "faers"] {
        for replay in [false, true] {
            let mut blocked = vec!["--disable-sources".into(), switch.into()];
            if replay {
                blocked.extend(["--replay".into(), record.to_string_lossy().into()]);
            }
            blocked.extend(args());
            blocked.extend(["--reaction".into(), "Synthetic term".into()]);
            let value = both(&env, blocked, 4)?;
            if value.pointer("/sources/0/reason") != Some(&json!("disabled"))
                || value.get("result").is_some()
            {
                return Err("disabled policy restored cached or recorded filtered page".into());
            }
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled filtered production route performed HTTP".into());
    }
    Ok(())
}
fn invalid() -> Check {
    let stub = super::Stub::for_source("api.fda.gov", "application/json", |_| {
        Err("invalid input reached HTTP".into())
    })?;
    let env = environment(&stub)?;
    for words in [
        vec![""],
        vec!["x", "--received-from", "2024-02"],
        vec![
            "x",
            "--received-from",
            "2024-12-31",
            "--received-to",
            "2024-01-01",
        ],
        vec!["x", "--limit", "0"],
    ] {
        let request = ["search", "adverse-event"]
            .into_iter()
            .chain(words)
            .map(str::to_owned)
            .collect();
        let value = both(&env, request, 2)?;
        if value.get("error").and_then(Value::as_object).is_none() {
            return Err("invalid selectors did not return usage".into());
        }
    }
    if !stub.finish()?.is_empty() {
        return Err("invalid input performed HTTP".into());
    }
    Ok(())
}
