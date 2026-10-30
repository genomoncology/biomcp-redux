//! Explicit PDF files through the real command and stdio MCP boundary.
use super::launch::{Environment, fixtures};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;

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
    assert!(
        output.stderr.is_empty(),
        "PDF commands must keep diagnostics out of their JSON response"
    );
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
        assert_eq!(
            fs::read(&target)?,
            b"preserve on failure",
            "Failed PDF downloads must leave no partial output behind"
        );
        assert_eq!(
            fs::read_dir(&folder)?.count(),
            1,
            "Failed PDF downloads must leave no partial output behind"
        );
    }
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
    assert_eq!(
        status, 200,
        "HTTP requests must refuse local PDF saves and retain the source link"
    );
    assert_eq!(
        reply.pointer("/result/isError"),
        Some(&json!(true)),
        "HTTP requests must refuse local PDF saves and retain the source link"
    );
    assert_eq!(
        reply.pointer("/result/structuredContent/status"),
        Some(&json!("usage")),
        "HTTP requests must refuse local PDF saves and retain the source link"
    );
    assert!(
        reply
            .pointer("/result/structuredContent/error/url")
            .and_then(Value::as_str)
            .is_some_and(|s| s.ends_with("39100.pdf")),
        "HTTP requests must refuse local PDF saves and retain the source link"
    );
    assert!(
        !target.exists(),
        "HTTP requests must refuse local PDF saves and retain the source link"
    );
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
                Some(&json!("gold")),
                "PDF admission must reject oversized or cut replies"
            );
            let expected = json!(["article", "pdf", "41351423", "--save", "41351423.pdf"]);
            assert!(
                result
                    .get("next")
                    .and_then(Value::as_array)
                    .is_some_and(|next| next
                        .iter()
                        .any(|step| step.get("args") == Some(&expected))),
                "PDF admission must reject oversized or cut replies"
            );
        } else {
            let result = command(&env, &replay, &save_args(&target), 4)?;
            assert_eq!(
                result.pointer("/result/file/reason"),
                Some(&json!("bad-reply")),
                "PDF admission must reject oversized or cut replies"
            );
        }
        assert!(
            !target.exists(),
            "PDF admission must reject oversized or cut replies"
        );
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
        assert_eq!(
            output.status.code(),
            Some(if bypass { 0 } else { 4 }),
            "PDF requests must preserve the configured proxy boundary"
        );
        assert!(
            !String::from_utf8_lossy(&output.stdout).contains("private-proxy-secret"),
            "PDF requests must preserve the configured proxy boundary"
        );
        if bypass {
            assert!(
                fs::read(&target)?.starts_with(b"%PDF-"),
                "PDF requests must preserve the configured proxy boundary"
            );
        } else {
            assert_eq!(
                result.pointer("/result/file/reason"),
                Some(&json!("proxy")),
                "PDF requests must preserve the configured proxy boundary"
            );
            assert!(
                !target.exists(),
                "PDF requests must preserve the configured proxy boundary"
            );
        }
        assert!(
            listener.accept().is_err(),
            "refused publisher request reached proxy"
        );
    }
    Ok(())
}
