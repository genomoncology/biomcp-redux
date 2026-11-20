//! Every refusal of a word after a get command's ID offers a next step that passes its checks:
//! help runs, and a corrected call is never itself refused as usage. Offline, a corrected
//! call cannot read its sources, so it ends failed rather than complete.
use super::launch::{Environment, fixtures};
use serde_json::Value;

type Check<T = ()> = Result<T, String>;

pub(super) fn trial() -> libtest_mimic::Trial {
    super::transcript::trial("query-validation/unknown-word-next-steps-pass", || {
        check().map_err(Into::into)
    })
}

fn run(env: &Environment, args: &[String]) -> Check<(Option<i32>, Value)> {
    let mut words = vec!["--offline".to_owned()];
    words.extend_from_slice(args);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &words)
        .output()
        .map_err(|error| error.to_string())?;
    let reply = serde_json::from_slice(&output.stdout)
        .map_err(|error| format!("{args:?} did not reply with JSON: {error}"))?;
    Ok((output.status.code(), reply))
}

fn check() -> Check {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let path = fixtures().join("cases/grammar/unknown-words-are-refused.yaml");
    let case: Value =
        serde_json::from_str(&std::fs::read_to_string(&path).map_err(|e| e.to_string())?)
            .map_err(|error| error.to_string())?;
    let refusals = case
        .get("steps")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter(|step| step.get("exit") == Some(&Value::from(2)))
        .filter_map(|step| serde_json::from_value::<Vec<String>>(step.get("run")?.clone()).ok());
    let mut checked = 0;
    for args in refusals {
        let (_, reply) = run(&env, &args)?;
        let next = reply
            .pointer("/next/0/args")
            .cloned()
            .and_then(|args| serde_json::from_value::<Vec<String>>(args).ok())
            .ok_or_else(|| format!("{args:?}: refusal has no next step: {reply}"))?;
        let (code, followed) = run(&env, &next)?;
        let usage = code == Some(2) || followed.get("status") == Some(&Value::from("usage"));
        let help_failed = next.first().is_some_and(|word| word == "help") && code != Some(0);
        if usage || help_failed {
            return Err(format!(
                "{args:?}: next {next:?} did not pass (exit {code:?}): {followed}"
            ));
        }
        checked += 1;
    }
    if checked < 20 {
        return Err(format!(
            "only {checked} refusals followed; the case lost its steps"
        ));
    }
    Ok(())
}
