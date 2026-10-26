//! Feature study-files: references retain native blob facts and publisher notices.
use super::{call, rows, seed};
use crate::runner::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let native = crate::runner::launch::fixtures().join("study-files-cases");
    let available = native.is_dir();
    vec![
        Trial::test("study-files/retained-native", move || {
            retained(&native).map_err(Into::into)
        })
        .with_ignored_flag(!available),
        Trial::test("study-files/complete-walk-and-notice", || {
            pages().map_err(Into::into)
        }),
        Trial::test("study-files/absence-failure-policy", || {
            failures().map_err(Into::into)
        }),
    ]
}
fn entries() -> Value {
    json!((0..12).map(|n| {
        let name = if n == 11 { "LICENSE".into() } else { format!("data_{n}.txt") };
        json!({"name":name,"path":format!("public/synthetic_0/{name}"),"html_url":format!("https://github.com/cBioPortal/datahub/blob/master/public/synthetic_0/{name}"),"download_url":format!("https://raw.githubusercontent.com/cBioPortal/datahub/master/public/synthetic_0/{name}"),"sha":"0123456789012345678901234567890123456789","size":131,"type":"file"})
    }).collect::<Vec<_>>())
}
fn record(replay: &Path, body: Value, status: u16, delay: u64) -> Check {
    let url = "https://api.github.com/repos/cBioPortal/datahub/contents/public/synthetic_0";
    let key = Sha256::digest([b"GET\0".as_slice(), url.as_bytes()].concat())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    let exchange = json!({"status":status,"fetched":42,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text","value":body.to_string()}});
    let exchanges = if matches!(status, 429 | 500) {
        vec![exchange; 3]
    } else {
        vec![exchange]
    };
    let value = json!({"source":"cbioportal","method":"GET","url":url,"exchanges":exchanges});
    fs::write(
        replay.join("cbioportal").join(format!("{key}.json")),
        value.to_string(),
    )
    .map_err(|e| e.to_string())
}
fn pages() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    let cold = call(&env, &replay, &["get", "study", "synthetic_0"], 0)?;
    record(&replay, entries(), 200, 0)?;
    let mut seen = std::collections::BTreeSet::new();
    for offset in [0, 5, 10] {
        let value = call(
            &env,
            &replay,
            &[
                "get",
                "study",
                "synthetic_0",
                "files",
                "--section-offset",
                &offset.to_string(),
            ],
            0,
        )?;
        check_page(&value, offset, &mut seen)?;
        let args = [
            "get",
            "study",
            "synthetic_0",
            "files",
            "--section-offset",
            &offset.to_string(),
        ]
        .map(str::to_owned);
        let mcp = crate::runner::mcp::call_with_globals(
            &env,
            &args,
            false,
            &env.globals(Some(&replay), false),
        )?;
        if value != mcp {
            return Err("CLI/MCP files differ".into());
        }
    }
    let card = call(&env, &replay, &["get", "study", "synthetic_0"], 0)?;
    if card != cold || card.pointer("/result/license_notice").is_some() || seen.len() != 12 {
        return Err("stored card changed or walk missing".into());
    }
    Ok(())
}
fn check_page(
    value: &Value,
    offset: usize,
    seen: &mut std::collections::BTreeSet<String>,
) -> Check {
    let items = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("file items")?;
    if value.pointer("/page/total") != Some(&json!(12)) || items.len() != (12 - offset).min(5) {
        return Err(format!("native page bounds lost: {value}"));
    }
    for row in items {
        let path = row
            .get("path")
            .and_then(Value::as_str)
            .ok_or("native path")?;
        if !seen.insert(path.to_owned())
            || row.get("repositoryBlobSize") != Some(&json!(131))
            || row.get("data_type").is_some()
        {
            return Err(format!("blob identity or size changed: {row}"));
        }
    }
    if value
        .pointer("/result/license_notice")
        .and_then(Value::as_str)
        != Some("https://github.com/cBioPortal/datahub/blob/master/public/synthetic_0/LICENSE")
    {
        return Err(format!("publisher notice missing: {value}"));
    }
    Ok(())
}
fn failures() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
    reject_scope(&env, &replay)?;
    record(&replay, json!({"message":"Not Found"}), 404, 0)?;
    let value = call(&env, &replay, &["get", "study", "synthetic_0", "files"], 1)?;
    if value.get("status") != Some(&json!("absent")) {
        return Err(format!("absent directory failed: {value}"));
    }
    call(&env, &replay, &["get", "study", "synthetic_0"], 0)?;
    for (status, delay) in [(429, 0), (500, 0), (200, 0), (200, 120001)] {
        record(&replay, json!({"broken":"body"}), status, delay)?;
        let value = call(&env, &replay, &["get", "study", "synthetic_0", "files"], 4)?;
        if value.pointer("/sources/0/use") != Some(&json!("restricted"))
            || value.get("result").is_some()
        {
            return Err(format!("failure became empty data: {value}"));
        }
    }
    for disabled in ["cbioportal", "restricted"] {
        record(&replay, entries(), 200, 0)?;
        let value = call(
            &env,
            &replay,
            &[
                "get",
                "study",
                "synthetic_0",
                "files",
                "--disable-sources",
                disabled,
            ],
            4,
        )?;
        if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("policy bypass: {value}"));
        }
    }
    Ok(())
}

fn reject_scope(env: &Environment, replay: &Path) -> Check {
    let mut wrong = entries();
    *wrong.pointer_mut("/0/path").ok_or("synthetic path")? = json!("public/other_study/data_0.txt");
    record(replay, wrong, 200, 0)?;
    let value = call(env, replay, &["get", "study", "synthetic_0", "files"], 4)?;
    if value.get("result").is_some() {
        return Err("wrong directory row was admitted".into());
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    let cached: i64 = db
        .query_row(
            "SELECT count(*) FROM reply WHERE url LIKE '%api.github.com/%'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| e.to_string())?;
    if cached != 0 {
        return Err("wrong directory reply entered cache".into());
    }
    Ok(())
}

fn retained(directory: &Path) -> Check {
    use std::io::Write;
    let registry = crate::runner::schema::Registry::embedded()?;
    let mut paths = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|entry| entry.map(|v| v.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    if paths.len() != 21 {
        return Err("native file case inventory differs".into());
    }
    for path in paths {
        let value = crate::runner::run_case(&path, &registry)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let comparable = crate::runner::source_terms::size(&value, "")?;
        writeln!(
            std::io::stdout(),
            "Study files native {} CLI={} MCP={} comparable={} estimated_tokens={}",
            path.file_stem().ok_or("case name")?.to_string_lossy(),
            full + 1,
            full,
            comparable,
            comparable.div_ceil(4)
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
