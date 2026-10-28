//! Explicit PDF files through the real command and stdio MCP boundary.
use super::launch::{Environment, fixtures};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("article/pdf/recorded-metadata-contract", || {
            record_metadata().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/pdf/size-cut-and-proxy-boundaries", || {
            boundaries().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/pdf/http-save-refused-with-link", || {
            http_refusal().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/pdf/atomic-explicit-saves", || {
            saves().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/pdf/hostile-existing-paths", || {
            existing_paths().map_err(|e| e.to_string().into())
        }),
        transcript_trial("article/pdf/failed-download-leaves-nothing", || {
            failures().map_err(|e| e.to_string().into())
        }),
    ]
}
fn command(
    env: &Environment,
    replay: &Path,
    args: &[String],
    exit: i32,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut full = env.globals(Some(replay), false);
    full.extend_from_slice(args);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &full).output()?;
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    Ok(serde_json::from_slice(&output.stdout)?)
}
fn save_args(path: &Path) -> Vec<String> {
    vec![
        "article".into(),
        "pdf".into(),
        "41351423".into(),
        "--save".into(),
        path.to_string_lossy().into(),
    ]
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
    let sha: String = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(
        receipt.pointer("/result/file/data/path"),
        Some(&json!(target.canonicalize()?.to_string_lossy()))
    );
    assert_eq!(
        receipt.pointer("/result/file/data/bytes"),
        Some(&json!(bytes.len()))
    );
    assert_eq!(
        receipt.pointer("/result/file/data/sha256"),
        Some(&json!(sha))
    );
    fs::write(&target, b"keep existing bytes")?;
    let refused = command(&env, &replay, &save_args(&folder), 2)?;
    assert_eq!(refused.pointer("/status"), Some(&json!("usage")));
    assert_eq!(fs::read(&target)?, b"keep existing bytes");
    let mut args = save_args(&folder);
    args.push("--replace".into());
    command(&env, &replay, &args, 0)?;
    assert_eq!(fs::read(&target)?, bytes);
    let mcp_path = folder.join("stdio.pdf");
    let mcp = super::mcp::call_with_globals(
        &env,
        &save_args(&mcp_path),
        false,
        &env.globals(Some(&replay), false),
    )?;
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
        command(
            &env,
            &fixtures().join("replies/article/pdf").join(name),
            &save_args(&target),
            4,
        )?;
        assert_eq!(fs::read_dir(&folder)?.count(), 0, "{name} left a file");
        fs::write(&target, b"preserve on failure")?;
        let mut args = save_args(&target);
        args.push("--replace".into());
        command(
            &env,
            &fixtures().join("replies/article/pdf").join(name),
            &args,
            4,
        )?;
        assert_eq!(fs::read(&target)?, b"preserve on failure");
        assert_eq!(fs::read_dir(&folder)?.count(), 1);
    }
    Ok(())
}

fn existing_paths() -> Check {
    let env = Environment::new()?;
    let replay = fixtures().join("replies/article/pdf/synthetic");
    let original = env.root().join("original.pdf");
    let target = env.root().join("target.pdf");
    fs::write(&original, b"caller-owned bytes")?;
    fs::hard_link(&original, &target)?;
    command(&env, &replay, &save_args(&target), 2)?;
    let mut args = save_args(&target);
    args.push("--replace".into());
    command(&env, &replay, &args, 0)?;
    assert!(fs::read(&target)?.starts_with(b"%PDF-"));
    assert_eq!(fs::read(&original)?, b"caller-owned bytes");
    fs::remove_file(&target)?;
    #[cfg(unix)]
    {
        for destination in [&original, &env.root().join("missing.pdf")] {
            std::os::unix::fs::symlink(destination, &target)?;
            command(&env, &replay, &save_args(&target), 2)?;
            command(&env, &replay, &args, 2)?;
            assert!(fs::symlink_metadata(&target)?.file_type().is_symlink());
            fs::remove_file(&target)?;
        }
    }
    assert_eq!(fs::read(&original)?, b"caller-owned bytes");
    assert!(!env.root().join("missing.pdf").exists());
    assert!(fs::read_dir(env.root())?.all(|entry| {
        entry.is_ok_and(|entry| !entry.file_name().to_string_lossy().starts_with(".tmp"))
    }));
    Ok(())
}

fn http_refusal() -> Check {
    let env = Environment::new()?;
    let target = env.root().join("never.pdf");
    let source = fixtures().join("replies/article/pdf/synthetic/semantic-scholar");
    let replay = env.root().join("link-only/semantic-scholar");
    fs::create_dir_all(&replay)?;
    // Only the API record exists: HTTP refusal cannot consume publisher bytes.
    let name = "69475a488feb05ba4c86ed028e3791cdf3e218666f11ac7a1c145a9b60baf883.json";
    fs::copy(source.join(name), replay.join(name))?;
    let mut server =
        super::http::Server::start_replay(replay.parent().ok_or("replay parent")?.to_owned())?;
    let call = super::mcp::request(
        json!(7),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":save_args(&target)}}),
    );
    let (status, reply) = server.request("POST", "/mcp", &call.to_string())?;
    assert_eq!(status, 200);
    assert_eq!(reply.pointer("/result/isError"), Some(&json!(true)));
    assert_eq!(
        reply.pointer("/result/structuredContent/status"),
        Some(&json!("usage"))
    );
    assert!(
        reply
            .pointer("/result/structuredContent/error/url")
            .and_then(Value::as_str)
            .is_some_and(|s| s.ends_with("39100.pdf"))
    );
    assert!(!target.exists());
    server.stop()?;
    Ok(())
}

fn boundaries() -> Check {
    for mode in ["oversize", "cut-api", "link-only"] {
        let env = Environment::new()?;
        let replay = env.root().join("recordings");
        super::response_cases::copy_capture("article/pdf/synthetic", &replay, |_, record| {
            if mode == "cut-api"
                && record["url"]
                    .as_str()
                    .is_some_and(|s| s.contains("api.semanticscholar.org"))
                && let Some(body) = record.pointer_mut("/exchanges/0/body")
            {
                *body = json!({"kind":"text","value":"{\"openAccessPdf\":"});
            }
        })?;
        let publisher = replay.join("semantic-scholar/0e4c6627e9a47ffe7aa8ad4caee7846838977da402ca279d1d34fbe2b02efcaf.json");
        if mode == "oversize" {
            let record: Value = serde_json::from_slice(&fs::read(&publisher)?)?;
            let sidecar = record
                .pointer("/exchanges/0/body/value")
                .and_then(Value::as_str)
                .ok_or("sidecar")?;
            fs::OpenOptions::new()
                .write(true)
                .open(publisher.parent().ok_or("parent")?.join(sidecar))?
                .set_len(25 * 1024 * 1024 + 1)?;
        }
        let target = env.root().join("saved.pdf");
        if mode == "link-only" {
            fs::remove_file(publisher)?;
            let args: Vec<String> = ["get", "article", "41351423", "pdf"]
                .map(str::to_owned)
                .into();
            let result = command(&env, &replay, &args, 0)?;
            assert_eq!(
                result.pointer("/result/items/0/sections/pdf/data/status"),
                Some(&json!("gold"))
            );
            let expected = json!(["article", "pdf", "41351423", "--save", "41351423.pdf"]);
            assert!(
                result
                    .get("next")
                    .and_then(Value::as_array)
                    .is_some_and(|next| next
                        .iter()
                        .any(|step| step.get("args") == Some(&expected)))
            );
        } else {
            let result = command(&env, &replay, &save_args(&target), 4)?;
            assert_eq!(
                result.pointer("/result/file/reason"),
                Some(&json!("bad_reply"))
            );
        }
        assert!(!target.exists());
    }
    proxy_paths()
}
fn proxy_paths() -> Check {
    let listener = std::net::TcpListener::bind("127.0.0.1:0")?;
    listener.set_nonblocking(true)?;
    for bypass in [false, true] {
        let env = Environment::new()?;
        let target = env.root().join("proxy.pdf");
        let mut args = env.globals(
            Some(&fixtures().join("replies/article/pdf/synthetic")),
            false,
        );
        args.extend(save_args(&target));
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .env(
                "HTTPS_PROXY",
                format!(
                    "http://user:private-proxy-secret@{}",
                    listener.local_addr()?
                ),
            )
            .env("NO_PROXY", if bypass { "storage.imrpress.com" } else { "" })
            .output()?;
        let result: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(output.status.code(), Some(if bypass { 0 } else { 4 }));
        assert!(!String::from_utf8_lossy(&output.stdout).contains("private-proxy-secret"));
        if bypass {
            assert!(fs::read(&target)?.starts_with(b"%PDF-"));
        } else {
            assert_eq!(result.pointer("/result/file/reason"), Some(&json!("proxy")));
            assert!(!target.exists());
        }
        assert!(
            listener.accept().is_err(),
            "refused publisher request reached proxy"
        );
    }
    Ok(())
}

fn record_metadata() -> Check {
    let stub = super::network_stub::Stub::for_source(
        "api.semanticscholar.org",
        "application/json",
        |_| Ok("{\"openAccessPdf\":null}".into()),
    )?;
    let env = Environment::new()?;
    let certificate = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&certificate, &stub.certificate)?;
    env.config(&json!({"ca_bundle":certificate}))?;
    let capture = env.root().join("recording");
    let args = vec![
        "--record".into(),
        capture.to_string_lossy().into(),
        "article".into(),
        "pdf".into(),
        "33123537".into(),
        "--save".into(),
        env.root().join("never.pdf").to_string_lossy().into(),
    ];
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .env("HTTPS_PROXY", format!("http://{}", stub.address))
        .output()?;
    assert_eq!(output.status.code(), Some(4));
    assert!(!env.root().join("never.pdf").exists());
    let requests = stub.finish()?;
    assert_eq!(requests.len(), 1);
    assert!(requests.first().is_some_and(|(_, request)| {
        request.contains("/paper/PMID%3A33123537?fields=openAccessPdf")
    }));
    for entry in fs::read_dir(capture.join("semantic-scholar"))? {
        let entry = entry?;
        if entry.path().extension().is_some_and(|v| v == "json") {
            let record: Value = serde_json::from_slice(&fs::read(entry.path())?)?;
            assert_eq!(
                record.pointer("/exchanges/0/body/value/openAccessPdf"),
                Some(&Value::Null)
            );
        }
    }
    Ok(())
}
