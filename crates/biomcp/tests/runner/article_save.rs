//! Explicit saves preserve source bytes and leave ownership with the caller.
use super::launch::{Environment, fixtures};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn check() -> Check {
    text()?;
    supplement()?;
    failures()?;
    http_refusal()
}
fn args(kind: &str, path: &Path) -> Vec<String> {
    let mut args = vec!["article".into(), kind.into(), "28944238".into()];
    if kind == "supplement" { args.push("MGG3-5-553-s002.xlsx".into()); }
    args.extend(["--save".into(), path.to_string_lossy().into()]);
    args
}
fn command(env: &Environment, capture: &str, args: &[String], exit: i32) -> Result<Value, Box<dyn std::error::Error>> {
    let mut all = env.globals(Some(&fixtures().join("replies").join(capture)), false);
    all.extend_from_slice(args);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &all).output()?;
    assert_eq!(output.status.code(), Some(exit), "{}", String::from_utf8_lossy(&output.stdout));
    assert!(output.stderr.is_empty());
    Ok(serde_json::from_slice(&output.stdout)?)
}
fn receipt(value: &Value, path: &Path, bytes: &[u8], parts: Option<u32>) -> Check {
    let saved = value.pointer("/result/content/data/saved").ok_or("missing receipt")?;
    let sha: String = Sha256::digest(bytes).iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(saved.get("path"), Some(&json!(path.canonicalize()?.to_string_lossy())));
    assert_eq!(saved.get("bytes"), Some(&json!(bytes.len())));
    assert_eq!(saved.get("sha256"), Some(&json!(sha)));
    assert_eq!(saved.get("parts"), parts.map(|n| json!(n)).as_ref());
    assert!(saved.get("url").and_then(Value::as_str).is_some_and(|s| s.starts_with("https://pmc-oa-opendata.s3.amazonaws.com/PMC5606870.1/")));
    assert_eq!(value.pointer("/result/content/data/license"), Some(&json!("cc_by")));
    super::schema::Registry::embedded()?.validate(value, &args(if parts.is_some() { "text" } else { "supplement" }, path), false)?;
    Ok(())
}
fn text() -> Check {
    let env = Environment::new()?;
    let folder = env.root().join("papers");
    fs::create_dir(&folder)?;
    let capture = "article/fulltext/live-introduction";
    let value = command(&env, capture, &args("text", &folder), 0)?;
    let path = folder.join("28944238.md");
    let body = fs::read_to_string(&path)?;
    receipt(&value, &path, body.as_bytes(), Some(25))?;
    assert!(body.starts_with("# Targeted sequencing of 36 known or putative colorectal cancer susceptibility genes\n"));
    for text in ["PMC5606870", "pmid: 28944238", "doi: 10.1002/mgg3.317", "License: cc_by", "Source: https://", "Introduction", "Discussion", "Table S4. Alternative splicing variants."] {
        assert!(body.contains(text), "lost complete text/header: {text}");
    }
    assert!(body.find("Introduction") < body.find("Discussion"));
    let original = body.as_bytes();
    fs::write(&path, b"caller content")?;
    command(&env, capture, &args("text", &folder), 2)?;
    assert_eq!(fs::read(&path)?, b"caller content");
    let mut replace = args("text", &folder);
    replace.push("--replace".into());
    command(&env, capture, &replace, 0)?;
    assert_eq!(fs::read(&path)?, original);
    let stdio = folder.join("stdio.md");
    let value = super::mcp::call_with_globals(&env, &args("text", &stdio), false, &env.globals(Some(&fixtures().join("replies").join(capture)), false))?;
    receipt(&value, &stdio, original, Some(25))?;
    assert_eq!(fs::read(stdio)?, original);
    assert_eq!(fs::read_dir(folder)?.count(), 2);
    Ok(())
}
fn supplement() -> Check {
    let env = Environment::new()?;
    let folder = env.root().join("supplements");
    fs::create_dir(&folder)?;
    let capture = "article/supplements/live";
    let original = fs::read(fixtures().join("replies").join(capture).join("pmc-oa/51c9ef213345bdbc7ed493ea19928bd900d5791413a6c5a65b60f2c50eac690f-0.bin"))?;
    let value = command(&env, capture, &args("supplement", &folder), 0)?;
    let path = folder.join("MGG3-5-553-s002.xlsx");
    assert_eq!(fs::read(&path)?, original);
    receipt(&value, &path, &original, None)?;
    let stdio = folder.join("stdio.xlsx");
    let value = super::mcp::call_with_globals(&env, &args("supplement", &stdio), false, &env.globals(Some(&fixtures().join("replies").join(capture)), false))?;
    receipt(&value, &stdio, &original, None)?;
    assert_eq!(fs::read(stdio)?, original);
    Ok(())
}
fn failures() -> Check {
    for (kind, capture) in [("supplement", "article/supplements/synthetic-cut"), ("text", "article/fulltext/synthetic-cut")] {
        let env = Environment::new()?;
        let folder = env.root().join("empty");
        fs::create_dir(&folder)?;
        let target = folder.join("never");
        command(&env, capture, &args(kind, &target), 4)?;
        assert_eq!(fs::read_dir(&folder)?.count(), 0);
        fs::write(&target, b"caller content")?;
        let mut replace = args(kind, &target);
        replace.push("--replace".into());
        command(&env, capture, &replace, 4)?;
        assert_eq!(fs::read(&target)?, b"caller content");
        assert_eq!(fs::read_dir(folder)?.count(), 1);
    }
    Ok(())
}
fn http_refusal() -> Check {
    for (kind, capture) in [("text", "article/fulltext/live-introduction"), ("supplement", "article/supplements/live")] {
        let env = Environment::new()?;
        let path = env.root().join("server-must-not-write");
        let mut server = super::http::Server::start_replay(fixtures().join("replies").join(capture))?;
        let call = super::mcp::request(json!(7), "tools/call", json!({"name":"biomcp","arguments":{"args":args(kind, &path)}}));
        let (status, reply) = server.request("POST", "/mcp", &call.to_string())?;
        assert_eq!(status, 200);
        assert_eq!(reply.pointer("/result/isError"), Some(&json!(true)));
        assert_eq!(reply.pointer("/result/structuredContent/status"), Some(&json!("usage")));
        assert!(reply.pointer("/result/structuredContent/error/url").and_then(Value::as_str).is_some_and(|s| s.starts_with("https://pmc-oa-opendata.s3.amazonaws.com/")));
        assert!(!path.exists());
        server.stop()?;
    }
    Ok(())
}
