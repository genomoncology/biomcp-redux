//! Feature drug-recall: replay bounded publisher captures without committing bodies.
use super::{configuration::invoke, maude::environment, mcp, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};
pub(super) fn trials() -> Vec<Trial> {
    let Some(root) = std::env::var_os("BIOMCP_DRUG_RECALL_CAPTURE_ROOT") else {
        return Vec::new();
    };
    (0..7)
        .map(|n| {
            let root = PathBuf::from(&root);
            transcript_trial(format!("drug-recall/retained-publisher-{n}"), move || {
                retained(&root, n).map_err(Into::into)
            })
        })
        .collect()
}
fn retained(root: &Path, n: usize) -> Result<(), String> {
    let (body, expected, request_line) = input(root, n)?;
    let stub = Stub::for_source("api.fda.gov", "application/json", move |request| {
        if request.lines().next() != Some(request_line.as_str()) {
            return Err(format!("retained request changed: {request}"));
        }
        Ok(body.clone())
    })?;
    let env = environment(&stub)?;
    let mut args = match n {
        0 => vec!["search", "drug", "metformin"],
        1 => vec!["search", "drug", "--firm", "Apotex"],
        2 | 5 => vec!["search", "drug", "--classification", "Class II"],
        3 => vec!["search", "drug", "--reason", "Lack of Sterility Assurance"],
        4 => vec![
            "search",
            "drug",
            "--initiated-from",
            "2024-01-01",
            "--initiated-to",
            "2024-12-31",
        ],
        _ => vec![
            "get",
            "drug",
            expected.as_str().ok_or("invalid native recall number")?,
        ],
    }
    .into_iter()
    .map(str::to_owned)
    .collect::<Vec<_>>();
    args.extend(["--source".into(), "fda-drug-recall".into()]);
    if n < 6 {
        args.extend(["--limit".into(), "1".into()]);
    }
    if n == 5 {
        args.extend(["--offset".into(), "1".into()]);
    }
    let cli = invoke(&env, args.clone(), 0)?;
    let tool = mcp::call_with_globals(&env, &args, false, &[])?;
    let path = if n < 6 {
        "/result/items/0/recall_number"
    } else {
        "/result/header/recall_number"
    };
    if cli != tool || cli.pointer(path) != Some(&expected) {
        return Err(format!(
            "retained recall projection differs: {cli} / {tool}"
        ));
    }
    if stub.finish()?.len() != 1 {
        return Err("retained warm call repeated HTTP".into());
    }
    Ok(())
}
use sha2::Digest;

fn input(root: &Path, n: usize) -> Result<(String, Value, String), String> {
    let (folder, index) = if n == 6 {
        ("get-capture", 0)
    } else {
        ("search-capture", n)
    };
    let directory = root.join(folder);
    let body =
        std::fs::read(directory.join(format!("{index:04}.body"))).map_err(|e| e.to_string())?;
    let receipt: Value = serde_json::from_slice(
        &std::fs::read(directory.join(format!("{index:04}.receipt.json")))
            .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let hash = sha2::Sha256::digest(&body)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect::<String>();
    if receipt.get("body_sha256") != Some(&json!(hash)) {
        return Err(format!("retained body digest changed: {receipt}"));
    }
    let raw: Value = serde_json::from_slice(&body).map_err(|e| e.to_string())?;
    let expected = raw
        .pointer("/results/0/recall_number")
        .ok_or("capture has no recall")?
        .clone();
    let body = String::from_utf8(body).map_err(|e| e.to_string())?;
    let plan: Value = serde_json::from_slice(
        &std::fs::read(directory.join("plan.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let url = plan
        .pointer(&format!("/requests/{index}/url"))
        .and_then(Value::as_str)
        .ok_or("capture URL missing")?;
    let path = url
        .strip_prefix("https://api.fda.gov")
        .ok_or("capture host changed")?;
    let request_line = format!("GET {} HTTP/1.1", path.replace('+', "%20"));
    Ok((body, expected, request_line))
}
