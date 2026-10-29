//! Retained historical variant component pages.
use super::{Stub, Value, component_body, json, source_environment};

pub(super) fn timestamp_admission() -> Result<(), String> {
    let prefix = "/result/candidates/0/sections/conservation/data/components/0";
    for mode in ["local", "offset", "invalid", "whitespace", "empty"] {
        let value = super::super::response_cases::checked_case(
            "variant/synthetic-components",
            &[
                "get",
                "variant",
                "rs113488022",
                "conservation",
                "--build",
                "GRCh37",
            ],
            |_, record| timestamp_body(mode, record),
            json!({"/result/candidates/0/sections/conservation/state":"data"}),
            0,
        )
        .map_err(|error| error.to_string())?;
        if mode == "local" || mode == "offset" {
            let (text, timezone) = if mode == "local" {
                ("2020-05-08T17:33:59.756164", "source-gave-none")
            } else {
                ("2020-05-08t17:33:59.756164-04:30", "in-value")
            };
            if value.pointer(&format!("{prefix}/aggregator_built")) != Some(&json!(text))
                || value.pointer(&format!("{prefix}/uploads/0/date"))
                    != Some(&json!({"value":text,"timezone":timezone}))
            {
                return Err(format!(
                    "local timestamp precision or timezone changed: {value}"
                ));
            }
        } else {
            let fields: Vec<_> = value
                .get("problems")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(|problem| problem.get("field").and_then(Value::as_str))
                .collect();
            let field = if mode == "empty" {
                "src.dbnsfp"
            } else {
                "build_date"
            };
            if !fields.contains(&field)
                || value
                    .pointer(&format!("{prefix}/aggregator_built"))
                    .is_some()
            {
                return Err(format!(
                    "invalid or empty metadata discarded usable component data: {value}"
                ));
            }
        }
    }
    Ok(())
}

fn timestamp_body(mode: &str, record: &mut Value) {
    if record
        .get("url")
        .and_then(Value::as_str)
        .is_some_and(|url| url.ends_with("/metadata"))
    {
        let Some(body) = record.pointer_mut("/exchanges/0/body/value") else {
            return;
        };
        if mode == "empty" {
            if let Some(source) = body.get_mut("src") {
                *source = json!({});
            }
        } else {
            let timestamp = if mode == "local" {
                "2020-05-08T17:33:59.756164"
            } else if mode == "offset" {
                "2020-05-08t17:33:59.756164-04:30"
            } else if mode == "whitespace" {
                " 2020-05-08T17:33:59.756164 "
            } else {
                "2024-02-30T10:00:00"
            };
            if let Some(date) = body.get_mut("build_date") {
                *date = json!(timestamp);
            }
            if let Some(date) = body.pointer_mut("/src/dbnsfp/upload_date") {
                *date = json!({"synthetic_part":timestamp});
            }
        }
    }
}

pub(super) fn component_walk() -> Result<(), String> {
    let stub = Stub::for_source("myvariant.info", "application/json", component_page_body)?;
    let environment = source_environment(&stub)?;
    let mut ids = Vec::new();
    for offset in [0, 2] {
        let mut args = vec![
            "get".into(),
            "variant".into(),
            "chr7:g.140453136A>T".into(),
            "cosmic".into(),
            "--build".into(),
            "GRCh37".into(),
            "--section-limit".into(),
            "2".into(),
            "--section-offset".into(),
            offset.to_string(),
        ];
        if offset != 0 {
            args.push("--offline".into());
        }
        let output = environment
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .output()
            .map_err(|e| e.to_string())?;
        let value: Value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
        let data = value
            .pointer("/result/candidates/0/sections/cosmic/data")
            .ok_or_else(|| format!("source page missing: {value}"))?;
        if data.get("total") != Some(&json!(4)) {
            return Err(format!("source total lost: {value}"));
        }
        if offset == 0 && data.pointer("/next/section_offset") != Some(&json!(2)) {
            return Err(format!(
                "typed section continuation lost its independent offset: {value}"
            ));
        }
        if offset != 0 && data.get("next").is_some() {
            return Err("extra historical page invented".into());
        }
        for row in data
            .get("records")
            .and_then(Value::as_array)
            .ok_or("rows missing")?
        {
            ids.push(row.get("cosmic_id").cloned().ok_or("native ID missing")?);
        }
    }
    if ids
        != [
            json!("COSM999991"),
            json!("COSM999992"),
            json!("COSM999993"),
            json!("COSM999994"),
        ]
        || stub.finish()?.len() != 3
    {
        return Err("source walk repeated, dropped, reordered or fetched a warm page".into());
    }
    Ok(())
}
fn component_page_body(request: &str) -> Result<String, String> {
    if !request.contains("fields=_id%2Ccosmic") {
        return component_body(request);
    }
    // Explicit synthetic rows keep source order and use no captured source content.
    Ok(json!(
        {"_id":"chr7:g.140453136A>T","cosmic":[{"cosmic_id":"COSM999991"},{"cosmic_id":"COSM999992"},{"cosmic_id":"COSM999993"},
        {"cosmic_id":"COSM999994"}]}
    )
    .to_string())
}
