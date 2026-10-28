//! The shipped command owns key configuration and never includes its value in output.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::PathBuf;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("article/optional-scholar-key-is-outgoing-only", || {
            configured().map_err(Into::into)
        }),
        Trial::test(
            "article/scholar-key-crosses-only-the-outgoing-header",
            || outgoing().map_err(Into::into),
        ),
    ]
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
            command.env("S2_API_KEY", "placeholder-scholar-key");
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

fn outgoing() -> Result<(), String> {
    let quota = biomcp_adapters::semantic_scholar::SPEC.quota;
    if quota.interval(false) != std::time::Duration::from_secs(2)
        || quota.interval(true) != std::time::Duration::from_secs(1)
    {
        return Err("the source declaration changed its optional-key rate policy".into());
    }
    for configured in [false, true] {
        outgoing_with_key(configured)?;
    }
    Ok(())
}

fn outgoing_with_key(configured: bool) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = super::network_stub::Stub::for_source(
        "api.semanticscholar.org",
        "application/json",
        |_| Ok(r#"{"total":0,"data":[]}"#.into()),
    )?;
    let certificate = env.root().join("ca.pem");
    stub.retain(&env);
    std::fs::write(&certificate, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({
        "ca_bundle":certificate,"proxy":format!("http://{}",stub.address),
    }))
    .map_err(|e| e.to_string())?;
    if configured {
        super::key_registration::invoke(
            &env,
            &["key", "set", "semantic-scholar"],
            "placeholder-scholar-key\n",
            0,
        )?;
    }
    let capture = env.root().join("capture");
    let args = vec![
        "search".into(),
        "article".into(),
        "EGFR".into(),
        "--source".into(),
        "semantic-scholar".into(),
        "--record".into(),
        capture.to_string_lossy().into(),
    ];
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env(
            "S2_API_KEY",
            if configured {
                "synthetic-environment-shadowed"
            } else {
                "placeholder-scholar-key"
            },
        )
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    let requests = stub.finish()?;
    if !output.status.success()
        || !output.stderr.is_empty()
        || requests.len() != 1
        || !requests.first().is_some_and(|(_, request)| {
            request
                .to_ascii_lowercase()
                .contains("x-api-key: placeholder-scholar-key")
        })
    {
        return Err(format!(
            "the declared source did not send its configured key: {value}"
        ));
    }
    if value.to_string().contains("placeholder-scholar-key") {
        return Err("the key entered the envelope".into());
    }
    check_recordings(&capture)?;
    replay_keyless(&capture, args)
}

fn replay_keyless(capture: &std::path::Path, mut args: Vec<String>) -> Result<(), String> {
    args.truncate(5);
    args.extend(["--replay".into(), capture.to_string_lossy().into()]);
    let keyless = Environment::new().map_err(|e| e.to_string())?;
    let output = keyless
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || !output.stderr.is_empty()
        || value.pointer("/result/items").is_some()
    {
        return Err("credential presence changed the canonical request identity".into());
    }
    Ok(())
}

fn check_recordings(capture: &std::path::Path) -> Result<(), String> {
    for file in std::fs::read_dir(capture.join("semantic-scholar")).map_err(|e| e.to_string())? {
        let text = std::fs::read_to_string(file.map_err(|e| e.to_string())?.path())
            .map_err(|e| e.to_string())?;
        if text.contains("placeholder-scholar-key")
            || text.to_ascii_lowercase().contains("x-api-key")
        {
            return Err("the key entered a recording".into());
        }
    }
    Ok(())
}
