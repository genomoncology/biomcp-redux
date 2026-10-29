//! Contract: public inventory binds values without borrowing a different case's captures.
use super::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trial() -> Trial {
    transcript_trial("parity/shape-current-question-and-capture", || {
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
    capture_guards(&env, &fixtures, selection)?;
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
    let provenance = std::path::Path::new("sdlc/planning/slices/gene.md");
    let copied = fixtures.join("support").join(provenance);
    fs::create_dir_all(copied.parent().ok_or("provenance parent missing")?)
        .map_err(|e| e.to_string())?;
    fs::copy(super::launch::support().join(provenance), copied).map_err(|e| e.to_string())?;
    Ok(fixtures)
}

fn capture_guards(
    env: &Environment,
    fixtures: &std::path::Path,
    mut selection: Value,
) -> Result<(), String> {
    *selection
        .get_mut("source_free")
        .ok_or("source_free missing")? = json!(false);
    *selection
        .get_mut("recordings")
        .ok_or("recordings missing")? = json!([{"source":"mygene-info", "endpoint_env":"BIOMCP_MYGENE_BASE",
        "host":"mygene.info", "request_prefix":"GET /v3/query?", "request_contains":"",
        "request_query":{"q":"symbol:\"{symbol}\"","species":"human"}, "capture":""}]);
    let plan = json!({"baseline_release":"0.9.3","cases":[],"shapes":[{
        "command":["get","gene","{symbol}"],"exit":0,"config":null,"declaration":selection}]});
    let case = json!({"story":"Keep source request identity in the current replay", "replay":"current", "steps":[
        {"run":["get","gene","TP53"],"expect":{"/result/symbol":"TP53"}},
        {"run":["get","gene","TP53"],"expect":{"/result/symbol":"TP53"}}]});
    let path = fixtures.join("cases/shape.yaml");
    fs::write(path, serde_json::to_vec(&case).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let capture = json!({"source":"mygene","method":"GET",
        "url":"https://mygene.info/v3/query?q=symbol%3A%22TP53%22&species=human","exchanges":[]});
    for replay in ["current", "other-question"] {
        let directory = fixtures.join("replies").join(replay).join("mygene");
        fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
        fs::write(
            directory.join("one.json"),
            serde_json::to_vec(&capture).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    for (label, capture, complete) in capture_cases(&capture) {
        fs::write(
            fixtures.join("replies/current/mygene/one.json"),
            serde_json::to_vec(&capture).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        captured_inventory(env, fixtures, &plan, label, complete)?;
    }
    extra_guards(env, fixtures, &plan, &case, &capture)?;
    fs::write(fixtures.join("replies/current/mygene/one.json"), b"{broken")
        .map_err(|e| e.to_string())?;
    captured_inventory(env, fixtures, &plan, "malformed", false)?;
    Ok(())
}
fn captured_inventory(
    env: &Environment,
    fixtures: &std::path::Path,
    plan: &Value,
    label: &str,
    complete: bool,
) -> Result<(), String> {
    let path = env.root().join(format!("capture-{label}.json"));
    fs::write(&path, serde_json::to_vec(plan).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let report = env.root().join(format!("capture-{label}-report.json"));
    #[expect(
        clippy::disallowed_methods,
        reason = "Invoke the actual public inventory entry point"
    )]
    let runner = std::env::current_exe().map_err(|e| e.to_string())?;
    let output = env
        .command(
            runner,
            &[
                "--parity-inventory".into(),
                path.display().to_string(),
                fixtures.join("cases").display().to_string(),
                report.display().to_string(),
            ],
        )
        .env("BIOMCP_TEST_FIXTURES", fixtures)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() != complete {
        return Err(format!(
            "capture binding {label}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let value: Value = serde_json::from_slice(&fs::read(report).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if value.pointer("/cases/0/declaration_reuse/reused_steps")
        != Some(&json!(if complete { 2 } else { 0 }))
    {
        return Err(format!("capture identity escaped: {label}"));
    }
    Ok(())
}

fn extra_guards(
    env: &Environment,
    fixtures: &std::path::Path,
    plan: &Value,
    case: &Value,
    capture: &Value,
) -> Result<(), String> {
    let current = fixtures.join("replies/current/mygene");
    for name in ["one.json", "two.json"] {
        fs::write(
            current.join(name),
            serde_json::to_vec(capture).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    captured_inventory(env, fixtures, plan, "ambiguous", false)?;
    fs::remove_file(current.join("two.json")).map_err(|e| e.to_string())?;
    let mut fixed = plan.clone();
    *fixed
        .pointer_mut("/shapes/0/declaration/baseline_command")
        .ok_or("native command missing")? = json!(["get", "gene", "BRAF"]);
    captured_inventory(env, fixtures, &fixed, "unbound-native-question", false)?;
    let mut disabled = plan.clone();
    *disabled
        .pointer_mut("/shapes/0/declaration/baseline_command")
        .ok_or("native command missing")? = json!([
        "get",
        "gene",
        "{symbol}",
        "--disable-sources",
        "mygene-info"
    ]);
    captured_inventory(env, fixtures, &disabled, "disabled", false)?;
    let mut path_plan = plan.clone();
    *path_plan
        .pointer_mut("/shapes/0/declaration/recordings/0/request_prefix")
        .ok_or("route missing")? = json!("GET /v3/gene/{symbol}");
    for (ordinal, symbol) in ["two words", "a/b", "a#b", "a?b"].iter().enumerate() {
        let mut path_case = case.clone();
        for step in path_case
            .get_mut("steps")
            .ok_or("steps missing")?
            .as_array_mut()
            .ok_or("steps missing")?
        {
            *step.pointer_mut("/run/2").ok_or("symbol missing")? = json!(symbol);
        }
        fs::write(
            fixtures.join("cases/shape.yaml"),
            serde_json::to_vec(&path_case).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        captured_inventory(env, fixtures, &path_plan, &format!("path-{ordinal}"), false)?;
    }
    fs::write(
        fixtures.join("cases/shape.yaml"),
        serde_json::to_vec(case).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn capture_cases(capture: &Value) -> Vec<(&'static str, Value, bool)> {
    vec![
        ("native", capture.clone(), true),
        (
            "different-window",
            json!({"source":"mygene","method":"GET",
            "url":"https://mygene.info/v3/query?q=symbol%3A%22TP53%22&species=human&retmax=9999"}),
            false,
        ),
        (
            "wrong-question",
            json!({"source":"mygene","method":"GET",
            "url":"https://mygene.info/v3/query?q=symbol%3A%22BRAF%22&species=human"}),
            false,
        ),
        (
            "wrong-host",
            json!({"source":"mygene","method":"GET",
            "url":"https://foreign.invalid/v3/query?q=symbol%3A%22TP53%22&species=human"}),
            false,
        ),
        (
            "wrong-method",
            json!({"source":"mygene","method":"POST","url":capture.get("url")}),
            false,
        ),
        (
            "wrong-source",
            json!({"source":"pubmed","method":"GET","url":capture.get("url")}),
            false,
        ),
        (
            "wrong-type",
            json!({"source":"mygene","method":"GET","url":17}),
            false,
        ),
        (
            "duplicate-selector",
            json!({"source":"mygene","method":"GET",
            "url":"https://mygene.info/v3/query?q=symbol%3A%22TP53%22&q=other&species=human"}),
            false,
        ),
    ]
}
