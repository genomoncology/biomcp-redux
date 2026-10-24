//! The shipped command owns key configuration and never includes its value in output.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::PathBuf;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "article/optional-scholar-key-is-outgoing-only",
        || configured().map_err(Into::into),
    )]
}
fn configured() -> Result<(), String> {
    let mut baseline = None;
    for mode in 0..3 {
        let env = Environment::new().map_err(|e| e.to_string())?;
        if mode == 1 {
            env.config(&json!({"keys":{"semantic_scholar_api_key":"placeholder-scholar-key"}}))
                .map_err(|e| e.to_string())?;
        }
        let replay = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/replies/article/semantic-scholar/live");
        let mut args = vec![
            "search".into(),
            "article".into(),
            "EGFR T790M osimertinib".into(),
            "--source".into(),
            "semantic-scholar".into(),
        ];
        args.extend(env.globals(Some(&replay), false));
        let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
        if mode == 2 {
            command.env("SEMANTIC_SCHOLAR_API_KEY", "placeholder-scholar-key");
        }
        let output = command.output().map_err(|e| e.to_string())?;
        if output.status.code() != Some(4)
            || !output.stderr.is_empty()
            || output
                .stdout
                .windows(b"placeholder-scholar-key".len())
                .any(|s| s == b"placeholder-scholar-key")
        {
            return Err(format!(
                "key mode {mode} leaked a key or changed the failure boundary"
            ));
        }
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        if value.pointer("/sources/0/reason") != Some(&json!("rate_limited")) {
            return Err("the optional key blocked the recorded source leg".into());
        }
        if let Some(baseline) = &baseline {
            if &value != baseline {
                return Err("configuration changed the public source response".into());
            }
        } else {
            baseline = Some(value);
        }
    }
    Ok(())
}
