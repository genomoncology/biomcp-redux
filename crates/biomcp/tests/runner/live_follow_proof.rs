//! Exercise diagnostic dependency planning through the runner, without live sources.
use super::launch::{Environment, fixtures};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
        "live-source-checks/follow-predecessors",
        || check().map_err(|e| e.to_string().into()),
    )]
}
fn check() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let root = env.root().join("fixtures");
    fs::create_dir_all(root.join("cases"))?;
    let witnessed = fixtures().join("cases/article/search/checked-request.yaml");
    fs::copy(&witnessed, root.join("cases/checked.yaml"))?;
    fs::write(
        root.join("cases/excluded.yaml"),
        serde_json::to_vec(&json!({
            "live_check": "public", "story": "An offline source predecessor stays excluded.",
            "replay": "article/search/live", "steps": [
                {"run": ["search", "article", "AKT1 E17K"], "offline": true},
                {"follow": "page"}, {"follow": "page"}
            ]
        }))?,
    )?;
    fs::write(
        root.join("cases/eligible.yaml"),
        serde_json::to_vec(&json!({
            "live_check": "public", "story": "An eligible source permits its follow.",
            "replay": "article/search/live", "steps": [
                {"run": ["search", "article", "AKT1 E17K"]}, {"follow": "page"}
            ]
        }))?,
    )?;
    let plan = diagnostic(&env, &root, true)?;
    let cases = plan["cases"].as_array().ok_or("missing inventory")?;
    for name in ["checked.yaml", "excluded.yaml"] {
        if !cases.iter().any(|case| {
            case["case"] == name && case["skipped"] == "follow predecessor is ineligible"
        }) {
            return Err(format!("dependent case not excluded: {plan}").into());
        }
    }
    if cases.len() != 3
        || !cases
            .iter()
            .any(|case| case["case"] == "eligible.yaml" && case["eligible"] == true)
    {
        return Err(format!("eligible follow inventory changed: {plan}").into());
    }
    // Remove the live case before execution: this proof never contacts a source.
    fs::remove_file(root.join("cases/eligible.yaml"))?;
    let report = diagnostic(&env, &root, false)?;
    if report["discovered"] != 2
        || report["checked"] != 0
        || report["error_cases"] != 0
        || report["skipped"]["follow predecessor is ineligible"] != 2
    {
        return Err(format!("skipped diagnostic counts changed: {report}").into());
    }
    // Ordinary replay still executes the local error and follows its real restart.
    super::run_case(&witnessed, &super::schema::Registry::embedded()?)?;
    Ok(())
}
fn diagnostic(
    env: &Environment,
    root: &std::path::Path,
    list: bool,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut args = vec!["--live-check".into()];
    if list {
        args.push("--list".into());
    }
    let output = env
        .command(std::env::current_exe()?, &args)
        .env("BIOMCP_TEST_FIXTURES", root)
        .output()?;
    if !output.status.success() {
        return Err(format!(
            "diagnostic failed: {}",
            String::from_utf8_lossy(&output.stderr)
        )
        .into());
    }
    Ok(serde_json::from_slice(&output.stdout)?)
}
