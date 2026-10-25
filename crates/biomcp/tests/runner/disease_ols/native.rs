//! Retained source bytes enter only this explicit external-evidence proof.
use super::{args, record, replay, setup};
use crate::runner::network_stub::Stub;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{collections::BTreeMap, fs, path::Path};
pub(in crate::runner) fn run(root: &Path) -> Result<(), String> {
    let requests = load(root)?;
    let mut replies = BTreeMap::new();
    for row in &requests {
        replies.insert(
            key(row["url"].as_str().ok_or("missing url")?),
            fs::read_to_string(root.join(row["local_body"].as_str().ok_or("missing body")?))
                .map_err(|e| e.to_string())?,
        );
    }
    let stub = Stub::for_source_status(
        "www.ebi.ac.uk",
        "application/json",
        move |request| {
            let path = request
                .lines()
                .next()
                .and_then(|s| s.split_whitespace().nth(1))
                .ok_or("missing target")?;
            replies
                .get(&key(path))
                .cloned()
                .ok_or_else(|| format!("unplanned source request: {path}"))
        },
        |_| 200,
    )?;
    let env = setup(&stub)?;
    let output = root.join("product-proof");
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    for (index, row) in requests.iter().enumerate() {
        let words = command(row)?;
        let directory = output.join(format!("record-{index}"));
        let refs = words.iter().map(String::as_str).collect::<Vec<_>>();
        let result = record(&env, &refs, &directory)?;
        restore_date(
            &directory,
            row["fetched"].as_u64().ok_or("missing original date")?,
        )?;
        let body: Value = serde_json::from_str(
            &fs::read_to_string(root.join(row["local_body"].as_str().ok_or("missing body")?))
                .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        let mut expected = serde_json::Map::new();
        expected.insert("/status".into(), json!("complete"));
        if let Some(response) = body.get("response") {
            expected.insert("/page/total".into(), response["numFound"].clone());
            expected.insert("/page/offset".into(), response["start"].clone());
            for (i, hit) in response["docs"]
                .as_array()
                .ok_or("missing docs")?
                .iter()
                .enumerate()
            {
                expected.insert(
                    format!("/result/items/{i}/concept/id"),
                    hit["obo_id"].clone(),
                );
            }
        } else {
            let term = &body["_embedded"]["terms"][0];
            expected.insert("/result/returned_id".into(), term["obo_id"].clone());
            expected.insert("/result/obsolete".into(), term["is_obsolete"].clone());
            if term["is_obsolete"] == true {
                expected.insert("/result/replaced_by".into(), Value::Null);
                expected.insert(
                    "/result/consider".into(),
                    json!(["MONDO:0800444", "MONDO:0800445"]),
                );
            }
        }
        let size = serde_json::to_vec(&result)
            .map_err(|e| e.to_string())?
            .len();
        let mut steps = vec![
            json!({"run":words,"mcp":words,"expect":expected,"max_bytes":(size*115).div_ceil(100)}),
        ];
        if row["url"]
            .as_str()
            .is_some_and(|s| s.contains("DOID%3A3910"))
        {
            for section in ["synonyms", "xrefs"] {
                for offset in [0, 2] {
                    let q = vec![
                        "get".to_owned(),
                        "disease".into(),
                        "DOID:3910".into(),
                        section.into(),
                        "--source".into(),
                        "ols".into(),
                        "--ontology".into(),
                        "doid".into(),
                        "--section-limit".into(),
                        "2".into(),
                        "--section-offset".into(),
                        offset.to_string(),
                    ];
                    steps.push(
                        json!({"run":q,"mcp":q,"expect":{"/page/offset":offset},"max_bytes":6000}),
                    );
                }
            }
        }
        replay(&directory, Value::Array(steps))?;
        fs::write(
            output.join(format!("output-{index}.json")),
            serde_json::to_vec_pretty(&result).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        eprintln!(
            "native disease proof {index}: {size} full JSON bytes; original {}",
            row["sha256"]
        );
    }
    if stub.finish()?.len() != requests.len() {
        return Err("unexpected source attempt count".into());
    }
    Ok(())
}
fn load(root: &Path) -> Result<Vec<Value>, String> {
    let rows: Vec<Value> = serde_json::from_slice(
        &fs::read(root.join("native-inputs.json")).map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    for row in &rows {
        let bytes = fs::read(root.join(row["local_body"].as_str().ok_or("missing body")?))
            .map_err(|e| e.to_string())?;
        let hash = format!("{:x}", Sha256::digest(bytes));
        if row["sha256"] != hash {
            return Err("retained source hash mismatch".into());
        }
    }
    Ok(rows)
}
fn key(url: &str) -> String {
    let path = url.strip_prefix("https://www.ebi.ac.uk").unwrap_or(url);
    let (path, query) = path.split_once('?').unwrap_or((path, ""));
    let mut fields = query.split('&').map(decode).collect::<Vec<_>>();
    fields.sort();
    format!("{path}?{}", fields.join("&"))
}
fn decode(value: &str) -> String {
    let mut out = Vec::new();
    let bytes = value.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'%' && i + 2 < bytes.len() {
            if let Ok(b) = u8::from_str_radix(&value[i + 1..i + 3], 16) {
                out.push(b);
                i += 3;
                continue;
            }
        }
        out.push(if bytes[i] == b'+' { b' ' } else { bytes[i] });
        i += 1;
    }
    String::from_utf8_lossy(&out).into()
}
fn command(row: &Value) -> Result<Vec<String>, String> {
    let url = row["url"].as_str().ok_or("missing url")?;
    let mut fields = BTreeMap::new();
    for pair in url.split_once('?').ok_or("missing query")?.1.split('&') {
        let (k, v) = pair.split_once('=').ok_or("missing value")?;
        fields.insert(k, decode(v));
    }
    if let Some(text) = fields.get("q") {
        Ok(vec![
            "search".into(),
            "disease".into(),
            text.clone(),
            "--ontology".into(),
            fields.get("ontology").ok_or("missing ontology")?.clone(),
            "--limit".into(),
            "5".into(),
            "--offset".into(),
            fields.get("start").ok_or("missing offset")?.clone(),
        ])
    } else {
        let ontology = url
            .split("/ontologies/")
            .nth(1)
            .and_then(|s| s.split('/').next())
            .ok_or("missing ontology")?;
        let mut q = args(&[
            "get",
            "disease",
            fields.get("obo_id").ok_or("missing id")?,
            "--source",
            "ols",
            "--ontology",
            ontology,
        ]);
        q.shrink_to_fit();
        Ok(q)
    }
}
fn restore_date(root: &Path, fetched: u64) -> Result<(), String> {
    for file in fs::read_dir(root.join("ols")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        if file.path().extension().is_some_and(|x| x == "json") {
            let mut value: Value =
                serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                    .map_err(|e| e.to_string())?;
            for exchange in value["exchanges"]
                .as_array_mut()
                .ok_or("missing exchanges")?
            {
                exchange["fetched"] = json!(fetched);
            }
            fs::write(
                file.path(),
                serde_json::to_vec(&value).map_err(|e| e.to_string())?,
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
