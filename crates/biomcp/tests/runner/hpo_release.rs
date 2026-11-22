//! Feature disease-hpo-release: a continuation survives a change in the publisher's current release.
use super::launch::{self, Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};

pub(super) fn trial() -> Trial {
    super::transcript::trial("disease-hpo-release/current-release-advances", || {
        advances().map_err(Into::into)
    })
}

fn advances() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let replay = env.root().join("replay");
    let input = launch::fixtures().join("replies/disease/hpo-release/hpo");
    let target = replay.join("hpo");
    fs::create_dir_all(&target).map_err(|error| error.to_string())?;
    let mut latest = None;
    for entry in fs::read_dir(input).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let path = entry.path();
        if path.extension().and_then(|ext| ext.to_str()) != Some("json") {
            continue;
        }
        let record = fs::read_to_string(&path).map_err(|error| error.to_string())?;
        let path = target.join(entry.file_name());
        if record.contains("/releases/latest") {
            fs::write(&path, record.replace("v2099-02-01", "v2099-01-01"))
                .map_err(|error| error.to_string())?;
            latest = Some((path, record));
        } else {
            fs::write(path, record).map_err(|error| error.to_string())?;
        }
    }
    let (path, new_release) = latest.ok_or("missing latest release recording")?;
    let first_args = [
        "search",
        "disease",
        "--source",
        "hpo",
        "--phenotype",
        "HP:0001083",
        "--phenotype",
        "HP:0000098",
        "--phenotype",
        "HP:0001634",
        "--limit",
        "1",
    ]
    .map(str::to_owned);
    let first = cli(&env, &replay, &first_args)?;
    if first.pointer("/result/release") != Some(&json!("v2099-01-01")) {
        return Err(format!(
            "first search missed the publisher's original release: {first}"
        ));
    }
    fs::write(path, new_release).map_err(|error| error.to_string())?;
    let new = cli(&env, &replay, &first_args)?;
    if new.pointer("/result/items/0/gene_links/0/symbol") != Some(&json!("SYNTH3")) {
        return Err(format!(
            "a new search did not pick up the new release: {new}"
        ));
    }
    let next = cli(&env, &replay, &super::action(&first, "page")?)?;
    if next.pointer("/result/release") != Some(&json!("v2099-01-01"))
        || next.pointer("/result/items/0/gene_links_missing") != Some(&json!(true))
    {
        return Err(format!(
            "continuation changed release when the publisher updated it: {next}"
        ));
    }
    Ok(())
}

fn cli(env: &Environment, replay: &Path, args: &[String]) -> Result<Value, String> {
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &launch::arguments(args, &env.globals(Some(replay), false)),
        )
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "{args:?} exited {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())
}
