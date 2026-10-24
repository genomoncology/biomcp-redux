use super::{launch::Environment, live, response_cases};
use biomcp_fetch::Observation;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{collections::BTreeMap, fs, path::Path};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("live-check/offline-planted-drift", || {
            check().map_err(|e| e.to_string().into())
        }),
        Trial::test("live-source-checks/offline-eligibility", || {
            plan().map_err(|e| e.to_string().into())
        }),
        Trial::test("live-source-checks/raw-wire-formats-and-retries", || {
            formats().map_err(|e| e.to_string().into())
        }),
    ]
}
fn check() -> Result<(), Box<dyn std::error::Error>> {
    let env = Environment::new()?;
    let before = env.root().join("before");
    let after = env.root().join("after");
    for (root, edited) in [(&before, false), (&after, true)] {
        response_cases::copy_capture("gene/main", root, |source, record| {
            if source != "mygene" {
                return;
            }
            let Some(exchanges) = record.get_mut("exchanges").and_then(Value::as_array_mut) else {
                return;
            };
            for exchange in exchanges {
                let Some(items) = exchange
                    .pointer_mut("/body/value")
                    .and_then(Value::as_array_mut)
                else {
                    continue;
                };
                for item in items.iter_mut().filter_map(Value::as_object_mut) {
                    item.insert("count".into(), json!(if edited { 0 } else { 1 }));
                    if edited && let Some(summary) = item.remove("summary") {
                        item.insert("renamed_summary".into(), summary);
                    }
                }
            }
        })?;
    }
    let before = observe(&env, &before, "before", &["get", "gene", "KRAS"])?;
    let after = observe(&env, &after, "after", &["get", "gene", "KRAS"])?;
    let mut changes = Vec::new();
    live::compare(&before, &after, &mut changes, &mut BTreeMap::new());
    if !changes
        .iter()
        .any(|s| s.contains("field removed: /*/summary"))
        || !changes
            .iter()
            .any(|s| s.contains("count dropped to zero: /*/count"))
    {
        return Err(format!("edited recording drift missing: {changes:?}").into());
    }
    // The same bounded observer also sees XML fields, bundle headers and attempt metadata.
    if !before.iter().any(|o| o.shape.header.is_some())
        || !before.iter().any(|o| o.bytes > 0 && o.status == Some(200))
    {
        return Err("missing bundle header or attempt metadata".into());
    }
    Ok(())
}
fn observe(
    env: &Environment,
    replay: &Path,
    name: &str,
    args: &[&str],
) -> Result<Vec<Observation>, Box<dyn std::error::Error>> {
    let report = env.root().join(format!("{name}.jsonl"));
    let mut args = args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
    args.extend([
        "--replay".into(),
        replay.display().to_string(),
        "--observe".into(),
        report.display().to_string(),
    ]);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    if !output.status.success() {
        return Err("observed replay failed".into());
    }
    let text = fs::read_to_string(report)?;
    if text.is_empty() || text.contains("https:") || text.contains("summary\":\"") {
        return Err("wire report empty or contains payload/URL".into());
    }
    Ok(text
        .lines()
        .map(serde_json::from_str)
        .collect::<Result<_, _>>()?)
}

fn plan() -> Result<(), Box<dyn std::error::Error>> {
    let environment = Environment::new()?;
    let output = environment
        .command(
            std::env::current_exe()?,
            &["--live-check".into(), "--list".into()],
        )
        .output()?;
    if !output.status.success() {
        return Err("offline eligibility listing failed".into());
    }
    let report: Value = serde_json::from_slice(&output.stdout)?;
    let cases = report
        .get("cases")
        .and_then(Value::as_array)
        .ok_or("case list missing")?;
    if !cases.iter().any(|c| {
        c.get("case") == Some(&json!("article/get/live-capture.yaml"))
            && c.get("eligible") == Some(&json!(true))
    }) || !cases.iter().any(|c| {
        c.get("case") == Some(&json!("gene/failure-429.yaml")) && c.get("skipped").is_some()
    }) {
        return Err("eligibility list omitted public capture or failure skip".into());
    }
    Ok(())
}

fn formats() -> Result<(), Box<dyn std::error::Error>> {
    let environment = Environment::new()?;
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies");
    let xml = observe(
        &environment,
        &root.join("article/get/live"),
        "xml",
        &["get", "article", "33123537"],
    )?;
    if !xml
        .iter()
        .any(|o| o.shape.fields.contains_key("PubmedArticleSet"))
    {
        return Err("raw XML names were discarded".into());
    }
    let retries = observe(
        &environment,
        &root.join("gene/synthetic-429"),
        "retries",
        &["get", "gene", "BRAF"],
    )?;
    if retries.iter().filter(|o| o.status == Some(429)).count() < 2 {
        return Err("retry HTTP attempts were discarded".into());
    }
    let report = environment.root().join("xml.jsonl");
    let original = fs::read(&report)?;
    let output = environment
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &[
                "get".into(),
                "gene".into(),
                "BRAF".into(),
                "--observe".into(),
                report.display().to_string(),
            ],
        )
        .output()?;
    if output.status.success() || fs::read(report)? != original {
        return Err("existing observation report was replaced".into());
    }
    Ok(())
}
