//! Contract: offline inventory cannot hide a missing direct fact projection.
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};

pub(super) fn trial() -> Trial {
    Trial::test("parity/inventory-requires-every-direct-fact", || {
        check().map_err(Into::into)
    })
}
fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cases = env.root().join("cases/gene");
    fs::create_dir_all(&cases).map_err(|e| e.to_string())?;
    let root = super::launch::fixtures();
    let source: Value = serde_json::from_slice(
        &fs::read(root.join("cases/gene/braf.yaml")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let summary = source
        .pointer("/steps/0/expect/~1result~1items~10~1answer~1gene~1summary~1text")
        .ok_or("summary missing")?;
    let pointer = "/result/items/0/answer/gene/summary/text";
    let case = json!({"story":"A direct summary question", "steps":[{
        "run":["get","gene","BRAF"], "expect":{pointer:summary}}]});
    fs::write(
        cases.join("braf.yaml"),
        serde_json::to_vec(&case).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let mut plan: Value = serde_json::from_slice(
        &fs::read(root.join("parity/sample.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    for (name, projected) in [("complete", true), ("missing", false)] {
        if !projected {
            *plan.pointer_mut("/cases/0/facts").ok_or("facts missing")? = json!({});
        }
        let plan_path = env.root().join(format!("{name}.json"));
        fs::write(
            &plan_path,
            serde_json::to_vec(&plan).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        invoke(&env, &plan_path, projected)?;
    }
    Ok(())
}
fn invoke(env: &Environment, plan: &Path, complete: bool) -> Result<(), String> {
    let report = env.root().join(if complete {
        "complete-report.json"
    } else {
        "missing-report.json"
    });
    let args = vec![
        "--parity-inventory".into(),
        plan.display().to_string(),
        env.root().join("cases").display().to_string(),
        report.display().to_string(),
    ];
    #[expect(
        clippy::disallowed_methods,
        reason = "The proof invokes the actual public inventory entry point"
    )]
    let runner = std::env::current_exe().map_err(|e| e.to_string())?;
    let output = env
        .command(runner, &args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.success() != complete {
        return Err(format!(
            "inventory exit: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let value: Value = serde_json::from_slice(&fs::read(report).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    if value.get("assessment_gaps") != Some(&json!(usize::from(!complete)))
        || value.get("cases").and_then(Value::as_array).map(Vec::len) != Some(1)
    {
        return Err("missing projection escaped offline inventory".into());
    }
    Ok(())
}
