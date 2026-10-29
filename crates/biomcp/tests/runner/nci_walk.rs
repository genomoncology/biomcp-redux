//! Native NCI record walks retain every source row and every original text character.
#![expect(
    clippy::indexing_slicing,
    reason = "Captured source and checked JSON shapes fail directly on drift"
)]
use super::{launch::Environment, nci::invoke, response_cases::copy_capture};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("trial/nci/exact-source-identity", || {
            identity().map_err(Into::into)
        }),
        transcript_trial("trial/nci/native-section-walks", || {
            walks(false).map_err(Into::into)
        }),
        transcript_trial("trial/nci/oversized-record-and-summary", || {
            walks(true).map_err(Into::into)
        }),
    ]
}
fn source() -> Result<Value, String> {
    let dir = super::launch::fixtures().join("replies/trial/nci/get/nci");
    let entry = fs::read_dir(dir)
        .map_err(|e| e.to_string())?
        .next()
        .ok_or("missing NCI capture")?
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&fs::read(entry.path()).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    Ok(value["exchanges"][0]["body"]["value"].clone())
}
fn walks(oversized: bool) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let dir = env.root().join("replies");
    let mut raw = source()?;
    if oversized {
        raw["brief_summary"] = json!(" \n Ω\\\" \t".repeat(4000));
        raw["eligibility"]["unstructured"][0]["description"] = json!("  Ω\n".repeat(3000));
    }
    copy_capture("trial/nci/get", &dir, |_, record| {
        record["exchanges"][0]["body"]["value"] = raw.clone()
    })
    .map_err(|e| e.to_string())?;
    let globals = env.globals(Some(&dir), false);
    for section in ["eligibility", "outcomes", "arms", "diseases", "summary"] {
        walk(&env, &raw, section, &globals)?;
    }
    Ok(())
}
fn walk(env: &Environment, raw: &Value, section: &str, globals: &[String]) -> Check {
    let mut args = selector(section);
    let mut rows = Vec::new();
    let mut text = String::new();
    let mut finished = false;
    for page in 0..100 {
        let mut command = args.clone();
        command.extend(if page == 0 {
            globals.to_vec()
        } else {
            env.globals(None, true)
        });
        let output = invoke(env, &command, 0)?;
        if output.pointer("/query/source") != Some(&json!("nci")) {
            return Err("NCI continuation lost selected source".into());
        }
        let data = &output["result"]["items"][0]["sections"][section]["data"];
        if section == "summary" {
            if data["offset"] != json!(text.chars().count()) {
                return Err("NCI text skipped or repeated characters".into());
            }
            text.push_str(data["text"].as_str().ok_or("summary fragment missing")?);
            if super::source_terms::size(&output, "")? > 16000 {
                return Err("summary exceeded bounded card plus character page".into());
            }
        } else {
            let records = if section == "eligibility" {
                &data["unstructured"]
            } else {
                data
            };
            if records["offset"] != json!(rows.len()) {
                return Err("NCI walk skipped or repeated rows".into());
            }
            rows.extend(
                records["items"]
                    .as_array()
                    .ok_or("NCI records missing")?
                    .iter()
                    .cloned(),
            );
            if section == "eligibility" && data["structured"] != raw["eligibility"]["structured"] {
                return Err("native age/sex fields changed".into());
            }
        }
        match super::action(&output, "page") {
            Ok(next) => args = next,
            Err(_) => {
                finished = true;
                break;
            }
        }
    }
    if !finished {
        return Err("bounded NCI walk did not finish".into());
    }
    if section == "summary" {
        if Some(text.as_str()) != raw["brief_summary"].as_str() {
            return Err("NCI summary lost original whitespace or characters".into());
        }
    } else {
        let mut expected = expected(raw, section)?;
        for row in &mut expected {
            super::assertions::omit_empty_lists(row);
            let optional = optional_fields(section, row);
            super::assertions::omit_null_fields(row, optional);
        }
        if rows != expected {
            return Err(format!(
                "NCI {section} changed original records or relationships"
            ));
        }
    }
    if section == "diseases" {
        snapshot_policy(env, &args)?;
    }
    Ok(())
}
fn optional_fields(section: &str, row: &Value) -> &'static [&'static str] {
    match section {
        "eligibility" => &["inclusion_indicator", "display_order"],
        "outcomes" => &["name", "description", "timeframe", "type_code"],
        "diseases" => &["inclusion_indicator", "is_lead_disease"],
        "arms" if row.get("record") == Some(&json!("arm")) => &["name", "description", "type"],
        "arms" => &["description", "type", "category", "inclusion_indicator"],
        _ => &[],
    }
}

fn snapshot_policy(env: &Environment, args: &[String]) -> Check {
    let mut disabled = args.to_vec();
    disabled.extend(["--disable-sources".into(), "nci".into()]);
    disabled.extend(env.globals(None, true));
    let output = invoke(env, &disabled, 4)?;
    if output.pointer("/sources/0/reason") != Some(&json!("disabled"))
        || output.pointer("/result/items/0/card").is_some()
    {
        return Err("disabled NCI restored saved protocol content".into());
    }
    let mut bare = args.to_vec();
    bare.extend(env.globals(None, true));
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &bare)
        .output()
        .map_err(|e| e.to_string())?;
    let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || value.pointer("/status") != Some(&json!("complete"))
        || value.pointer("/result/items/0/card").is_none()
    {
        return Err("saved NCI pool required an unused source key".into());
    }
    Ok(())
}

fn expected(raw: &Value, section: &str) -> Result<Vec<Value>, String> {
    let array = |v: &Value| {
        v.as_array()
            .cloned()
            .ok_or("source collection missing".into())
    };
    match section {
        "eligibility" => array(&raw["eligibility"]["unstructured"]),
        "outcomes" => array(&raw["outcome_measures"]),
        "diseases" => array(&raw["diseases"]),
        "arms" => {
            let mut expected = Vec::new();
            for (index, arm) in raw["arms"]
                .as_array()
                .ok_or("source arms missing")?
                .iter()
                .enumerate()
            {
                expected.push(json!({"record":"arm","arm_index":index,"data":{"name":arm["name"],"description":arm["description"],"type":arm["type"]}}));
                for (position, item) in arm["interventions"]
                    .as_array()
                    .ok_or("source interventions missing")?
                    .iter()
                    .enumerate()
                {
                    expected.push(json!({"record":"intervention","arm_index":index,"index":position,"data":item}));
                }
            }
            Ok(expected)
        }
        _ => Err("unknown source collection".into()),
    }
}

fn identity() -> Check {
    for absent in [false, true] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let dir = env.root().join("replies");
        copy_capture("trial/nci/get", &dir, |_, record| {
            if absent {
                record["exchanges"][0]["status"] = json!(404);
            } else {
                record["exchanges"][0]["body"]["value"]["nct_id"] = json!("NCT02296125");
            }
        })
        .map_err(|e| e.to_string())?;
        let mut args = ["get", "trial", "NCT05136196", "--source", "nci"]
            .map(str::to_owned)
            .to_vec();
        args.extend(env.globals(Some(&dir), false));
        let output = invoke(&env, &args, if absent { 1 } else { 4 })?;
        if output.pointer("/result/items/0/card").is_some()
            || output.pointer("/result/items/0/status")
                != Some(&json!(if absent { "absent" } else { "failed" }))
        {
            return Err("NCI identity mismatch or absence fabricated a card".into());
        }
        if !absent {
            if output.pointer("/sources/0/reason") != Some(&json!("bad-reply")) {
                return Err("NCI wrong-identity reply was not rejected".into());
            }
            let mut args = ["get", "trial", "NCT05136196", "--source", "nci"]
                .map(str::to_owned)
                .to_vec();
            args.extend(env.globals(None, true));
            let output = invoke(&env, &args, 4)?;
            if output.pointer("/sources/0/reason") != Some(&json!("offline")) {
                return Err("wrong-identity NCI record entered cache".into());
            }
        }
    }
    Ok(())
}

fn selector(section: &str) -> Vec<String> {
    [
        "get",
        "trial",
        "NCT05136196",
        section,
        "--source",
        "nci",
        "--limit",
        "10",
    ]
    .map(str::to_owned)
    .to_vec()
}
