//! Source publications, explicit source-byte saves and independently complete section walks.
#![expect(
    clippy::indexing_slicing,
    reason = "Captured JSON regression witnesses fail at the named field"
)]
use super::{
    launch::{Environment, fixtures},
    response_cases::copy_capture,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), Box<dyn std::error::Error>>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("trial/documents/linked-reference-article", || {
            reference().map_err(|e| e.to_string().into())
        }),
        Trial::test("trial/documents/explicit-save-and-receipt", || {
            saves().map_err(|e| e.to_string().into())
        }),
        Trial::test("trial/documents/manifest-membership-and-failure", || {
            failures().map_err(|e| e.to_string().into())
        }),
        Trial::test("trial/documents/http-refuses-before-download", || {
            http().map_err(|e| e.to_string().into())
        }),
        Trial::test("trial/documents/all-section-continuations", || {
            continuations().map_err(|e| e.to_string().into())
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
    let value: Value = serde_json::from_slice(&output.stdout)?;
    super::schema::Registry::embedded()?.validate(&value, args, exit == 0)?;
    let ceiling = if args.get(3).is_some_and(|s| s == "references") {
        7808
    } else {
        6500
    };
    assert!(
        super::source_terms::raw_size(&output.stdout)? <= ceiling,
        "bounded document reply: {}",
        output.stdout.len()
    );
    Ok(value)
}
fn args(path: &Path) -> Vec<String> {
    vec![
        "trial".into(),
        "document".into(),
        "NCT04303780".into(),
        "SAP_001.pdf".into(),
        "--save".into(),
        path.to_string_lossy().into(),
    ]
}
fn saves() -> Check {
    let env = Environment::new()?;
    let replay = fixtures().join("replies/trial/documents/synthetic");
    let folder = env.root().join("caller");
    fs::create_dir(&folder)?;
    let path = folder.join("SAP_001.pdf");
    let out = command(&env, &replay, &args(&folder), 0)?;
    let bytes = fs::read(&path)?;
    assert_eq!(
        bytes,
        b"%PDF-1.7\nSynthetic document save witness; no source document text.\n"
    );
    let sha: String = Sha256::digest(&bytes)
        .iter()
        .map(|b| format!("{b:02x}"))
        .collect();
    assert_eq!(
        out.pointer("/result/file/data/path"),
        Some(&json!(path.canonicalize()?.to_string_lossy()))
    );
    assert_eq!(
        out.pointer("/result/file/data/bytes"),
        Some(&json!(bytes.len()))
    );
    assert_eq!(out.pointer("/result/file/data/sha256"), Some(&json!(sha)));
    assert_eq!(
        out.pointer("/result/file/data/document/license"),
        Some(&json!("other"))
    );
    assert_eq!(
        out.pointer("/result/file/data/document/url"),
        Some(&json!(
            "https://cdn.clinicaltrials.gov/large-docs/80/NCT04303780/SAP_001.pdf"
        ))
    );
    fs::write(&path, b"caller original")?;
    command(&env, &replay, &args(&folder), 2)?;
    assert_eq!(fs::read(&path)?, b"caller original");
    let mut replace = args(&folder);
    replace.push("--replace".into());
    command(&env, &replay, &replace, 0)?;
    assert_eq!(fs::read(&path)?, bytes);
    let stdio = folder.join("stdio.pdf");
    let out = super::mcp::call_with_globals(
        &env,
        &args(&stdio),
        false,
        &env.globals(Some(&replay), false),
    )?;
    assert_eq!(out.pointer("/result/file/data/sha256"), Some(&json!(sha)));
    assert_eq!(fs::read(&stdio)?, bytes);
    let offline_path = folder.join("offline.pdf");
    let mut offline = env.globals(None, true);
    offline.extend(args(&offline_path));
    let result = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &offline)
        .output()?;
    assert_eq!(result.status.code(), Some(4));
    assert!(!offline_path.exists());
    assert_eq!(fs::read_dir(folder)?.count(), 2);
    Ok(())
}
fn metadata_only(env: &Environment) -> Result<std::path::PathBuf, Box<dyn std::error::Error>> {
    let replay = env.root().join("manifest-only");
    copy_capture("trial/get/NCT04303780", &replay, |_, _| {})?;
    Ok(replay)
}
fn failures() -> Check {
    let env = Environment::new()?;
    let replay = metadata_only(&env)?;
    let path = env.root().join("never.pdf");
    for bad in ["absent.pdf", "../SAP_001.pdf", "https://evil.example/a.pdf"] {
        let mut a = args(&path);
        a[3] = bad.into();
        command(&env, &replay, &a, 2)?;
        assert!(!path.exists());
    }
    // No document recording is available: a source failure must preserve an existing destination.
    fs::write(&path, b"caller original")?;
    let mut replace = args(&path);
    replace.push("--replace".into());
    command(&env, &replay, &replace, 5)?;
    assert_eq!(fs::read(&path)?, b"caller original");
    let bad = env.root().join("not-pdf");
    copy_capture("trial/documents/synthetic", &bad, |_, _| {})?;
    for file in fs::read_dir(bad.join("ctgov"))? {
        let path = file?.path();
        if path.extension().is_some_and(|s| s == "bin") {
            fs::write(path, b"<html>source login</html>")?;
        }
    }
    let out = command(&env, &bad, &replace, 4)?;
    assert_eq!(out.pointer("/result/file/reason"), Some(&json!("not_pdf")));
    assert_eq!(fs::read(&path)?, b"caller original");
    let malformed = env.root().join("malformed-manifest");
    copy_capture("trial/get/NCT04303780", &malformed, |_, r| {
        if let Some(d) = r.pointer_mut(
            "/exchanges/0/body/value/documentSection/largeDocumentModule/largeDocs/0/filename",
        ) {
            *d = json!("../../evil.pdf");
        }
    })?;
    let fresh = Environment::new()?;
    let card_args = ["get", "trial", "NCT04303780", "documents"].map(str::to_owned);
    let card = command(&fresh, &malformed, &card_args, 0)?;
    assert_eq!(card.pointer("/status"), Some(&json!("partial")));
    assert_eq!(
        card.pointer("/result/items/0/card/nct_id"),
        Some(&json!("NCT04303780"))
    );
    let out = command(&fresh, &malformed, &args(&path), 4)?;
    assert_eq!(
        out.pointer("/result/file/reason"),
        Some(&json!("bad_reply"))
    );
    assert_eq!(
        out.pointer("/problems/0/field"),
        Some(&json!("largeDocs.filename"))
    );
    assert_eq!(fs::read(&path)?, b"caller original");
    Ok(())
}
fn http() -> Check {
    let env = Environment::new()?;
    let replay = metadata_only(&env)?;
    let path = env.root().join("server-never.pdf");
    let mut server = super::http::Server::start_replay(replay)?;
    let call = super::mcp::request(
        json!(7),
        "tools/call",
        json!({"name":"biomcp","arguments":{"args":args(&path)}}),
    );
    let (status, reply) = server.request("POST", "/mcp", &call.to_string())?;
    assert_eq!(status, 200);
    assert_eq!(reply.pointer("/result/isError"), Some(&json!(true)));
    assert_eq!(
        reply.pointer("/result/structuredContent/error/url"),
        Some(&json!(
            "https://cdn.clinicaltrials.gov/large-docs/80/NCT04303780/SAP_001.pdf"
        ))
    );
    assert!(!path.exists());
    server.stop()?;
    Ok(())
}
fn expanded(record: &mut Value) {
    let Some(body) = record.pointer_mut("/exchanges/0/body/value") else {
        return;
    };
    let Some(protocol) = body.get_mut("protocolSection") else {
        return;
    };
    for (pointer, field) in [
        ("/outcomesModule/primaryOutcomes", "measure"),
        ("/armsInterventionsModule/armGroups", "label"),
        ("/contactsLocationsModule/locations", "facility"),
        ("/referencesModule/references", "citation"),
    ] {
        if let Some(rows) = protocol.pointer_mut(pointer).and_then(Value::as_array_mut)
            && let Some(row) = rows.first().cloned()
        {
            *rows = (0..3)
                .map(|index| {
                    let mut row = row.clone();
                    row[field] = json!(format!("Synthetic row {index}"));
                    row
                })
                .collect();
        }
    }
    protocol["outcomesModule"]["secondaryOutcomes"] = json!([]);
    protocol["outcomesModule"]["otherOutcomes"] = json!([]);
    protocol["armsInterventionsModule"]["interventions"] = json!([]);
    protocol["contactsLocationsModule"]["overallOfficials"] = json!([]);
    protocol["contactsLocationsModule"]["centralContacts"] = json!([
        {"name":"Synthetic row 0","role":"CONTACT","phone":"000","email":"a@example.invalid"},
        {"name":"Synthetic row 1","role":"CONTACT","phone":"000","email":"a@example.invalid"},
        {"name":"Synthetic row 2","role":"CONTACT","phone":"000","email":"a@example.invalid"}
    ]);
    protocol["eligibilityModule"]["eligibilityCriteria"] =
        json!(" Ω source whitespace \n".repeat(800));
    if let Some(rows) = body
        .pointer_mut("/documentSection/largeDocumentModule/largeDocs")
        .and_then(Value::as_array_mut)
        && let Some(row) = rows.first().cloned()
    {
        *rows = (0..3)
            .map(|index| {
                let mut row = row.clone();
                row["filename"] = json!(format!("Synthetic_{index}.pdf"));
                row
            })
            .collect();
    }
}
fn continuation_args(q: &Value) -> Result<Vec<String>, Box<dyn std::error::Error>> {
    Ok(vec![
        "get".into(),
        "trial".into(),
        q["input"][0].as_str().ok_or("identity")?.into(),
        q["section"][0].as_str().ok_or("section")?.into(),
        "--pool".into(),
        q["pool"].as_str().ok_or("pool")?.into(),
        "--offset".into(),
        q["offset"].to_string(),
        "--limit".into(),
        q["limit"].to_string(),
    ])
}
fn continuations() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("expanded");
    copy_capture("trial/get/NCT03511664", &replay, |_, r| expanded(r))?;
    let args = ["get", "trial", "NCT03511664", "all", "--limit", "1"].map(str::to_owned);
    let mut full = env.globals(Some(&replay), false);
    full.extend_from_slice(&args);
    let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &full).output()?;
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stdout)
    );
    let first: Value = serde_json::from_slice(&output.stdout)?;
    let mcp =
        super::mcp::call_with_globals(&env, &args, false, &env.globals(Some(&replay), false))?;
    assert_eq!(first, mcp);
    for name in [
        "eligibility",
        "outcomes",
        "arms",
        "locations",
        "contacts",
        "references",
        "documents",
    ] {
        walk_section(name, &first, &env)?;
    }
    Ok(())
}

fn walk_section(name: &str, first: &Value, env: &Environment) -> Check {
    let mut data = first["result"]["items"][0]["sections"][name]["data"].clone();
    assert!(
        data.get("continuation").is_some(),
        "{name} did not truncate in the witness"
    );
    let mut count = if name == "eligibility" {
        data["criteria"].as_str().ok_or("criteria")?.chars().count()
    } else {
        data["items"].as_array().ok_or("items")?.len()
    };
    let mut text = data["criteria"].as_str().unwrap_or_default().to_owned();
    let mut recovered = data["items"].as_array().cloned().unwrap_or_default();
    let total = data[if name == "eligibility" {
        "total_characters"
    } else {
        "total"
    }]
    .as_u64()
    .ok_or("total")? as usize;
    for _ in 0..20 {
        let Some(next) = data.get("continuation") else {
            break;
        };
        let args = continuation_args(next)?;
        let offline = env.globals(None, true);
        let mut full = offline.clone();
        full.extend_from_slice(&args);
        let output = env.command(env!("CARGO_BIN_EXE_biomcp"), &full).output()?;
        assert!(
            output.status.success(),
            "{}",
            String::from_utf8_lossy(&output.stdout)
        );
        let out: Value = serde_json::from_slice(&output.stdout)?;
        assert_eq!(
            out,
            super::mcp::call_with_globals(env, &args, false, &offline)?
        );
        super::schema::Registry::embedded()?.validate(&out, &args, true)?;
        data = out["result"]["items"][0]["sections"][name]["data"].clone();
        assert_eq!(data["offset"], json!(count));
        if name == "eligibility" {
            let s = data["criteria"].as_str().ok_or("criteria")?;
            count += s.chars().count();
            text.push_str(s);
        } else {
            let rows = data["items"].as_array().ok_or("items")?;
            count += rows.len();
            recovered.extend_from_slice(rows);
        }
    }
    assert_eq!(count, total, "{name} stranded source content");
    if name == "eligibility" {
        assert_eq!(text, " Ω source whitespace \n".repeat(800));
    } else {
        ordered_records(name, &recovered)?;
    }

    Ok(())
}

fn reference() -> Check {
    let env = Environment::new()?;
    let replay = env.root().join("reference-link");
    copy_capture("trial/get/NCT03511664", &replay, |_, _| {})?;
    copy_capture("trial/references-article", &replay, |_, _| {})?;
    let args = ["get", "trial", "NCT03511664", "references"].map(str::to_owned);
    let out = command(&env, &replay, &args, 0)?;
    let row = &out["result"]["items"][0]["sections"]["references"]["data"]["items"][5];
    assert_eq!(row["pmid"], json!("34161051"));
    assert_eq!(row["kind"], json!("RESULT"));
    let step = out["next"]
        .as_array()
        .ok_or("next")?
        .iter()
        .find(|s| {
            s["args"]
                .as_array()
                .is_some_and(|a| a.get(1) == Some(&json!("article")))
        })
        .ok_or("linked article step")?;
    let args = step["args"].as_array().ok_or("args")?;
    assert!(args.contains(&row["pmid"]));
    // Select this source PMID from the current-page batch; no reference is reclassified as primary.
    let selected = vec![
        args[0].as_str().ok_or("verb")?.to_owned(),
        args[1].as_str().ok_or("entity")?.to_owned(),
        row["pmid"].as_str().ok_or("pmid")?.to_owned(),
    ];
    let article = command(&env, &replay, &selected, 0)?;
    assert_eq!(
        article.pointer("/result/items/0/answer/card/pmid"),
        Some(&row["pmid"])
    );
    assert_eq!(
        article.pointer("/result/items/0/answer/card/doi"),
        Some(&json!("10.1056/NEJMoa2107322"))
    );
    assert_eq!(
        article,
        super::mcp::call_with_globals(&env, &selected, false, &env.globals(Some(&replay), false))?
    );
    Ok(())
}

// Distinct source rows expose repeats, gaps, reordering, and changes to attached fields.
fn ordered_records(name: &str, rows: &[Value]) -> Check {
    let field = match name {
        "outcomes" => "/measure",
        "arms" => "/data/label",
        "locations" => "/facility",
        "contacts" => "/data/name",
        "references" => "/citation",
        "documents" => "/filename",
        _ => return Err("unexpected records section".into()),
    };
    assert_eq!(rows.len(), 3, "{name}");
    let first = rows.first().ok_or("first source record")?;
    for (index, row) in rows.iter().enumerate() {
        let mut expected = first.clone();
        let marker = if name == "documents" {
            format!("Synthetic_{index}.pdf")
        } else {
            format!("Synthetic row {index}")
        };
        *expected.pointer_mut(field).ok_or("source field")? = json!(marker);
        if name == "documents" {
            expected["url"] = json!(format!(
                "https://cdn.clinicaltrials.gov/large-docs/64/NCT03511664/{marker}"
            ));
        }
        assert_eq!(row, &expected, "{name} source record {index}");
    }
    Ok(())
}
