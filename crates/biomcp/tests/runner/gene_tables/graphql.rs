use super::{Check, case};
use serde_json::{Value, json};

fn graphql(source: &str, change: impl Fn(&mut Value), expect: Value) -> Check {
    let (capture, args) = match source {
        "civic" => ("gene/civic", vec!["get", "gene", "ERBB2", "civic"]),
        _ => ("gene/constraint", vec!["get", "gene", "TP53", "constraint"]),
    };
    case(
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
            json!({"/result/items/0/sections/civic/state": "unavailable",
            "/sources/2/reason": "bad_reply"}),
        )?;
    }
    Ok(())
}

fn constraint() -> Check {
    for mode in 0..13 {
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
                    _ => {
                        *body = json!({"kind":"text", "value":"{\"data\":{\"g0\":null,\"g0\":null},\"errors\":[{\"message\":\"Gene not found\"},{\"message\":\"Gene not found\"}]}"})
                    }
                }
            },
            json!({"/result/items/0/sections/constraint/state": "unavailable",
            "/sources/2/reason": "bad_reply"}),
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
        graphql(
            "gnomad",
            |body| {
                body["value"]["data"]["g0"]["mane_select_transcript"] = Value::Null;
                body["value"]["data"]["g0"]["gnomad_constraint"]["flags"] = flags.clone();
            },
            json!({"/result/items/0/sections/constraint/data/transcript": "ENST00000269305",
            "/result/items/0/sections/constraint/data/flags": []}),
        )?;
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

fn clingen() -> Check {
    for mode in [
        "duplicate",
        "missing classification",
        "failed dosage",
        "validity only",
        "dosage only",
    ] {
        let expect = match mode {
            "validity only" => {
                json!({"/result/items/0/sections/clingen/data/validity": {"count": 1}})
            }
            "dosage only" => json!({"/result/items/0/sections/clingen/data/validity": [],
                "/result/items/0/sections/clingen/data/haploinsufficiency": "Sufficient Evidence for Haploinsufficiency"}),
            _ => {
                json!({"/result/items/0/sections/clingen/state": "unavailable", "/sources/2/reason": "bad_reply"})
            }
        };
        case(
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
                if mode == "failed dosage" && dosage {
                    record["exchanges"][0]["status"] = 500.into();
                }
            },
            expect,
            0,
        )?;
    }
    // sections/local-replay-miss.yaml owns an incomplete bundle recording.
    Ok(())
}
