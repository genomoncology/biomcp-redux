//! Explicit PDF files through the real command and stdio MCP boundary.
use super::launch::{Environment, fixtures};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("article/pdf/atomic-explicit-saves", || saves().map_err(|e| e.to_string().into())),
        Trial::test("article/pdf/failed-download-leaves-nothing", || failures().map_err(|e| e.to_string().into()))]
}
fn command(env: &Environment, replay: &Path, args: &[String], exit: i32) -> Result<Value, Box<dyn std::error::Error>> {
    let mut full = env.globals(Some(replay), false);
    full.extend_from_slice(args);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &full).output()?;
    assert_eq!(output.status.code(), Some(exit), "{}", String::from_utf8_lossy(&output.stdout));
    assert!(output.stderr.is_empty());
    Ok(serde_json::from_slice(&output.stdout)?)
}
fn save_args(path: &Path) -> Vec<String> {
    vec!["article".into(), "pdf".into(), "41351423".into(), "--save".into(), path.to_string_lossy().into()]
}
fn saves() -> Check {
    let env = Environment::new()?;
    let replay = fixtures().join("replies/article/pdf/synthetic");
    let folder = env.root().join("downloads");
    fs::create_dir(&folder)?;
    let target = folder.join("41351423.pdf");
    let receipt = command(&env, &replay, &save_args(&folder), 0)?;
    let bytes = fs::read(&target)?;
    assert!(bytes.starts_with(b"%PDF-"));
    let sha: String = Sha256::digest(&bytes).iter().map(|b| format!("{b:02x}")).collect();
    assert_eq!(receipt.pointer("/result/file/data/path"), Some(&json!(target.canonicalize()?.to_string_lossy())));
    assert_eq!(receipt.pointer("/result/file/data/bytes"), Some(&json!(bytes.len())));
    assert_eq!(receipt.pointer("/result/file/data/sha256"), Some(&json!(sha)));
    fs::write(&target, b"keep existing bytes")?;
    let refused = command(&env, &replay, &save_args(&folder), 2)?;
    assert_eq!(refused.pointer("/status"), Some(&json!("usage")));
    assert_eq!(fs::read(&target)?, b"keep existing bytes");
    let mut args = save_args(&folder);
    args.push("--replace".into());
    command(&env, &replay, &args, 0)?;
    assert_eq!(fs::read(&target)?, bytes);
    let mcp_path = folder.join("stdio.pdf");
    let mcp = super::mcp::call_with_globals(&env, &save_args(&mcp_path), false, &env.globals(Some(&replay), false))?;
    assert_eq!(mcp.pointer("/result/file/data/sha256"), Some(&json!(sha)));
    assert_eq!(fs::read(&mcp_path)?, bytes);
    assert_eq!(fs::read_dir(folder)?.count(), 2);
    Ok(())
}
fn failures() -> Check {
    for name in ["not-pdf", "unavailable", "timeout"] {
        let env = Environment::new()?;
        let folder = env.root().join("downloads");
        fs::create_dir(&folder)?;
        let target = folder.join("failed.pdf");
        command(&env, &fixtures().join("replies/article/pdf").join(name), &save_args(&target), 1)?;
        assert_eq!(fs::read_dir(&folder)?.count(), 0, "{name} left a file");
        fs::write(&target, b"preserve on failure")?;
        let mut args = save_args(&target);
        args.push("--replace".into());
        command(&env, &fixtures().join("replies/article/pdf").join(name), &args, 1)?;
        assert_eq!(fs::read(&target)?, b"preserve on failure");
        assert_eq!(fs::read_dir(&folder)?.count(), 1);
    }
    Ok(())
}
