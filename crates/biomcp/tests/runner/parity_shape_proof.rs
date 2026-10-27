//! Contract: public inventory binds values without borrowing a different case's captures.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trial() -> Trial {
    Trial::test("parity/shape-current-question-and-capture", || {
        check().map_err(Into::into)
    })
}
fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let fixtures = setup(&env)?;
    let cases = fixtures.join("cases");
    let selection = json!({"case":"shape.yaml","step":0,
        "baseline_command":["get","gene","{symbol}"],
        "spelling_table":"sdlc/planning/slices/gene.md",
        "release_source":["src/entities/gene.rs::Gene"],"baseline_globals":["--json"],
        "source_free":true,"recordings":[],"facts":{"/result/symbol":"/symbol"}});
    let plan = json!({"baseline_release":"0.9.3","cases":[],"shapes":[{
        "command":["get","gene","{symbol}"],"exit":0,"config":null,
        "declaration":selection}]});
    for (ordinal, command, config, complete) in [
        (0, json!(["get", "gene", "BRAF"]), Value::Null, true),
        (1, json!(["get", "gene", "TP53"]), Value::Null, true),
        (
            2,
            json!(["get", "gene", "BRAF", "civic"]),
            Value::Null,
            false,
        ),
        (3, json!(["get", "gene", "--fresh"]), Value::Null, false),
        (
            4,
            json!(["get", "gene", "BRAF"]),
            json!({"offline":true}),
            false,
        ),
    ] {
        let case = json!({"story":"Bind the current gene question", "config":config,
            "steps":[{"run":command,"expect":{"/result/symbol":"BRAF"}},
                     {"run":command,"expect":{"/result/symbol":"BRAF"}}]});
        let path = cases.join("shape.yaml");
        fs::write(path, serde_json::to_vec(&case).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let path = env.root().join(format!("plan-{ordinal}.json"));
        fs::write(&path, serde_json::to_vec(&plan).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let report = env.root().join(format!("report-{ordinal}.json"));
        #[expect(
            clippy::disallowed_methods,
            reason = "Invoke the public inventory entry point"
        )]
        let runner = std::env::current_exe().map_err(|e| e.to_string())?;
        let output = env
            .command(
                runner,
                &[
                    "--parity-inventory".into(),
                    path.display().to_string(),
                    cases.display().to_string(),
                    report.display().to_string(),
                ],
            )
            .env("BIOMCP_TEST_FIXTURES", &fixtures)
            .output()
            .map_err(|e| e.to_string())?;
        if output.status.success() != complete {
            return Err(format!(
                "shape binding {ordinal}: {}",
                String::from_utf8_lossy(&output.stderr)
            ));
        }
        let value: Value = serde_json::from_slice(&fs::read(report).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if value.pointer("/cases/0/declaration_reuse/reused_steps")
            != Some(&json!(if complete { 2 } else { 0 }))
        {
            return Err("shape changed question, configuration or command layout".into());
        }
    }
    Ok(())
}

fn setup(env: &Environment) -> Result<std::path::PathBuf, String> {
    let fixtures = env.root().join("fixtures");
    let cases = fixtures.join("cases");
    fs::create_dir_all(&cases).map_err(|e| e.to_string())?;
    fs::create_dir(fixtures.join("parity")).map_err(|e| e.to_string())?;
    for name in ["released-sources.json", "released-endpoints.json"] {
        fs::copy(
            super::launch::fixtures().join("parity").join(name),
            fixtures.join("parity").join(name),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(fixtures)
}
