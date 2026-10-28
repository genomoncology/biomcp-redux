//! Feature author-search: retained source contracts and output measurements.
use crate::runner::transcript::trial as transcript_trial;
use super::{launch::Environment, mcp};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
mod controls;
mod cost;
mod disabled_pool;
mod pool;
pub(super) fn trials() -> Vec<Trial> {
    let root = super::launch::fixtures().join("author-native");
    let absent = !root.is_dir();
    vec![
        transcript_trial("author-search/disabled-warm-source-pools", || {
            disabled_pool::check().map_err(Into::into)
        }),
        transcript_trial("author-search/zero-io-controls", || {
            controls::check().map_err(Into::into)
        }),
        transcript_trial("author-papers/help-and-markdown", || {
            help().map_err(Into::into)
        }),
        transcript_trial("author-search/retained-native-pool-contract", || {
            pool::check("pool").map_err(Into::into)
        }),
        transcript_trial("author-search/stale-source-provenance", || {
            pool::check("stale").map_err(Into::into)
        }),
        transcript_trial(
            "author-search/retained-native-records-and-costs",
            move || native(&root).map_err(Into::into),
        )
        .with_ignored_flag(absent),
    ]
}
fn native(root: &Path) -> Result<(), String> {
    let mut entries = fs::read_dir(root.join("cases"))
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
        let replay = case.replay.map(std::path::PathBuf::from);
        let mut previous = None;
        for (index, step) in case.steps.into_iter().enumerate() {
            let args = if let Some(kind) = &step.follow {
                super::action(
                    previous
                        .as_ref()
                        .ok_or("native follow requires prior page")?,
                    kind,
                )?
            } else {
                step.run
                    .clone()
                    .or(step.mcp.clone())
                    .ok_or("native case requires command")?
            };
            let phase = format!(
                "native/{}/{index}",
                path.file_stem()
                    .and_then(|s| s.to_str())
                    .ok_or("case stem")?
            );
            let globals = env.globals(replay.as_deref(), step.offline);
            previous = Some(cost::surfaces(&env, &phase, &args, &globals, &step)?);
        }
    }
    discovery()?;
    Ok(())
}

fn discovery() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let retained = env.root().join("discovery");
    fs::create_dir_all(&retained).map_err(|e| e.to_string())?;
    let frames = mcp::exchange_retained(
        &env,
        &[mcp::request(json!(1), "tools/list", json!({}))],
        &[],
        Some(&retained),
    )?;
    let frame = frames.first().ok_or("tools/list frame")?;
    let description = frame
        .pointer("/result/tools/0/description")
        .and_then(Value::as_str)
        .ok_or("description")?;
    let raw = fs::read(retained.join("mcp.stdout")).map_err(|e| e.to_string())?;
    writeln!(
        std::io::stderr(),
        "AUTHOR_DISCOVERY description={} tools-list-compact={} tools-list-raw={}",
        description.len(),
        serde_json::to_vec(frame).map_err(|e| e.to_string())?.len(),
        raw.len()
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn help() -> Result<(), String> {
    let path =
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases/author/papers-help.yaml");
    let case: super::Case = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    for (index, step) in case.steps.into_iter().enumerate() {
        let args = step.run.clone().ok_or("help command")?;
        cost::surfaces(&env, &format!("papers-help/{index}"), &args, &[], &step)?;
    }
    Ok(())
}
