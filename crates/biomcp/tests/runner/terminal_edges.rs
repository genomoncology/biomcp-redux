//! Feature markdown-view, network and MCP boundaries through isolated commands.
use super::{launch::Environment, network_stub::Stub, process::Process};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write, process::Stdio};
type Check = Result<(), Box<dyn std::error::Error>>;

pub(super) fn trials() -> Vec<Trial> {
    [
        ("markdown-controls", markdown as fn() -> Check),
        ("proxy-case-precedence", proxy),
        ("proxy-bypass-precedence", bypass),
        ("proxy-help", help),
        ("parsed-format", format),
        ("dispatch-diagnostic", dispatch),
        ("stdio-oversized-then-valid", framing),
    ]
    .into_iter()
    .map(|(name, check)| Trial::test(format!("terminal-edges/{name}"), move || check().map_err(|e| e.to_string().into())))
    .collect()
}

fn markdown() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("recordings");
    let source = "Synthetic\u{1b}[31m\u{0}\u{7}\u{9}\u{7f}\u{85}\u{202a}\u{202e}\u{202c}\u{2066}\u{2069} protein";
    super::response_cases::copy_capture("protein/synthetic", &replay, |_, record| {
        *record.pointer_mut("/exchanges/0/body/value/proteinDescription/recommendedName/fullName/value").unwrap() = json!(source);
    })?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(["get", "protein", "P00533"].map(str::to_owned));
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout)?;
    assert!(value.to_string().contains("Synthetic"));
    assert!(String::from_utf8(output.stdout)?.contains("\\u001b"));
    assert!(value.to_string().contains('\u{202e}'), "JSON must retain bidi data");
    args.extend(["--format", "md"].map(str::to_owned));
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &args).output()?;
    assert!(output.status.success());
    let text = String::from_utf8(output.stdout)?;
    for ch in source.chars().filter(|ch| ch.is_control() || matches!(ch, '\u{202a}'..='\u{202e}' | '\u{2066}'..='\u{2069}')) {
        assert!(!text.contains(ch), "raw control {ch:?}: {text:?}");
        assert!(text.contains(&format!("\\u{{{:04x}}}", u32::from(ch))), "missing visible escape {ch:?}: {text:?}");
    }
    Ok(())
}

fn proxy() -> Check {
    for (lower, upper, invalid) in [
        ("http_proxy", "HTTP_PROXY", "http://[invalid"),
        ("https_proxy", "HTTPS_PROXY", "http://[invalid"),
        ("no_proxy", "NO_PROXY", "bad:port"),
    ] {
        let env = Environment::new()?;
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &["health".into(), "--offline".into()])
            .env(lower, invalid).output()?;
        let value: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(output.status.code(), Some(5), "lower-case {lower} ignored: {value}");
        assert_eq!(value.pointer("/error/option"), Some(&json!(if cfg!(windows) { upper } else { lower })));
        assert!(output.stderr.is_empty());
    }
    // Invalid upper-case settings ensure the unfixed program cannot reach a provider.
    // Windows has case-insensitive names, so it cannot hold this conflicting pair.
    if !cfg!(windows) {
        let env = Environment::new()?;
        let stub = Stub::new()?;
        stub.retain(&env);
        let ca = env.root().join("ca.pem");
        fs::write(&ca, &stub.certificate)?;
        env.config(&json!({"ca_bundle":ca}))?;
        let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &["sync".into(), "hgnc".into()]);
        command.env("https_proxy", format!("http://{}", stub.address))
            .env("http_proxy", format!("http://{}", stub.address)).env("no_proxy", "unrelated.invalid");
        command.env("HTTP_PROXY", "http://[invalid-upper").env("HTTPS_PROXY", "http://[invalid-upper").env("NO_PROXY", "*");
        let output = command.output()?;
        let value: Value = serde_json::from_slice(&output.stdout)?;
        assert!(output.status.success(), "{value}");
        assert!(output.stderr.is_empty());
        assert_eq!(value.pointer("/result/rows"), Some(&json!(176)));
        assert_eq!(stub.finish()?.len(), 2);
    }
    if cfg!(windows) {
        return Ok(());
    }
    // Empty lower-case values override upper-case settings, too. No network is needed.
    let env = Environment::new()?;
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &["health".into(), "--offline".into()])
        .env("http_proxy", "").env("https_proxy", "").env("no_proxy", "")
        .env("HTTP_PROXY", "http://[invalid-upper").env("HTTPS_PROXY", "http://[invalid-upper").env("NO_PROXY", "bad:port")
        .output()?;
    assert!(output.status.success(), "{}", String::from_utf8_lossy(&output.stdout));
    Ok(())
}

fn help() -> Check {
    let env = Environment::new()?;
    let value = super::configuration::invoke(&env, vec!["help".into(), "network".into(), "proxy".into()], 0)?;
    let text = value.pointer("/result/commands/0/description").and_then(Value::as_str).ok_or("missing proxy help")?;
    for name in ["http_proxy", "HTTP_PROXY", "https_proxy", "HTTPS_PROXY", "no_proxy", "NO_PROXY"] {
        assert!(text.contains(name), "missing {name}: {text}");
    }
    assert!(text.contains("lower case"));
    assert!(text.contains("Publisher PDF downloads are refused when a proxy applies"));
    Ok(())
}

fn bypass() -> Check {
    for (bypass, dual) in [(false, false), (false, true), (true, true)] {
        let env = Environment::new()?;
        let target = env.root().join("article.pdf");
        let mut args = env.globals(Some(&super::launch::fixtures().join("replies/article/pdf/synthetic")), false);
        args.extend(["article".into(), "pdf".into(), "41351423".into(), "--save".into(), target.to_string_lossy().into()]);
        let mut command = env.command(env!("CARGO_BIN_EXE_biomcp"), &args);
        command.env("https_proxy", "http://127.0.0.1:9")
            .env("no_proxy", if bypass { "storage.imrpress.com" } else { "unrelated.invalid" });
        if dual && !cfg!(windows) {
            command.env("HTTPS_PROXY", "http://127.0.0.1:9")
                .env("NO_PROXY", if bypass { "unrelated.invalid" } else { "*" });
        }
        let output = command.output()?;
        let value: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(output.status.code(), Some(if bypass { 0 } else { 4 }), "{value}");
        assert!(output.stderr.is_empty());
        if bypass {
            assert!(fs::read(&target)?.starts_with(b"%PDF-"));
        } else {
            assert_eq!(value.pointer("/result/file/reason"), Some(&json!("proxy")));
            assert!(!target.exists());
        }
    }
    Ok(())
}

fn format() -> Check {
    let env = Environment::new()?;
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &[
        "get".into(), "gene".into(), "--help".into(), "--format".into(), "md".into(),
    ]).output()?;
    assert!(output.status.success());
    let value: Value = serde_json::from_slice(&output.stdout)?;
    assert_eq!(value.get("schema"), Some(&json!("biomcp.help/3")));
    Ok(())
}

fn dispatch() -> Check {
    let env = Environment::new()?;
    let directory = env.root().join("config/biomcp");
    fs::create_dir(&directory)?;
    // Observation creation changes an initially absent config file before the request reads it.
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &[
        "get".into(), "gene".into(), "BRAF".into(), "--offline".into(),
        "--observe".into(), directory.join("config.json").to_string_lossy().into(),
    ]).output()?;
    assert_eq!(output.status.code(), Some(5));
    assert!(output.stdout.is_empty());
    assert_eq!(output.stderr, b"biomcp: local configuration failed\n");
    Ok(())
}

fn framing() -> Check {
    let env = Environment::new()?;
    let child = env.command(env!("CARGO_BIN_EXE_biomcp"), &["serve".into()])
        .stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::piped()).spawn()?;
    let mut process = Process::new(child)?;
    let mut stdin = process.child.stdin.take().ok_or("missing stdin")?;
    let mut input = vec![b'x'; 128 * 1024];
    input.push(b'\n');
    input.extend(serde_json::to_vec(&super::mcp::request(json!("after"), "ping", json!({})))?);
    input.push(b'\n');
    let sent = stdin.write_all(&input);
    drop(stdin);
    let (status, stdout, stderr) = process.finish()?;
    sent?;
    assert!(status.success());
    assert!(stderr.is_empty());
    let replies: Vec<Value> = stdout.split(|byte| *byte == b'\n').filter(|line| !line.is_empty())
        .map(serde_json::from_slice).collect::<Result<_, _>>()?;
    assert_eq!(replies.len(), 2, "{replies:?}");
    assert_eq!(replies[0].pointer("/error/code"), Some(&json!(-32600)));
    assert_eq!(replies[1].get("id"), Some(&json!("after")));
    assert_eq!(replies[1].get("result"), Some(&json!({})));
    Ok(())
}
