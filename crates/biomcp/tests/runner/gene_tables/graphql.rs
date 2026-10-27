use super::super::response_cases::checked_case;
use super::Check;
use serde_json::{Value, json};

fn graphql(source: &str, change: impl Fn(&mut Value), expect: Value) -> Check {
    checked_graphql(source, change, expect).map(|_| ())
}

fn checked_graphql(
    source: &str,
    change: impl Fn(&mut Value),
    expect: Value,
) -> Result<Value, Box<dyn std::error::Error>> {
    let (capture, args) = match source {
        "civic" => ("gene/civic", vec!["get", "gene", "ERBB2", "civic"]),
        _ => ("gene/constraint", vec!["get", "gene", "TP53", "constraint"]),
    };
    checked_case(
        capture,
        &args,
        |name, record| {
            if name == source {
                change(&mut record["exchanges"][0]["body"]);
            }
        },
        expect,
        0,
    )
}

pub(super) fn check() -> Check {
    civic()?;
    constraint()?;
    clingen()
}

fn civic() -> Check {
    for mode in 0..11 {
        graphql(
            "civic",
            |body| {
                let value = &mut body["value"];
                match mode {
                    0 => *value = json!({"data":{}}),
                    1 => *value = json!({"data":{"other":null}}),
                    2 => *value = json!({"data":{"g0":7}}),
                    3 => *value = json!({"data":{"g0":null},"errors":null}),
                    4 => *value = json!({"data":{"g0":null},"errors":[]}),
                    5 => *value = json!({"data":{"g0":null},"errors":[{"message":"missing"}]}),
                    6 => {
                        value["data"]["g0"]
                            .as_object_mut()
                            .unwrap()
                            .remove("description");
                    }
                    7 => value["data"]["g0"]["entrezId"] = json!(0),
                    8 => value["data"]["g0"]["link"] = json!("https://other.example/features/20"),
                    9 => value["data"]["g0"]["id"] = json!(0),
                    _ => {
                        *body =
                            json!({"kind":"text", "value":"{\"data\":{\"g0\":null,\"g0\":null}}"})
                    }
                }
            },
            json!({"/result/items/0/sections/civic/state": "unavailable", "/sources/2/reason": "bad_reply"}),
        )?;
    }
    Ok(())
}

fn constraint() -> Check {
    for mode in 0..16 {
        graphql(
            "gnomad",
            |body| {
                let value = &mut body["value"];
                match mode {
                    0 => *value = json!({"data":{}}),
                    1 => *value = json!({"data":17}),
                    2 => *value = json!({"data":{"extra":null}}),
                    3 => *value = json!({"data":{"g0":7}}),
                    4 => *value = json!({"data":{"g0":null}}),
                    5 => *value = json!({"data":{"g0":null},"errors":[]}),
                    6 => {
                        *value = json!({"data":{"g0":null},"errors":[{"message":"Gene not found"},{"message":"bad query"}]})
                    }
                    7 => *value = json!({"data":{"g0":null},"errors":[{}]}),
                    8 => {
                        *value = json!({"data":{"g0":null},"errors":[{"message":"gene not found"}]})
                    }
                    9 => {
                        value["data"]["g0"]
                            .as_object_mut()
                            .unwrap()
                            .remove("gnomad_constraint");
                    }
                    10 => value["data"]["g0"]["gnomad_constraint"]["pli"] = json!("NaN"),
                    11 => {
                        value["data"]["g0"]["mane_select_transcript"]["ensembl_id"] = json!("bad")
                    }
                    12 => {
                        value["data"]["g0"]["mane_select_transcript"]["ensembl_id"] =
                            json!("ENST00000269305.01")
                    }
                    13 => {
                        value["data"]["g0"]["mane_select_transcript"]["ensembl_id"] =
                            json!("ENST00000269305.bad")
                    }
                    14 => {
                        value["data"]["g0"]["mane_select_transcript"]["ensembl_id"] =
                            json!("NM_004333.4")
                    }
                    _ => {
                        *body = json!({"kind":"text", "value":"{\"data\":{\"g0\":null,\"g0\":null},\"errors\":[{\"message\":\"Gene not \
                            found\"},{\"message\":\"Gene not found\"}]}"})
                    }
                }
            },
            json!({"/result/items/0/sections/constraint/state": "unavailable", "/sources/2/reason": "bad_reply"}),
        )?;
    }
    mappings()
}

fn mappings() -> Check {
    graphql(
        "gnomad",
        |body| {
            body["value"]["data"]["g0"]["mane_select_transcript"]["ensembl_id"] =
                json!("ENST00000311936.8");
            body["value"]["data"]["g0"]["gnomad_constraint"]["flags"] =
                json!(["source flag", 17, ""]);
        },
        json!({"/result/items/0/sections/constraint/data/transcript": "ENST00000311936",
        "/result/items/0/sections/constraint/data/flags": ["source flag"], "/problems/*/field": ["flags"]}),
    )?;
    for flags in [Value::Null, json!({"unexpected":"value"})] {
        let mut expect =
            json!({ "/result/items/0/sections/constraint/data/transcript": "ENST00000269305"});
        if !flags.is_null() {
            expect["/problems/*/field"] = json!(["flags"]);
        }
        let answer = checked_graphql(
            "gnomad",
            |body| {
                body["value"]["data"]["g0"]["mane_select_transcript"] = Value::Null;
                body["value"]["data"]["g0"]["gnomad_constraint"]["flags"] = flags.clone();
            },
            expect,
        )?;
        empty_flags(&answer, flags.is_null())?;
    }
    for mode in 0..2 {
        graphql(
            "gnomad",
            |body| {
                if mode == 0 {
                    body["value"]["data"]["g0"]["gnomad_constraint"] = Value::Null;
                } else {
                    body["value"] =
                        json!({"data":{"g0":null},"errors":[{"message":"Gene not found"}]});
                }
            },
            json!({"/result/items/0/sections/constraint/state": "empty"}),
        )?;
    }
    // Existing mapping-wrong-identity.yaml owns the mismatched identity claim.
    Ok(())
}

fn empty_flags(answer: &Value, no_problem: bool) -> Check {
    for pointer in ["/result/items/0/sections/constraint/data/flags"]
        .into_iter()
        .chain(no_problem.then_some("/problems"))
    {
        if answer
            .pointer(pointer)
            .is_some_and(|v| !v.as_array().is_some_and(Vec::is_empty))
        {
            return Err(format!("unexpected values at {pointer}").into());
        }
    }
    Ok(())
}

fn clingen() -> Check {
    for mode in [
        "duplicate",
        "missing classification",
        "failed dosage",
        "failed validity",
        "both failed",
        "validity only",
        "dosage only",
    ] {
        let expect = match mode {
            "failed dosage" => {
                json!({"/result/items/0/sections/clingen/state":"data", "/result/items/0/sections/clingen/data/validity_status/state":"ok", "/result/items/0/sections/clingen/data/dosage_status/state":"failed", "/result/items/0/sections/clingen/data/validity": {"count": 1}, "/sources/2/reason":"unavailable"})
            }
            "failed validity" => {
                json!({"/result/items/0/sections/clingen/data/validity_status/state":"failed", "/result/items/0/sections/clingen/data/dosage_status/state":"ok", "/result/items/0/sections/clingen/data/validity":[], "/result/items/0/sections/clingen/data/haploinsufficiency":"Sufficient Evidence for Haploinsufficiency"})
            }
            "both failed" => {
                json!({"/result/items/0/sections/clingen/data/validity_status/state":"failed", "/result/items/0/sections/clingen/data/dosage_status/state":"failed", "/result/items/0/sections/clingen/data/validity":[]})
            }
            "validity only" => {
                json!({"/result/items/0/sections/clingen/data/validity": {"count": 1}})
            }
            "dosage only" => json!({"/result/items/0/sections/clingen/data/validity": [],
                "/result/items/0/sections/clingen/data/haploinsufficiency": "Sufficient Evidence for Haploinsufficiency"}),
            _ => {
                json!({"/result/items/0/sections/clingen/state": "data", "/sources/2/reason": "bad_reply"})
            }
        };
        let answer = checked_case(
            "gene/clingen",
            &["get", "gene", "TP53", "clingen"],
            |source, record| {
                if source != "clingen" {
                    return;
                }
                let dosage = record["url"].as_str().unwrap().contains("dosage");
                let text = record["exchanges"][0]["body"]["value"].as_str().unwrap();
                let mut lines: Vec<_> = text.lines().map(str::to_owned).collect();
                let index = lines
                    .iter()
                    .position(|v| v.starts_with("\"TP53\""))
                    .unwrap();
                match (mode, dosage) {
                    ("duplicate", true) => lines.push(lines[index].clone()),
                    ("missing classification", false) => {
                        lines[index] = lines[index].replace("\"Definitive\"", "\"\"")
                    }
                    ("validity only", true) | ("dosage only", false) => {
                        lines.remove(index);
                    }
                    _ => (),
                }
                record["exchanges"][0]["body"]["value"] = format!("{}\n", lines.join("\n")).into();
                if mode == "both failed"
                    || (mode == "failed dosage" && dosage)
                    || (mode == "failed validity" && !dosage)
                {
                    record["exchanges"][0]["status"] = 500.into();
                }
            },
            expect,
            0,
        )?;
        if mode == "validity only"
            && answer
                .pointer("/result/items/0/sections/clingen/data/haploinsufficiency")
                .is_some()
        {
            return Err("validity-only source produced haploinsufficiency".into());
        }
    }
    // sections/local-replay-miss.yaml owns an incomplete bundle recording.
    Ok(())
}
