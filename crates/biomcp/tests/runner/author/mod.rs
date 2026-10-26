//! Feature author-search: output costs, help parity and retained native contracts.
use super::{launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
mod cost;
pub(super) fn trials() -> Vec<Trial> {
    let root = super::launch::fixtures().join("author-native");
    let absent = !root.is_dir();
    vec![
        Trial::test("author-search/output-costs-and-help-parity", || {
            costs().map_err(Into::into)
        }),
        Trial::test(
            "author-search/retained-native-records-and-costs",
            move || native(&root).map_err(Into::into),
        )
        .with_ignored_flag(absent),
    ]
}
fn costs() -> Result<(), String> {
    let directory = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases/author");
    cases(&directory, "synthetic")?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    for words in [
        vec!["help", "search", "author"],
        vec!["help", "get", "author"],
        vec!["list", "author"],
    ] {
        let name = words.join("/");
        cost::surfaces(
            &env,
            &name,
            &words.into_iter().map(String::from).collect::<Vec<_>>(),
            &[],
            0,
        )?;
    }
    Ok(())
}
fn native(root: &Path) -> Result<(), String> {
    cases(&root.join("cases"), "native")?;
    let registry = super::schema::Registry::embedded()?;
    for entry in fs::read_dir(root.join("cases")).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        super::run_case(&entry.path(), &registry)?;
    }
    Ok(())
}
fn cases(directory: &Path, prefix: &str) -> Result<(), String> {
    let mut entries = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    entries.sort();
    for path in entries {
        let case: super::Case =
            serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        if let Some(config) = case.config {
            env.config(&config).map_err(|e| e.to_string())?;
        }
        let replay = case.replay.map(|p| {
            let path = std::path::PathBuf::from(p);
            if path.is_absolute() {
                path
            } else {
                super::launch::fixtures().join("replies").join(path)
            }
        });
        for (index, step) in case.steps.into_iter().enumerate() {
            let args = step
                .run
                .or(step.mcp)
                .ok_or("cost case requires explicit command")?;
            let phase = format!(
                "{prefix}/{}/{index}",
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("case stem")?
            );
            let globals = env.globals(replay.as_deref(), step.offline);
            cost::surfaces(&env, &phase, &args, &globals, step.exit)?;
        }
    }
    Ok(())
}
