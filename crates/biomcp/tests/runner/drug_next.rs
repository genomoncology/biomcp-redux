//! Feature drug-next-steps: wholly synthetic D05/D17/D19-shaped source contexts.
//! These fixtures test selector propagation, never clinical answers or licensed annotations.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
mod native;
const ROOT: &str = "/result/candidates/0/sections/oncokb";
const NAMES: [&str; 9] = [
    "synthetic-explicit-context",
    "synthetic-d17-group-context",
    "synthetic-d19-no-allele",
    "missing-build",
    "missing-gene",
    "missing-tumor",
    "unsupported-build",
    "compound-profile",
    "all-distinct-pivots-and-both-stream-pages",
];
pub(super) fn trials() -> Vec<Trial> {
    let mut trials = native::trials();
    trials.extend(
        NAMES
            .into_iter()
            .enumerate()
            .map(|(n, name)| {
                Trial::test(format!("drug-next-steps/{name}"), move || {
                    run(n).map_err(Into::into)
                })
            })
            .collect::<Vec<_>>(),
    );
    trials
}
fn run(n: usize) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let stub = Stub::for_sources(
        &["civicdb.org", "www.oncokb.org"],
        |_| "application/json",
        reply,
    )?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    let mut config = json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca});
    env.config(&config).map_err(|e| e.to_string())?;
    let mut args = [
        "get",
        "drug",
        &(900 + n).to_string(),
        "civic",
        "--source",
        "civic",
    ]
    .map(str::to_owned)
    .to_vec();
    if n == 8 {
        args.extend(["--section-limit".into(), "7".into()]);
    }
    let recording = env.root().join("civic-recording");
    let mut record = args.to_vec();
    record.extend(["--record".into(), recording.display().to_string()]);
    let value = invoke(&env, record, 0)?;
    super::schema::Registry::embedded()?.validate(&value, &args, true)?;
    verify(n, &value)?;
    let globals = vec![
        "--replay".into(),
        recording.display().to_string(),
        "--fresh".into(),
    ];
    let mcp = super::mcp::call_with_globals(&env, &args, false, &globals)?;
    for key in ["result", "next", "problems"] {
        if value.get(key) != mcp.get(key) {
            return Err(format!("CLI/MCP {key} differs"));
        }
    }
    if n == 0 {
        visible_scope(&env, &args, &globals)?;
        follow(&env, &value, &mut config)?;
    }
    let requests = stub.finish()?;
    let expected = if n == 0 { 2 } else { 1 };
    if requests.len() != expected {
        return Err(format!("unexpected network calls: {}", requests.len()));
    }
    let comparable = super::source_terms::size(&value, "")?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "DRUG_NEXT_SIZE case={n} full_bytes={} comparable_bytes={comparable}",
        value.to_string().len()
    )
    .map_err(|e| e.to_string())?;
    // Measured full and source-terms-subtracted envelopes, each with 15 percent headroom.
    let (full_cap, comparable_cap) = [
        (2496, 2399),
        (2039, 1943),
        (1992, 1896),
        (1983, 1886),
        (2022, 1926),
        (1943, 1846),
        (2014, 1918),
        (2021, 1924),
        (13813, 13717),
    ]
    .get(n)
    .copied()
    .ok_or("case ceiling")?;
    if value.to_string().len() > full_cap || comparable > comparable_cap {
        return Err(format!("case {n} exceeds measured envelope ceilings"));
    }
    Ok(())
}
fn verify(n: usize, value: &Value) -> Result<(), String> {
    let empty = Vec::new();
    let steps = value
        .get("next")
        .and_then(Value::as_array)
        .unwrap_or(&empty);
    if n == 8 {
        return many(value, steps);
    }
    let annotation = steps
        .iter()
        .filter_map(|s| s.get("args").and_then(Value::as_array))
        .find(|a| a.iter().any(|v| v == "oncokb"));
    if n == 0 {
        let lookup = value
            .pointer("/result/civic/data/evidence/records/0/oncokb_lookup")
            .ok_or("missing scoped lookup")?;
        if lookup.get("member") != Some(&json!("900"))
            || lookup.get("label")
                != Some(&json!(
                    "Look up this source-listed genomic variant in the stated tumor context"
                ))
            || lookup.pointer("/query/input") != Some(&json!("NC_000007.14:g.140753336A>T"))
        {
            return Err("lookup lost source-member linkage or scope".into());
        }
        let args = annotation.ok_or("explicit source context omitted typed OncoKB step")?;
        if args
            != &json!([
                "get",
                "variant",
                "NC_000007.14:g.140753336A>T",
                "oncokb",
                "--build",
                "GRCh38",
                "--tumor-type",
                "Synthetic tumor"
            ])
            .as_array()
            .ok_or("args")?
            .clone()
        {
            return Err(format!("wrong source context: {args:?}"));
        }
    } else {
        if annotation.is_some() {
            return Err("invented complete allele context".into());
        }
        let field = match n {
            1 | 2 | 7 => "variant",
            3 | 6 => "build",
            4 => "gene",
            5 => "tumor_type",
            _ => return Err("case".into()),
        };
        if !value
            .pointer("/result/civic/data/evidence/records/0/oncokb_missing")
            .and_then(Value::as_array)
            .is_some_and(|p| p.iter().any(|p| p == &json!(field)))
        {
            return Err(format!("missing named context {field}: {value}"));
        }
    }
    if value.to_string().contains("LEVEL_1") {
        return Err("drug response gained therapy level".into());
    }
    Ok(())
}
fn follow(env: &Environment, value: &Value, config: &mut Value) -> Result<(), String> {
    let args = value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("next")?
        .iter()
        .filter_map(|s| s.get("args").and_then(Value::as_array))
        .find(|a| a.iter().any(|v| v == "oncokb"))
        .ok_or("annotation")?
        .iter()
        .map(|v| v.as_str().map(str::to_owned).ok_or("arg".into()))
        .collect::<Result<Vec<_>, String>>()?;
    let missing = invoke(env, args.clone(), 0)?;
    if missing.pointer(&format!("{ROOT}/reason")) != Some(&json!("no_key")) {
        return Err("missing key did not stop".into());
    }
    let mcp_missing = super::mcp::call_with_globals(env, &args, false, &[])?;
    if missing.get("result") != mcp_missing.get("result") {
        return Err("missing-key CLI/MCP differs".into());
    }
    config.as_object_mut().ok_or("config object")?.insert(
        "keys".into(),
        json!({"oncokb_api_key":"synthetic-oncokb-variant-key"}),
    );
    env.config(config).map_err(|e| e.to_string())?;
    let recorded = env.root().join("annotation-recording");
    let mut keyed = args.clone();
    keyed.extend(["--record".into(), recorded.display().to_string()]);
    let output = super::oncokb::invoke(env, &keyed, 0)?;
    if output.pointer(&format!("{ROOT}/state")) != Some(&json!("data")) {
        return Err(format!("keyed route failed: {output}"));
    }
    let globals = vec![
        "--replay".into(),
        recorded.display().to_string(),
        "--fresh".into(),
    ];
    let mcp = super::mcp::call_with_globals(env, &args, false, &globals)?;
    if output.get("result") != mcp.get("result") {
        return Err("keyed CLI/MCP differs".into());
    }
    for file in fs::read_dir(recorded.join("oncokb")).map_err(|e| e.to_string())? {
        let bytes = fs::read(file.map_err(|e| e.to_string())?.path()).map_err(|e| e.to_string())?;
        if bytes
            .windows(b"synthetic-oncokb-variant-key".len())
            .any(|b| b == b"synthetic-oncokb-variant-key")
        {
            return Err("key in recording".into());
        }
    }
    Ok(())
}
fn reply(request: &str) -> Result<String, String> {
    if request.starts_with("GET /api/v1/annotate/") {
        return super::oncokb::reply(request);
    }
    let (_, body) = request.split_once("\r\n\r\n").ok_or("POST")?;
    let query: Value = serde_json::from_str(body).map_err(|e| e.to_string())?;
    let id = query
        .pointer("/variables/id")
        .and_then(Value::as_u64)
        .ok_or("therapy ID")?;
    let n = id - 900;
    let hgvs = match n {
        1 => json!(["NC_000007.14:g.140753336A>T", "NC_000007.14:g.140753336A>C"]),
        2 => json!(["N/A"]),
        _ => json!(["NC_000007.14:g.140753336A>T"]),
    };
    let coordinates = match n {
        3 => Value::Null,
        6 => json!({"referenceBuild":"NCBI36"}),
        _ => json!({"referenceBuild":"GRCH38"}),
    };
    let gene = if n == 4 {
        "synthetic invalid gene"
    } else {
        "BRAF"
    };
    let member = json!({"id":900,"name":"Synthetic allele","link":"/variants/900",
        "feature":{"name":gene},"hgvsDescriptions":hgvs,"coordinates":coordinates});
    let profile = json!({"id":900,"name":"Synthetic profile","link":"/molecular-profiles/900","isComplex":n==7,"variants":[member]});
    let disease = if n == 5 {
        Value::Null
    } else {
        json!({"name":"Synthetic tumor","doid":"0000"})
    };
    let mut value = json!({"data":{"therapy":{"id":id,
"name":"Synthetic therapy",
"link":format!("/therapies/{id}")},
"evidenceItems":{"totalCount":1,
"pageInfo":{"hasNextPage":false,
"endCursor":null},
"nodes":[{"id":900,
"status":"ACCEPTED",
"evidenceType":"PREDICTIVE",
"evidenceLevel":"A",
"evidenceDirection":"SUPPORTS",
"significance":"SENSITIVITYRESPONSE",
"disease":disease,
"therapies":[{"id":id,
"name":"Synthetic therapy"}],
"source":{"citationId":"17611497",
"sourceType":"PUBMED"},
"molecularProfile":profile}]},
"assertions":{"totalCount":0,
"pageInfo":{"hasNextPage":false,
"endCursor":null},
"nodes":[]}}});
    if n == 8 {
        expand(&mut value)?;
    }
    Ok(value.to_string())
}

fn many(value: &Value, steps: &[Value]) -> Result<(), String> {
    let gets = steps
        .iter()
        .filter_map(|s| s.get("args").and_then(Value::as_array))
        .filter(|a| a.get(1) == Some(&json!("variant")))
        .collect::<Vec<_>>();
    if gets.len() != 6 {
        return Err(format!("expected all six distinct pivots: {value}"));
    }
    for (n, args) in gets.iter().enumerate() {
        if args.get(2) != Some(&json!(format!("NC_000007.14:g.{}A>T", 140753336 + n))) {
            return Err("variant pivot order changed".into());
        }
    }
    let pages = steps
        .iter()
        .filter(|s| s.get("kind") == Some(&json!("page")))
        .collect::<Vec<_>>();
    if pages.len() != 2 {
        return Err("independent claim continuations were hidden".into());
    }
    for kind in ["evidence", "assertions"] {
        if !pages.iter().any(|s| {
            s.get("args").and_then(Value::as_array).is_some_and(|a| {
                a.windows(2)
                    .any(|v| v == [json!("--civic-kind"), json!(kind)])
            })
        }) {
            return Err(format!("missing {kind} continuation"));
        }
    }
    Ok(())
}
fn expand(value: &mut Value) -> Result<(), String> {
    let base = value
        .pointer("/data/evidenceItems/nodes/0")
        .ok_or("source claim")?
        .clone();
    let rows = (0..6)
        .map(|n| {
            let mut row = base.clone();
            *row.pointer_mut("/id").ok_or("claim ID")? = json!(900 + n);
            *row.pointer_mut("/molecularProfile/id")
                .ok_or("profile ID")? = json!(900 + n);
            *row.pointer_mut("/molecularProfile/link")
                .ok_or("profile link")? = json!(format!("/molecular-profiles/{}", 900 + n));
            *row.pointer_mut("/molecularProfile/variants/0/id")
                .ok_or("member ID")? = json!(900 + n);
            *row.pointer_mut("/molecularProfile/variants/0/link")
                .ok_or("member link")? = json!(format!("/variants/{}", 900 + n));
            *row.pointer_mut("/molecularProfile/variants/0/hgvsDescriptions")
                .ok_or("HGVS")? = json!([format!("NC_000007.14:g.{}A>T", 140753336 + n)]);
            Ok(row)
        })
        .collect::<Result<Vec<Value>, String>>()?;
    let mut assertion = base;
    let fields = assertion.as_object_mut().ok_or("assertion object")?;
    fields.insert("assertionType".into(), json!("PREDICTIVE"));
    fields.insert("assertionDirection".into(), json!("SUPPORTS"));
    fields.insert("variantOrigin".into(), json!("SOMATIC"));
    *value.pointer_mut("/data/evidenceItems").ok_or("evidence")? = json!({"totalCount":7,
"pageInfo":{"hasNextPage":true,
"endCursor":"next-evidence"},
"nodes":rows});
    *value.pointer_mut("/data/assertions").ok_or("assertions")? = json!({"totalCount":2,
"pageInfo":{"hasNextPage":true,
"endCursor":"next-assertions"},
"nodes":[assertion]});
    Ok(())
}

fn visible_scope(env: &Environment, command: &[String], globals: &[String]) -> Result<(), String> {
    let mut args = globals.to_vec();
    args.extend(["--format".into(), "md".into()]);
    args.extend_from_slice(command);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success()
        || !output.stderr.is_empty()
        || !text.contains("Look up this source-listed genomic variant in the stated tumor context")
        || !text.contains("NC_000007.14:g.140753336A&gt;T")
            && !text.contains("NC_000007.14:g.140753336A>T")
    {
        return Err("Markdown omitted the scoped source-listed lookup".into());
    }
    Ok(())
}
