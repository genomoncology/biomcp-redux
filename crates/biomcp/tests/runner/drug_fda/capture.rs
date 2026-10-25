//! Optional retained-body replay; publisher bodies remain outside Git.
use super::super::configuration::invoke;
use super::{Check, both};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
pub(super) fn trials() -> Vec<Trial> {
    let Some(root) = std::env::var_os("BIOMCP_FDA_CAPTURE_ROOT") else {
        return Vec::new();
    };
    (0..15)
        .map(|n| {
            let root = PathBuf::from(&root);
            Trial::test(format!("drug-approvals/native-retained-{n}"), move || {
                native(&root, n).map_err(Into::into)
            })
        })
        .collect()
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str, String> {
    v.get(k)
        .and_then(Value::as_str)
        .ok_or_else(|| format!("missing {k}"))
}
fn entries(path: &Path) -> Result<Vec<Value>, String> {
    serde_json::from_slice(&std::fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())
}
fn input(root: &Path, n: usize) -> Result<(Value, String), String> {
    let (entry, path) = if n < 3 {
        let entry = entries(&root.join("ledger.json"))?
            .get(n)
            .cloned()
            .ok_or("new entry")?;
        let path = root.join(field(&entry, "body_file")?);
        (entry, path)
    } else {
        let entry = entries(&root.join("retained/retained.json"))?
            .into_iter()
            .filter(|e| e.get("source") == Some(&json!("drugsfda")))
            .nth(n - 3)
            .ok_or("retained entry")?;
        let path = root.join("retained").join(field(&entry, "file")?);
        (entry, path)
    };
    let bytes = std::fs::read(path).map_err(|e| e.to_string())?;
    let hash = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if hash != field(&entry, "sha256")? {
        return Err("native source body hash differs".into());
    }
    Ok((entry, String::from_utf8(bytes).map_err(|e| e.to_string())?))
}
fn native(root: &Path, n: usize) -> Check {
    let (entry, raw) = input(root, n)?;
    let url = field(&entry, "url")?;
    let query = url.split_once("?search=").ok_or("native selector")?.1;
    let (selector, rest) = query.split_once("&limit=").ok_or("native limit")?;
    let (limit, offset) = rest.split_once("&skip=").unwrap_or((rest, "0"));
    let selector_text = decode(selector)?;
    let (key, text) = selector_text
        .split_once(':')
        .ok_or("native field selector")?;
    let text = text
        .strip_prefix('"')
        .and_then(|t| t.strip_suffix('"'))
        .ok_or("native literal")?;
    let status = entry
        .get("http_status")
        .or_else(|| entry.get("status"))
        .and_then(Value::as_u64)
        .ok_or("native status")? as u16;
    let (env, stub) = delivery(raw.clone(), url, status)?;
    let recording = env.root().join("recording");
    let mut args = if key == "application_number" {
        vec![
            "get".into(),
            "drug".into(),
            text.into(),
            "approvals".into(),
            "--source".into(),
            "drugsfda".into(),
        ]
    } else {
        vec![
            "search".into(),
            "drug".into(),
            text.into(),
            "--source".into(),
            "drugsfda".into(),
            "--limit".into(),
            limit.into(),
            "--offset".into(),
            offset.into(),
        ]
    };
    let exit = if status == 404 { 1 } else { 0 };
    let mut record = vec!["--record".into(), recording.to_string_lossy().into_owned()];
    record.extend(args.clone());
    invoke(&env, record, exit)?;
    if stub.finish()?.len() != 1 {
        return Err("native local delivery requested more than once".into());
    }
    let mut replay = vec!["--replay".into(), recording.to_string_lossy().into_owned()];
    replay.append(&mut args);
    let value = both(&env, replay.clone(), exit)?;
    let original: Value = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    verify(&original, &value, key)?;
    if key == "application_number" {
        walk(&env, replay, value, &original)?;
    }
    use std::io::Write;
    writeln!(std::io::stdout(),"FDA original_url={url} original_date={} original_sha256={} local_delivery=true fresh_provider_calls=0",field(&entry,"retrieved_at")?,field(&entry,"sha256")?).map_err(|e|e.to_string())
}
fn delivery(
    raw: String,
    url: &str,
    status: u16,
) -> Result<(super::Environment, super::Stub), String> {
    let selector = url.split_once("?search=").ok_or("URL")?.1;
    // Legacy capture URLs encode spaces as +; the shared GET encoder uses %20.
    // Preserve the original URL in provenance and identify this as a local delivery.
    let selector = selector.replace('+', "%20");
    let expected = format!("GET /drug/drugsfda.json?search={selector} HTTP/1.1");
    let stub = super::Stub::for_source_status(
        "api.fda.gov",
        "application/json",
        move |request| {
            if request.lines().next() != Some(expected.as_str()) {
                return Err("native request changed".into());
            }
            Ok(raw.clone())
        },
        move |_| status,
    )?;
    let env = super::Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    std::fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    Ok((env, stub))
}
fn verify(raw: &Value, value: &Value, key: &str) -> Check {
    let Some(rows) = raw.get("results").and_then(Value::as_array) else {
        return if value.get("status") == Some(&json!("absent")) {
            Ok(())
        } else {
            Err("native absence lost".into())
        };
    };
    if key == "application_number" {
        if value.pointer("/result/identity/id") != raw.pointer("/results/0/application_number")
            || value.pointer("/result/products") != raw.pointer("/results/0/products")
        {
            return Err(format!("native application products changed: {value}"));
        }
    } else {
        let ids = rows
            .iter()
            .map(|r| r.get("application_number").cloned().ok_or("id"))
            .collect::<Result<Vec<_>, _>>()?;
        let actual = value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .ok_or("candidates")?
            .iter()
            .map(|r| r.pointer("/identity/id").cloned().ok_or("id"))
            .collect::<Result<Vec<_>, _>>()?;
        if actual != ids || value.pointer("/result/total") != raw.pointer("/meta/results/total") {
            return Err("native candidate order or total changed".into());
        }
    }
    Ok(())
}
fn walk(env: &super::Environment, base: Vec<String>, mut page: Value, raw: &Value) -> Check {
    let expected = raw
        .pointer("/results/0/submissions")
        .and_then(Value::as_array)
        .ok_or("submissions")?;
    let mut seen = Vec::new();
    loop {
        let records = page
            .pointer("/result/approvals/data/records")
            .and_then(Value::as_array)
            .ok_or("history")?;
        for row in records {
            if row.pointer("/event/product_number").is_some() {
                return Err("submission acquired product applicability".into());
            }
            seen.push((
                row.pointer("/event/kind").cloned(),
                row.get("submission_number").cloned(),
            ));
        }
        let Some(args) = page
            .get("next")
            .and_then(Value::as_array)
            .and_then(|a| a.iter().find(|n| n.get("kind") == Some(&json!("page"))))
            .and_then(|n| n.get("args"))
            .and_then(Value::as_array)
        else {
            break;
        };
        let mut next = base.iter().take(2).cloned().collect::<Vec<_>>();
        next.extend(
            args.iter()
                .map(|v| v.as_str().map(str::to_owned).ok_or("arg"))
                .collect::<Result<Vec<_>, _>>()?,
        );
        page = both(env, next, 0)?;
        if seen.len() > expected.len() {
            return Err("history continuation looped".into());
        }
    }
    let expected = expected
        .iter()
        .map(|r| {
            (
                r.get("submission_type").cloned(),
                r.get("submission_number").cloned(),
            )
        })
        .collect::<Vec<_>>();
    if seen != expected {
        return Err("native history walk lost or reordered submissions".into());
    }
    Ok(())
}
fn decode(value: &str) -> Result<String, String> {
    let mut bytes = value.bytes();
    let mut out = Vec::new();
    while let Some(b) = bytes.next() {
        if b == b'%' {
            let a = bytes
                .next()
                .and_then(|b| (b as char).to_digit(16))
                .ok_or("escape")?;
            let b = bytes
                .next()
                .and_then(|b| (b as char).to_digit(16))
                .ok_or("escape")?;
            out.push((a * 16 + b) as u8);
        } else {
            out.push(if b == b'+' { b' ' } else { b });
        }
    }
    String::from_utf8(out).map_err(|e| e.to_string())
}
