//! Publisher network policy and response-size refusals through the installed command.
use super::launch::{self, Environment};
use serde_json::Value;
use std::{fs, path::Path};
const PDF: &str = "/result/items/0/sections/pdf";
fn invoke(
    environment: &Environment,
    replay: &Path,
    extra: &[&str],
    bypass: Option<&str>,
) -> Result<Value, String> {
    let mut args = environment.globals(Some(replay), false);
    args.extend(extra.iter().map(|value| (*value).to_owned()));
    args.extend(["get", "article", "41351423", "pdf"].map(str::to_owned));
    let mut command = environment.command(env!("CARGO_BIN_EXE_biomcp"), &args);
    if let Some(bypass) = bypass {
        command
            .env("HTTPS_PROXY", "http://proxy.example.test:9000")
            .env("NO_PROXY", bypass);
    }
    let result = command.output().map_err(|error| error.to_string())?;
    serde_json::from_slice(&result.stdout).map_err(|error| format!("PDF result: {error}"))
}
fn value<'a>(result: &'a Value, path: &str) -> Result<&'a Value, String> {
    result
        .pointer(path)
        .ok_or_else(|| format!("missing {path}: {result}"))
}
pub(super) fn policy() -> Result<(), String> {
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let replay = launch::fixtures().join("replies/article/pdf/synthetic");
    let refused = invoke(&environment, &replay, &[], Some(""))?;
    if value(&refused, &format!("{PDF}/reason"))? != "proxy" {
        return Err("an applicable proxy did not prevent the publisher download".into());
    }
    let direct = invoke(&environment, &replay, &[], Some("storage.imrpress.com"))?;
    if value(&direct, &format!("{PDF}/state"))? != "data" {
        return Err("NO_PROXY did not retain the direct publisher route".into());
    }
    let disabled = invoke(
        &environment,
        &replay,
        &["--disable-sources", "semantic-scholar"],
        None,
    )?;
    if value(&disabled, &format!("{PDF}/state"))? != "skipped"
        || value(&disabled, &format!("{PDF}/reason"))? != "disabled"
    {
        return Err("stored PDF text bypassed the source switch".into());
    }
    Ok(())
}
pub(super) fn oversized() -> Result<(), String> {
    let directory = tempfile::tempdir().map_err(|error| error.to_string())?;
    copy(
        &launch::fixtures().join("replies/article/pdf/synthetic"),
        directory.path(),
    )?;
    let publisher = directory.path().join("semantic-scholar");
    for entry in fs::read_dir(&publisher).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().is_some_and(|extension| extension == "bin") {
            fs::OpenOptions::new()
                .write(true)
                .open(&path)
                .and_then(|file| file.set_len(25 * 1024 * 1024 + 1))
                .map_err(|error| error.to_string())?;
        }
    }
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let result = invoke(&environment, directory.path(), &[], None)?;
    if value(&result, &format!("{PDF}/reason"))? != "bad_reply"
        || value(&result, "/result/items/0/answer/resolution")? != "resolved"
    {
        return Err("oversized publisher reply did not preserve the article card".into());
    }
    Ok(())
}
fn copy(source: &Path, target: &Path) -> Result<(), String> {
    fs::create_dir_all(target).map_err(|error| error.to_string())?;
    for entry in fs::read_dir(source).map_err(|error| error.to_string())? {
        let entry = entry.map_err(|error| error.to_string())?;
        let destination = target.join(entry.file_name());
        if entry.path().is_dir() {
            copy(&entry.path(), &destination)?;
        } else {
            fs::copy(entry.path(), destination).map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}
