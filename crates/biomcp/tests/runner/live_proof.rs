use super::{launch::Environment, live};
use libtest_mimic::Trial;
use std::fs;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("live-check/offline-planted-drift", || {
        check().map_err(Into::into)
    })]
}
fn check() -> Result<(), String> {
    let original = serde_json::json!({"results": [{"title": "public example"}], "count": 1});
    let edited = serde_json::json!({"renamed": [], "count": 0});
    let before = biomcp_fetch::Shape::inspect(
        &serde_json::to_vec(&original).map_err(|e| e.to_string())?,
        "application/json",
    );
    let after = biomcp_fetch::Shape::inspect(
        &serde_json::to_vec(&edited).map_err(|e| e.to_string())?,
        "application/json",
    );
    let changes = live::drift(&before, &after);
    if !changes.iter().any(|s| s.contains("/results"))
        || !changes.iter().any(|s| s.contains("/count"))
    {
        return Err(format!(
            "planted rename and zero were not reported: {changes:?}"
        ));
    }
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay =
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/replies/gene/main");
    let report = env.root().join("wire.jsonl");
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &[
                "get".into(),
                "gene".into(),
                "KRAS".into(),
                "--replay".into(),
                replay.display().to_string(),
                "--observe".into(),
                report.display().to_string(),
            ],
        )
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("observed replay failed".into());
    }
    let text = fs::read_to_string(report).map_err(|e| e.to_string())?;
    if text.is_empty()
        || text.contains("public example")
        || text.contains("https:")
        || text.contains("summary\":\"")
    {
        return Err("wire report empty or contains payload/URL".into());
    }
    Ok(())
}
