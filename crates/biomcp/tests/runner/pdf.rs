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
        .ok_or_else(|| format!("missing {path}"))
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

pub(super) fn doi_reader() -> Result<(), String> {
    use super::network_stub::Stub;
    let environment = Environment::new().map_err(|error| error.to_string())?;
    let stub = Stub::for_source("api.semanticscholar.org", "application/json", |_| {
        Ok(
            serde_json::json!({"openAccessPdf":{"url":"","status":"CLOSED","license":null}})
                .to_string(),
        )
    })?;
    let certificate = environment.root().join("root.pem");
    fs::write(&certificate, &stub.certificate).map_err(|error| error.to_string())?;
    environment
        .config(&serde_json::json!({"ca_bundle":certificate}))
        .map_err(|error| error.to_string())?;
    let record = environment.root().join("record");
    fs::create_dir(&record).map_err(|error| error.to_string())?;
    let record = fs::canonicalize(record).map_err(|error| error.to_string())?;
    let args = vec![
        "--record".into(),
        record.to_string_lossy().into_owned(),
        "article".into(),
        "text".into(),
        "10.31083/FBL39100".into(),
        "--pdf".into(),
        "--part".into(),
        "1".into(),
    ];
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("HTTPS_PROXY", format!("http://{}", stub.address))
        .output()
        .map_err(|error| error.to_string())?;
    let result: Value =
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    if result.pointer("/result/content/reason") != Some(&Value::String("not_open".into())) {
        return Err("the checked DOI did not reach its paper endpoint".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("DOI recording made unexpected source requests".into());
    }
    complete_doi_fixture(&record)?;
    let mut args = environment.globals(Some(&record), false);
    args.extend(
        [
            "article",
            "text",
            "10.31083/FBL39100",
            "--pdf",
            "--part",
            "1",
        ]
        .map(str::to_owned),
    );
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    let result: Value =
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    if result.pointer("/result/content/data/origin") != Some(&Value::String("pdf".into())) {
        return Err("DOI PDF part was unavailable".into());
    }
    let step = result
        .pointer("/next/0/args")
        .and_then(Value::as_array)
        .ok_or("missing DOI continuation")?;
    if !step.contains(&Value::String("10.31083/FBL39100".into()))
        || !step.contains(&Value::String("--pdf".into()))
    {
        return Err("PDF continuation lost its DOI or reader".into());
    }
    let mut args = environment.globals(None, true);
    args.extend(
        step.iter()
            .map(|value| value.as_str().unwrap_or("").to_owned()),
    );
    let output = environment
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    let result: Value =
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    if result.pointer("/result/content/data/part") != Some(&Value::from(2)) {
        return Err("offline DOI continuation did not read its stored second part".into());
    }
    Ok(())
}
fn complete_doi_fixture(record: &Path) -> Result<(), String> {
    let source = launch::fixtures().join("replies/article/pdf/long-paragraph/semantic-scholar");
    let target = record.join("semantic-scholar");
    let mut body = None;
    for entry in fs::read_dir(source).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path
            .extension()
            .is_some_and(|extension| extension == "json")
        {
            let value: Value =
                serde_json::from_slice(&fs::read(&path).map_err(|error| error.to_string())?)
                    .map_err(|error| error.to_string())?;
            if value["url"]
                .as_str()
                .is_some_and(|url| url.contains("api.semanticscholar.org"))
            {
                body = value.pointer("/exchanges/0/body").cloned();
                continue;
            }
        }
        fs::copy(
            &path,
            target.join(path.file_name().ok_or("missing recording filename")?),
        )
        .map_err(|error| error.to_string())?;
    }
    for entry in fs::read_dir(&target).map_err(|error| error.to_string())? {
        let path = entry.map_err(|error| error.to_string())?.path();
        if path.extension().is_none_or(|extension| extension != "json") {
            continue;
        }
        let mut value: Value =
            serde_json::from_slice(&fs::read(&path).map_err(|error| error.to_string())?)
                .map_err(|error| error.to_string())?;
        if value["url"]
            .as_str()
            .is_some_and(|url| url.contains("api.semanticscholar.org"))
        {
            *value
                .pointer_mut("/exchanges/0/body")
                .ok_or("missing reply body")? = body.clone().ok_or("missing PDF link fixture")?;
            fs::write(
                path,
                serde_json::to_vec(&value).map_err(|error| error.to_string())?,
            )
            .map_err(|error| error.to_string())?;
        }
    }
    Ok(())
}
