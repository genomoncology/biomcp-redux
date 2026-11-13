//! Feature disease-orphadata: a file under another licence is refused and the installed set stays.
use super::{configuration::cached, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};

const SYNTHETIC: &str = "disease/orphadata-synthetic";

pub(super) fn trials() -> Vec<Trial> {
    vec![super::transcript::trial(
        "disease-orphadata/other-licence-keeps-set",
        || other_licence().map_err(Into::into),
    )]
}

fn other_licence() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cache = env.root().join("orphadata-case-cache");
    let good = super::launch::fixtures().join("replies").join(SYNTHETIC);
    let first = run(&env, cached(&["sync", "orphadata"], &cache, Some(&good)))?;
    if first.pointer("/result/rows") != Some(&json!(2)) {
        return Err(format!("synthetic set did not install: {first}"));
    }
    let other = env.root().join("other-licence");
    super::response_cases::copy_capture(SYNTHETIC, &other, |_, record| {
        let prevalence = record
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| url.ends_with("en_product9_prev.xml"));
        if let Some(Value::String(body)) = record.pointer_mut("/exchanges/0/body/value")
            && prevalence
        {
            *body = body.replace(
                "<ShortIdentifier>CC-BY-4.0<",
                "<ShortIdentifier>CC-BY-NC-4.0<",
            );
        }
    })
    .map_err(|e| e.to_string())?;
    let refused = run(
        &env,
        cached(&["sync", "orphadata", "--fresh"], &cache, Some(&other)),
    )?;
    if refused.get("status") == Some(&json!("complete")) {
        return Err(format!("a CC-BY-NC-4.0 file was installed: {refused}"));
    }
    let kept = run(
        &env,
        cached(
            &[
                "get",
                "disease",
                "Orphanet:900001",
                "prevalence",
                "--source",
                "orphadata",
            ],
            &cache,
            Some(&other),
        ),
    )?;
    if kept.pointer("/result/items/0/prevalence/1/value") != Some(&json!("1.5")) {
        return Err(format!("the installed set did not survive the refusal: {kept}"));
    }
    Ok(())
}

/// Runs one command and returns its JSON reply whatever its exit, which the caller judges.
fn run(env: &Environment, args: Vec<String>) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    serde_json::from_slice(&output.stdout).map_err(|e| {
        format!(
            "{e}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })
}
