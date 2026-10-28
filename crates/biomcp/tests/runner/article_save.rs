//! Explicit saves preserve source bytes and leave ownership with the caller.
use super::launch::{Environment, fixtures};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn check() -> Check {
    text()?;
    preprint()?;
    supplement()?;
    cache_boundary()?;
    pdf()?;
    opaque()?;
    arguments()?;
    failures()?;
    cut_body()?;
    http_refusal()
}
fn args(kind: &str, path: &Path) -> Vec<String> {
    let mut args = vec!["article".into(), kind.into(), "28944238".into()];
    if kind == "supplement" {
        args.push("MGG3-5-553-s002.xlsx".into());
    }
    args.extend(["--save".into(), path.to_string_lossy().into()]);
    args
}
fn command(
    env: &Environment,
    capture: &str,
    args: &[String],
    exit: i32,
) -> Result<Value, Box<dyn std::error::Error>> {
    command_at(env, &fixtures().join("replies").join(capture), args, exit)
}
fn command_at(
    env: &Environment,
    replay: &Path,
    args: &[String],
    exit: i32,
) -> Result<Value, Box<dyn std::error::Error>> {
    let mut all = env.globals(Some(replay), false);
    all.extend_from_slice(args);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &all).output()?;
    assert_eq!(
        output.status.code(),
        Some(exit),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    assert!(output.stderr.is_empty());
    Ok(serde_json::from_slice(&output.stdout)?)
}
fn receipt(value: &Value, path: &Path, bytes: &[u8], parts: Option<u32>) -> Check {
    let saved = value
        .pointer("/result/content/data/saved")
        .ok_or("missing receipt")?;
    let sha: String = Sha256::digest(bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(
        saved.get("path"),
        Some(&json!(path.canonicalize()?.to_string_lossy()))
    );
    assert_eq!(saved.get("bytes"), Some(&json!(bytes.len())));
    assert_eq!(saved.get("sha256"), Some(&json!(sha)));
    assert_eq!(saved.get("parts"), parts.map(|n| json!(n)).as_ref());
    assert!(
        saved.get("url").and_then(Value::as_str).is_some_and(
            |s| s.starts_with("https://pmc-oa-opendata.s3.amazonaws.com/PMC5606870.1/")
        )
    );
    assert_eq!(
        value.pointer("/result/content/data/license"),
        Some(&json!("cc_by"))
    );
    super::schema::Registry::embedded()?.validate(
        value,
        &args(
            if parts.is_some() {
                "text"
            } else {
                "supplement"
            },
            path,
        ),
        false,
    )?;
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
    assert!(body.starts_with(
        "# Targeted sequencing of 36 known or putative colorectal cancer susceptibility genes\n"
    ));
    for text in [
        "PMC5606870",
        "pmid: 28944238",
        "doi: 10.1002/mgg3.317",
        "License: cc_by",
        "Source: https://",
        "Introduction",
        "Discussion",
        "**Table S4.** Alternative splicing variants.",
    ] {
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
    let value = super::mcp::call_with_globals(
        &env,
        &args("text", &stdio),
        false,
        &env.globals(Some(&fixtures().join("replies").join(capture)), false),
    )?;
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
    let original =
        fs::read(fixtures().join("replies").join(capture).join(
            "pmc-oa/51c9ef213345bdbc7ed493ea19928bd900d5791413a6c5a65b60f2c50eac690f-0.bin",
        ))?;
    let value = command(&env, capture, &args("supplement", &folder), 0)?;
    let path = folder.join("MGG3-5-553-s002.xlsx");
    assert_eq!(fs::read(&path)?, original);
    receipt(&value, &path, &original, None)?;
    let stdio = folder.join("stdio.xlsx");
    let value = super::mcp::call_with_globals(
        &env,
        &args("supplement", &stdio),
        false,
        &env.globals(Some(&fixtures().join("replies").join(capture)), false),
    )?;
    receipt(&value, &stdio, &original, None)?;
    assert_eq!(fs::read(stdio)?, original);
    Ok(())
}
fn failures() -> Check {
    {
        let (kind, capture) = ("text", "article/fulltext/synthetic-cut");
        let env = Environment::new()?;
        let folder = env.root().join("empty");
        fs::create_dir(&folder)?;
        let target = folder.join("never");
        let mut initial = args(kind, &target);
        *initial.get_mut(2).ok_or("identity")? = "PMC5606870".into();
        command(&env, capture, &initial, 4)?;
        assert_eq!(fs::read_dir(&folder)?.count(), 0);
        fs::write(&target, b"caller content")?;
        let mut replace = initial;
        replace.push("--replace".into());
        command(&env, capture, &replace, 4)?;
        assert_eq!(fs::read(&target)?, b"caller content");
        assert_eq!(fs::read_dir(folder)?.count(), 1);
    }
    Ok(())
}
fn http_refusal() -> Check {
    for (kind, capture) in [
        ("text", "article/fulltext/live-introduction"),
        ("supplement", "article/supplements/live"),
    ] {
        let env = Environment::new()?;
        let path = env.root().join("server-must-not-write");
        let replay = env.root().join("metadata-only");
        super::response_cases::copy_capture(capture, &replay, |_, _| {})?;
        for source in fs::read_dir(&replay)? {
            for file in fs::read_dir(source?.path())? {
                let path = file?.path();
                if path.extension().is_some_and(|v| v == "bin") {
                    fs::remove_file(path)?;
                }
            }
        }
        let mut server = super::http::Server::start_replay(replay)?;
        let call = super::mcp::request(
            json!(7),
            "tools/call",
            json!({"name":"biomcp","arguments":{"args":args(kind, &path)}}),
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
                .is_some_and(|s| s.starts_with("https://pmc-oa-opendata.s3.amazonaws.com/"))
        );
        assert!(!path.exists());
        server.stop()?;
    }
    Ok(())
}

fn pdf() -> Check {
    let env = Environment::new()?;
    let replay = fixtures().join("replies/article/saves/pdf");
    let mut original = None;
    for entry in fs::read_dir(replay.join("pmc-oa"))? {
        let path = entry?.path();
        if path.extension().is_some_and(|v| v == "bin") {
            original = Some(fs::read(path)?);
        }
    }
    let original = original.ok_or("recorded source PDF absent")?;
    assert!(original.starts_with(b"%PDF-"));
    let folder = env.root().join("pdf");
    fs::create_dir(&folder)?;
    let mut cli = args("supplement", &folder);
    *cli.get_mut(3).ok_or("file argument")? = "MGG3-5-553-s001.pdf".into();
    let value = command_at(&env, &replay, &cli, 0)?;
    let path = folder.join("MGG3-5-553-s001.pdf");
    receipt(&value, &path, &original, None)?;
    assert_eq!(fs::read(path)?, original);
    let stdio = folder.join("stdio.pdf");
    let mut cli = args("supplement", &stdio);
    *cli.get_mut(3).ok_or("file argument")? = "MGG3-5-553-s001.pdf".into();
    let value =
        super::mcp::call_with_globals(&env, &cli, false, &env.globals(Some(&replay), false))?;
    receipt(&value, &stdio, &original, None)?;
    assert_eq!(fs::read(stdio)?, original);
    Ok(())
}
fn opaque() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("opaque");
    // Preserve request identity and exact declared filename; edit only source MIME/body.
    super::response_cases::copy_capture("article/supplements/live", &replay, |_, record| {
        if let Some(Value::String(xml)) = record.pointer_mut("/exchanges/0/body/value") {
            *xml = xml.replace(
                "xlink:href=\"MGG3-5-553-s002.xlsx\"",
                "mimetype=\"application\" mime-subtype=\"vnd.openxmlformats-officedocument.wordprocessingml.document\" xlink:href=\"MGG3-5-553-s002.xlsx\"",
            );
        }
    })?;
    let bytes = b"PK\x03\x04opaque Word document, no extraction";
    fs::write(
        replay
            .join("pmc-oa/51c9ef213345bdbc7ed493ea19928bd900d5791413a6c5a65b60f2c50eac690f-0.bin"),
        bytes,
    )?;
    let reader = vec![
        "article".into(),
        "supplement".into(),
        "28944238".into(),
        "MGG3-5-553-s002.xlsx".into(),
    ];
    let unavailable = command_at(&env, &replay, &reader, 0)?;
    assert_eq!(
        unavailable.pointer("/result/content/data/reason"),
        Some(&json!("unsupported"))
    );
    let path = env.root().join("opaque.docx");
    let value = command_at(&env, &replay, &args("supplement", &path), 0)?;
    receipt(&value, &path, bytes, None)?;
    assert_eq!(fs::read(path)?, bytes);
    Ok(())
}
fn arguments() -> Check {
    let env = Environment::new()?;
    for kind in ["text", "supplement"] {
        for extra in [
            vec!["--find", "literal"],
            vec!["--variant", "E17K"],
            vec!["--offset", "1"],
        ] {
            let path = env.root().join("never");
            let mut cli = args(kind, &path);
            cli.extend(extra.iter().map(|s| (*s).to_owned()));
            command(&env, "article/supplements/live", &cli, 2)?;
            assert!(!path.exists());
        }
    }
    Ok(())
}

fn cut_reply(request: &str) -> Result<String, String> {
    if request.contains(".xlsx?") {
        return Ok("PK\x03".into());
    }
    let target = request.split_whitespace().nth(1).ok_or("request target")?;
    for file in fs::read_dir(fixtures().join("replies/article/supplements/live/pmc-oa"))
        .map_err(|e| e.to_string())?
    {
        let path = file.map_err(|e| e.to_string())?.path();
        if path.extension().is_none_or(|v| v != "json") {
            continue;
        }
        let record: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if record
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| url.ends_with(target))
        {
            let body = record.pointer("/exchanges/0/body/value").ok_or("body")?;
            return Ok(body
                .as_str()
                .map_or_else(|| body.to_string(), str::to_owned));
        }
    }
    Err(format!("unrecorded local request {target}"))
}
fn cut_body() -> Check {
    let stub = super::network_stub::Stub::with_length(
        "pmc-oa-opendata.s3.amazonaws.com",
        |request| {
            if request.contains("list-type=") {
                "application/xml"
            } else {
                "binary/octet-stream"
            }
        },
        cut_reply,
        |request, bytes| {
            if request.contains(".xlsx?") {
                bytes + 100
            } else {
                bytes
            }
        },
    )?;
    let env = Environment::new()?;
    let ca = env.root().join("source.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate)?;
    env.config(&json!({"ca_bundle":ca, "proxy":format!("http://{}",stub.address)}))?;
    let folder = env.root().join("cut");
    fs::create_dir(&folder)?;
    let path = folder.join("never.xlsx");
    let mut failures = Vec::new();
    for replace in [false, true] {
        if replace {
            fs::write(&path, b"caller content")?;
        }
        let mut cli = args("supplement", &path);
        *cli.get_mut(2).ok_or("identity")? = "PMC5606870".into();
        if replace {
            cli.push("--replace".into());
        }
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &cli).output()?;
        assert_eq!(
            output.status.code(),
            Some(4),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        failures.push(String::from_utf8_lossy(&output.stdout).into_owned());
        if replace {
            assert_eq!(fs::read(&path)?, b"caller content");
            assert_eq!(fs::read_dir(&folder)?.count(), 1);
        } else {
            assert_eq!(fs::read_dir(&folder)?.count(), 0);
        }
    }
    let requests = stub.finish()?;
    assert!(
        requests
            .iter()
            .any(|(_, request)| request.contains(".xlsx?")),
        "requests={requests:?}; replies={failures:?}"
    );
    Ok(())
}

fn preprint() -> Check {
    let env = Environment::new()?;
    let folder = env.root().join("preprints");
    fs::create_dir(&folder)?;
    let mut cli = args("text", &folder);
    *cli.get_mut(2).ok_or("identity")? = "PPR1295425".into();
    let value = command(&env, "article/preprint/live", &cli, 0)?;
    let text = fs::read_to_string(folder.join("PPR1295425.md"))?;
    assert!(text.contains("ID: PPR1295425"));
    assert!(text.contains("License: cc_by"));
    assert!(text.contains("Introduction"));
    assert_eq!(
        value.pointer("/result/content/data/saved/url"),
        Some(&json!(
            "https://www.ebi.ac.uk/europepmc/webservices/rest/PPR1295425/fullTextXML"
        ))
    );
    assert!(
        value
            .pointer("/result/content/data/saved/parts")
            .and_then(Value::as_u64)
            .is_some_and(|n| n > 1)
    );
    Ok(())
}

// Explicit saves must neither supply nor consume the row reader's reply cache.
fn cache_boundary() -> Check {
    let stub = super::network_stub::Stub::with_length(
        "pmc-oa-opendata.s3.amazonaws.com",
        |request| {
            if request.contains("list-type=") {
                "application/xml"
            } else {
                "binary/octet-stream"
            }
        },
        csv_reply,
        |_, bytes| bytes,
    )?;
    for reader_first in [false, true] {
        let env = Environment::new()?;
        let ca = env.root().join("source.pem");
        stub.retain(&env);
        fs::write(&ca, &stub.certificate)?;
        env.config(&json!({"ca_bundle":ca, "proxy":format!("http://{}",stub.address)}))?;
        let path = env.root().join("original.csv");
        let mut save = args("supplement", &path);
        *save.get_mut(2).ok_or("identity")? = "PMC5606870".into();
        let reader = save.get(..4).ok_or("reader arguments")?.to_vec();
        let initial = if reader_first { &reader } else { &save };
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), initial)
            .output()?;
        assert_eq!(
            output.status.code(),
            Some(0),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        if !reader_first {
            assert_eq!(fs::read(&path)?, b"gene,value\nAKT1,1\n");
            fs::remove_file(&path)?;
        }
        let mut offline = vec!["--offline".into()];
        offline.extend(if reader_first { save } else { reader });
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &offline)
            .output()?;
        assert_eq!(
            output.status.code(),
            Some(4),
            "raw save crossed the reply cache boundary (reader_first={reader_first}): {}",
            String::from_utf8_lossy(&output.stdout)
        );
        let value: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(
            value.pointer("/result/content/reason"),
            Some(&json!("offline"))
        );
        assert!(!path.exists());
    }
    assert!(
        stub.finish()?
            .iter()
            .any(|(_, request)| request.contains(".xlsx?"))
    );
    Ok(())
}
fn csv_reply(request: &str) -> Result<String, String> {
    if request.contains(".xlsx?") {
        return Ok("gene,value\nAKT1,1\n".into());
    }
    Ok(cut_reply(request)?.replace(
        "xlink:href=\"MGG3-5-553-s002.xlsx\"",
        "mimetype=\"text\" mime-subtype=\"csv\" xlink:href=\"MGG3-5-553-s002.xlsx\"",
    ))
}
