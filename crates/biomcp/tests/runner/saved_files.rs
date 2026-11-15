//! Every checked-in saved.sqlite opens with this build and exports its checked-in JSON
//! (sdlc/planning/saved-work.md, Compatibility 1). BIOMCP_WRITE_SAVED_FIXTURE=DIR first
//! writes this build's file and export under DIR/<version>/ from real commands.
use super::launch::{Environment, fixtures};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;
use std::{fs, path::Path, path::PathBuf};

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "saved-work/checked-in-files-export",
        || exports().map_err(Into::into),
    )]
}

/// Where the product resolves saved.sqlite inside an isolated test environment.
fn saved_file(env: &Environment) -> PathBuf {
    if cfg!(windows) {
        env.root().join("config/biomcp/saved.sqlite")
    } else {
        env.root().join("home/.local/share/biomcp/saved.sqlite")
    }
}

fn run(env: &Environment, args: &[&str]) -> Result<Vec<u8>, String> {
    let args: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "{args:?} failed: {}",
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    Ok(output.stdout)
}

#[expect(
    clippy::disallowed_methods,
    reason = "The fixture writer is selected only by an explicit job variable"
)]
fn requested_output() -> Option<PathBuf> {
    std::env::var_os("BIOMCP_WRITE_SAVED_FIXTURE").map(PathBuf::from)
}

/// Build this release's file through the commands a user runs.
fn write(out: &Path) -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let replay = fixtures().join("replies/trial/condition");
    let mut search = vec!["search", "trial", "--condition", "melanoma", "--save-as", "melanoma-trials"];
    let globals = env.globals(Some(replay.as_path()), false);
    search.extend(globals.iter().map(String::as_str));
    run(&env, &search)?;
    run(
        &env,
        &[
            "saved", "list", "add", "melanoma-picks", "--entity", "trial", "--note",
            "Recruiting in the US", "NCT04631731", "NCT05355701",
        ],
    )?;
    run(&env, &["saved", "list", "add", "braf-genes", "--entity", "gene", "BRAF", "NRAS"])?;
    let folder = out.join(env!("CARGO_PKG_VERSION"));
    fs::create_dir_all(&folder).map_err(|error| error.to_string())?;
    fs::copy(saved_file(&env), folder.join("saved.sqlite")).map_err(|error| error.to_string())?;
    let export: Value = serde_json::from_slice(&run(&env, &["saved", "export"])?)
        .map_err(|error| error.to_string())?;
    let mut text = serde_json::to_string_pretty(&export).map_err(|error| error.to_string())?;
    text.push('\n');
    fs::write(folder.join("export.json"), text).map_err(|error| error.to_string())
}

fn exports() -> Result<(), String> {
    if let Some(out) = requested_output() {
        write(&out)?;
    }
    let root = fixtures().join("fixtures/saved");
    let mut folders: Vec<PathBuf> = fs::read_dir(&root)
        .map_err(|error| format!("{}: {error}", root.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|error| error.to_string())?;
    folders.sort();
    if folders.is_empty() {
        return Err("no checked-in saved files".into());
    }
    for folder in folders {
        let env = Environment::new().map_err(|error| error.to_string())?;
        let target = saved_file(&env);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(folder.join("saved.sqlite"), &target).map_err(|error| error.to_string())?;
        let actual: Value = serde_json::from_slice(&run(&env, &["saved", "export"])?)
            .map_err(|error| error.to_string())?;
        let expected: Value = serde_json::from_str(
            &fs::read_to_string(folder.join("export.json")).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;
        if actual != expected {
            return Err(format!(
                "{}: export differs\nactual: {actual}\nexpected: {expected}",
                folder.display()
            ));
        }
    }
    Ok(())
}
