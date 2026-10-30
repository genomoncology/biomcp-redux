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
    for mode in 0..13 {
        let mut expect = match mode {
            6 => json!({
                "/result/items/0/sections/civic/state":"data",
                "/result/items/0/sections/civic/data/id/id":20,
                "/result/items/0/sections/civic/data/id/namespace":"CIViC",
                "/result/items/0/sections/civic/data":{"omits":["description"]},
                "":{"omits":["problems"]}
            }),
            8 | 11 => json!({
                "/result/items/0/sections/civic/state":"data",
                "/result/items/0/sections/civic/data/id/id":20,
                "/result/items/0/sections/civic/data/id/namespace":"CIViC",
                "/result/items/0/sections/civic/data/variant_count":126,
                "/result/items/0/sections/civic/data":{"omits":[if mode == 8 {"url"} else {"description"}]},
                "/problems":{"count":1},
                "/problems/0/field":if mode == 8 {"link"} else {"description"},
                "/problems/0/record":"20"
            }),
            7 | 9 | 12 => json!({
                "/result/items/0/sections/civic/state":"empty",
                "/result/items/0/sections/civic":{"omits":["record","data"]},
                "/problems":{"count":1},
                "/problems/0/field":if mode == 9 {"id"} else {"entrezId"}
            }),
            _ => json!({
                "/result/items/0/sections/civic/state":"unavailable",
                "/sources/2/reason":"bad-reply"
            }),
        };
        let fields = expect.as_object_mut().ok_or("expected assertion object")?;
        fields.insert("/result/items/0/answer/gene/symbol".into(), json!("ERBB2"));
        fields.insert(
            "/result/items/0/answer/gene/hgnc_id".into(),
            json!("HGNC:3430"),
        );
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
                    11 => value["data"]["g0"]["description"] = json!(42),
                    12 => value["data"]["g0"]["entrezId"] = json!(1956),
                    _ => {
                        *body =
                            json!({"kind":"text", "value":"{\"data\":{\"g0\":null,\"g0\":null}}"})
                    }
                }
            },
            expect,
        )
        .map_err(|error| format!("CIViC boundary mode {mode}: {error}"))?;
    }
    Ok(())
}

fn constraint() -> Check {
    for mode in (0..10).chain(11..16) {
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
            match mode {
                3 => {
                    json!({"/result/items/0/sections/constraint/state": "unavailable", "/problems/*/field": ["gene_id"], "/sources/2/state": "ok"})
                }
                9 => json!({"/result/items/0/sections/constraint/state": "empty"}),
                11..=14 => {
                    json!({"/result/items/0/sections/constraint/state": "empty", "/problems/*/field": ["mane_select_transcript.ensembl_id"]})
                }
                _ => {
                    json!({"/result/items/0/sections/constraint/state": "unavailable", "/sources/2/reason": "bad-reply"})
                }
            },
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
        "malformed classification",
        "failed dosage",
        "failed validity",
        "both failed",
        "validity only",
        "dosage only",
    ] {
        let expect = clingen_expectation(mode);
        let args = if matches!(mode, "duplicate" | "missing classification") {
            vec!["get", "gene", "TP53", "MLH1", "clingen"]
        } else {
            vec!["get", "gene", "TP53", "clingen"]
        };
        let answer = checked_case(
            "gene/clingen",
            &args,
            |source, record| {
                if source == "hgnc" {
                    // Keep this section witness on the recorded TP53 annotation request.
                    let body = &mut record["exchanges"][0]["body"]["value"];
                    *body = body.as_str().unwrap().replace("\t4292\t", "\t\t").into();
                }
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
                    ("missing classification" | "malformed classification", false) => {
                        let classification = if mode == "missing classification" {
                            ""
                        } else {
                            "bad\u{1}"
                        };
                        lines[index] = lines[index].replace("Definitive", classification);
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
                    // Keep this independent-status fixture within the shared source budget.
                    record["exchanges"][0]["delay_ms"] = 0.into();
                    // Three failed attempts exercise exhaustion, not a missing recording.
                    record["exchanges"] = json!(vec![record["exchanges"][0].clone(); 3]);
                }
            },
            expect,
            0,
        )
        .map_err(|error| format!("ClinGen {mode}: {error}"))?;
        clingen_checked(mode, &answer)?;
    }
    // sections/local-replay-miss.yaml owns an incomplete bundle recording.
    Ok(())
}

fn clingen_checked(mode: &str, answer: &Value) -> Check {
    if matches!(mode, "failed validity" | "both failed" | "dosage only")
        && answer
            .pointer("/result/items/0/sections/clingen/data/validity")
            .is_some()
    {
        return Err("absent or failed validity produced a validity list".into());
    }
    if matches!(mode, "duplicate" | "missing classification") {
        for field in ["validity_status", "dosage_status"] {
            let pointer = format!("/result/items/1/sections/clingen/data/{field}/state");
            if answer.pointer(&pointer) != Some(&json!("ok")) {
                return Err(format!("malformed TP53 record affected MLH1 {field}").into());
            }
        }
    }
    if matches!(mode, "validity only" | "failed dosage" | "both failed")
        && answer
            .pointer("/result/items/0/sections/clingen/data/haploinsufficiency")
            .is_some()
    {
        return Err("absent or failed dosage produced haploinsufficiency".into());
    }
    Ok(())
}

fn clingen_expectation(mode: &str) -> Value {
    match mode {
        "failed dosage" => {
            json!({
                "/result/items/0/sections/clingen/state":"data",
                "/result/items/0/sections/clingen/data/validity_status/state":"ok",
                "/result/items/0/sections/clingen/data/dosage_status/state":"failed",
                "/result/items/0/sections/clingen/data/validity": {"count": 1},
                "/sources/2/reason":"unavailable"
            })
        }
        "failed validity" => {
            json!({"/result/items/0/sections/clingen/data/validity_status/state":"failed",
                "/result/items/0/sections/clingen/data/dosage_status/state":"ok",
                "/result/items/0/sections/clingen/data/haploinsufficiency":"Sufficient Evidence for Haploinsufficiency"})
        }
        "both failed" => {
            json!({"/result/items/0/sections/clingen/data/validity_status/state":"failed",
                "/result/items/0/sections/clingen/data/dosage_status/state":"failed"})
        }
        "validity only" => {
            json!({"/result/items/0/sections/clingen/data/validity": {"count": 1}})
        }
        "dosage only" => json!({
            "/result/items/0/sections/clingen/data/haploinsufficiency": "Sufficient Evidence for Haploinsufficiency"}),
        "duplicate" => {
            json!({"/result/items/0/sections/clingen/state": "data", "/problems/*/field": ["HGNC ID"]})
        }
        "missing classification" => json!({
            "/result/items/0/sections/clingen/state": "data",
            "/result/items/0/sections/clingen/data/validity/0/mondo_id": "MONDO:0018875",
            "/result/items/0/sections/clingen/data/validity/0": {"omits": ["classification"]},
            "": {"omits": ["problems"]}
        }),
        _ => json!({
            "/result/items/0/sections/clingen/state": "data",
            "/result/items/0/sections/clingen/data/validity/0/mondo_id": "MONDO:0018875",
            "/result/items/0/sections/clingen/data/validity/0": {"omits": ["classification"]},
            "/problems/*/field": ["CLASSIFICATION"]
        }),
    }
}
