//! Feature study-get: exact stored cards and bounded source metadata without member IDs.
use super::{call, rows, seed};
use crate::runner::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("study-get/complete-pages-and-native-counts", || {
            pages().map_err(Into::into)
        }),
        Trial::test("study-get/identity-failures-and-policy", || {
            failures().map_err(Into::into)
        }),
        Trial::test("study-get/admission-and-absent-count", || {
            admission().map_err(Into::into)
        }),
    ]
}
fn profiles() -> Value {
    json!((0..17).map(|n| json!({"molecularProfileId":format!("native_profile_{n}"),"studyId":"synthetic_0","molecularAlterationType":"MRNA_EXPRESSION","datatype":"CONTINUOUS","name":"Synthetic profile","description":"Native profile description","patientLevel":false,"showProfileInAnalysisTab":true})).collect::<Vec<_>>())
}
fn lists() -> Value {
    json!((0..13).map(|n| json!({"sampleListId":format!("native_list_{n}"),"studyId":"synthetic_0","category":"all_cases_in_study","name":"Synthetic list","description":"Native list description","sampleCount":n})).collect::<Vec<_>>())
}
fn record(replay: &Path, section: &str, body: Value, status: u16) -> Check {
    recording(replay, section, body, status, 0)
}
fn recording(replay: &Path, section: &str, body: Value, status: u16, delay: u64) -> Check {
    let url =
        format!("https://www.cbioportal.org/api/studies/synthetic_0/{section}?projection=DETAILED");
    let key = Sha256::digest(url.as_bytes())
        .iter()
        .map(|v| format!("{v:02x}"))
        .collect::<String>();
    let exchange = json!({"status":status,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text","value":body.to_string()}});
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
    record(&replay, "molecular-profiles", profiles(), 200)?;
    record(&replay, "sample-lists", lists(), 200)?;
    for (section, total, id) in [
        ("profiles", 17, "molecularProfileId"),
        ("sample-lists", 13, "sampleListId"),
    ] {
        let mut count = 0;
        let mut seen = std::collections::BTreeSet::new();
        loop {
            let offset = count.to_string();
            let value = call(
                &env,
                &replay,
                &[
                    "get",
                    "study",
                    "synthetic_0",
                    section,
                    "--section-offset",
                    &offset,
                ],
                0,
            )?;
            let items = value
                .pointer("/result/items")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("missing section items: {value}"))?;
            if value.pointer("/page/total") != Some(&json!(total)) || items.len() > 5 {
                return Err(format!("native page bounds: {value}"));
            }
            for row in items {
                let identity = row.get(id).and_then(Value::as_str).ok_or("native ID")?;
                if !seen.insert(identity.to_owned())
                    || row.get("studyId") != Some(&json!("synthetic_0"))
                    || row.get("sampleIds").is_some()
                {
                    return Err("collection changed identity or exposed members".into());
                }
            }
            count += items.len();
            if count == total {
                break;
            }
            if items.is_empty() {
                return Err("page walk stopped early".into());
            }
        }
    }
    let all = call(&env, &replay, &["get", "study", "synthetic_0", "all"], 0)?;
    if all
        .pointer("/result/profiles/data/items")
        .and_then(Value::as_array)
        .map(Vec::len)
        != Some(5)
        || all
            .pointer("/result/sample_lists/data/items")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(5)
    {
        return Err(format!("all lost independent bounded sections: {all}"));
    }
    Ok(())
}
fn failures() -> Check {
    for section in ["profiles", "sample-lists"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env, rows(), 200)?;
        call(&env, &replay, &["sync", "cbioportal-studies"], 0)?;
        let route = if section == "profiles" {
            "molecular-profiles"
        } else {
            section
        };
        let mut body = if section == "profiles" {
            profiles()
        } else {
            lists()
        };
        *body.pointer_mut("/0/studyId").ok_or("scope")? = json!("other_study");
        record(&replay, route, body, 200)?;
        let value = call(&env, &replay, &["get", "study", "synthetic_0", section], 0)?;
        if value.get("status") != Some(&json!("partial"))
            || value
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(format!("wrong scope not disclosed: {value}"));
        }
        for disabled in ["cbioportal", "restricted"] {
            let value = call(
                &env,
                &replay,
                &[
                    "get",
                    "study",
                    "synthetic_0",
                    section,
                    "--disable-sources",
                    disabled,
                ],
                4,
            )?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            {
                return Err(format!("policy bypass: {value}"));
            }
        }
        for (status, delay) in [(429, 0), (500, 0), (200, 0), (200, 120001)] {
            recording(&replay, route, json!({"broken":"body"}), status, delay)?;
            let value = call(&env, &replay, &["get", "study", "synthetic_0", section], 4)?;
            if value.pointer("/sources/0/use") != Some(&json!("restricted"))
                || value.get("result").is_some()
            {
                return Err(format!(
                    "failure lost source terms or became empty page: {value}"
                ));
            }
        }
    }
    Ok(())
}

fn admission() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    let mut body = lists();
    let row = body
        .get_mut(0)
        .and_then(Value::as_object_mut)
        .ok_or("row")?;
    row.remove("sampleCount");
    row.insert("sampleIds".into(), json!([null,{"ignored":true}]));
    record(&replay, "sample-lists", body, 200)?;
    let value = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0", "sample-lists"],
        0,
    )?;
    if value.pointer("/result/items/0/sampleCount").is_some()
        || value.to_string().contains("sampleIds")
    {
        return Err(format!(
            "missing count inferred or members retained: {value}"
        ));
    }
    let mut body = lists();
    *body.pointer_mut("/0/description").ok_or("description")? = json!("x".repeat(1_526_432));
    record(&replay, "sample-lists", body, 200)?;
    let value = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0", "sample-lists"],
        4,
    )?;
    if value.get("result").is_some()
        || value.pointer("/problems/0/code") != Some(&json!("collection.incomplete"))
    {
        return Err(format!("over-bound response returned a cut page: {value}"));
    }
    Ok(())
}
