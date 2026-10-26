//! Feature phenotype-associations: exact direct native rows and safe identity pivots.
use super::{Stub, invoke, setup};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "phenotype-associations/direct-pages-preserve-native-rows",
            || exact().map_err(Into::into),
        ),
        Trial::test(
            "phenotype-associations/native-two-row-paging-and-row-local-fields",
            || pages().map_err(Into::into),
        ),
        Trial::test(
            "phenotype-associations/bad-endpoints-taxa-and-outages-never-cache",
            || failures().map_err(Into::into),
        ),
        Trial::test(
            "phenotype-associations/disabled-components-skip-cached-and-recorded-pages",
            || disabled().map_err(Into::into),
        ),
    ]
}
pub(super) fn row() -> Value {
    json!({"id":"synthetic-first","subject":"HGNC:9910","subject_label":"RBMX","subject_taxon":"NCBITaxon:9606","object":"HP:0001250","object_label":"Seizure","predicate":"biolink:has_phenotype","original_subject":"NCBIGene:27316","has_evidence":null,"publications":["PMID:1"],"negated":false,"frequency_qualifier":"HP:0040282","subject_closure":["HGNC:9910"],"object_closure":["HP:0001250"]})
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            if !request.starts_with("GET /v3/api/association?object=HP%3A0001250&subject_category=biolink%3AGene&subject_taxon=NCBITaxon%3A9606&direct=true&limit=1&offset=0 ") { return Err(format!("wrong direct request: {request}")); }
            Ok(json!({"limit":1,"offset":0,"total":3,"items":[row()]}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let result = invoke(
        &env,
        &[
            "get",
            "phenotype",
            "HP:0001250",
            "genes",
            "--source",
            "monarch",
        ],
        0,
    )?;
    for (pointer, expected) in [
        ("/page/total", json!(3)),
        ("/page/returned", json!(1)),
        ("/result/direct", json!(true)),
        (
            "/result/items/0/association/requested_id",
            json!("HP:0001250"),
        ),
        (
            "/result/items/0/association/original_subject",
            json!("NCBIGene:27316"),
        ),
        ("/result/items/0/association/negated", json!(false)),
        ("/next/0/args", json!(["get", "gene", "HGNC:9910"])),
    ] {
        if result.pointer(pointer) != Some(&expected) {
            return Err(format!("{pointer} changed: {result}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("direct page fetched another resource".into());
    }
    Ok(())
}

fn pages() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            let offset = if request.contains("offset=2 ") { 2 } else { 0 };
            if !request.contains("subject_category=biolink%3ADisease&direct=true&limit=2&offset=") {
                return Err(format!("wrong disease scope: {request}"));
            }
            let mut first = row();
            first["subject"] = json!("MONDO:0014379");
            first["original_subject"] = json!("Orphanet:1");
            first["id"] = json!(format!("synthetic-{offset}"));
            let mut second = first.clone();
            second["id"] = json!("synthetic-provenance-2");
            second["negated"] = json!(null);
            second["publications"] = json!(null);
            second["frequency_qualifier"] = json!(null);
            let items = if offset == 0 {
                vec![first, second]
            } else {
                vec![first]
            };
            Ok(json!({"limit":2,"offset":offset,"total":3,"items":items}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let words = [
        "get",
        "phenotype",
        "HP:0001250",
        "diseases",
        "--source",
        "monarch",
        "--section-limit",
        "2",
    ];
    let first = invoke(&env, &words, 0)?;
    if first.pointer("/page/returned") != Some(&json!(2))
        || first
            .pointer("/result/items/1/association/negated")
            .is_some()
        || first.pointer("/result/items/1/association/publications") != Some(&json!([]))
        || first.pointer("/result/items/1/association/qualifiers") != Some(&json!([]))
    {
        return Err(format!("native independent rows collapsed: {first}"));
    }
    let next = first
        .pointer("/next")
        .and_then(Value::as_array)
        .ok_or("next")?;
    if next.len() != 2
        || next[0]["args"] != json!(["get", "disease", "MONDO:0014379"])
        || next[1]["kind"] != json!("page")
    {
        return Err(format!("association navigation changed: {first}"));
    }
    let args = next[1]["args"]
        .as_array()
        .ok_or("page args")?
        .iter()
        .map(|v| v.as_str().ok_or("word"))
        .collect::<Result<Vec<_>, _>>()?;
    let last = invoke(&env, &args, 0)?;
    if last.pointer("/query/section_limit") != Some(&json!(2))
        || last.pointer("/page/offset") != Some(&json!(2))
        || last
            .get("next")
            .and_then(Value::as_array)
            .is_some_and(|v| v.iter().any(|s| s["kind"] == "page"))
    {
        return Err(format!("native page selector changed: {last}"));
    }
    if stub.finish()?.len() != 2 {
        return Err("paging fetched extra resources".into());
    }
    Ok(())
}
fn failures() -> Result<(), String> {
    for kind in 0..7 {
        let stub = Stub::for_source_status(
            "api-v3.monarchinitiative.org",
            "application/json",
            move |_| {
                if kind == 4 {
                    return Ok("{broken".into());
                }
                let mut row = row();
                match kind {
                    0 => row["object"] = json!("HP:0000001"),
                    1 => row["subject_taxon"] = json!("NCBITaxon:10090"),
                    2 => row["subject_taxon"] = json!(null),
                    _ => (),
                }
                Ok(
                    json!({"limit":1,"offset":if kind==3 {1} else {0},"total":3,"items":[row]})
                        .to_string(),
                )
            },
            move |_| match kind {
                5 => 429,
                6 => 500,
                _ => 200,
            },
        )?;
        let env = setup(&stub)?;
        let words = ["get", "phenotype", "HP:0001250", "genes"];
        let dir = env.root().join("failures");
        let result = super::record(&env, &words, &dir, 4)?;
        super::native::mcp(&dir, &words, &result, 4, None)?;
        if result.pointer("/sources/0/terms") != Some(&json!("Monarch source-specific licensing"))
            || result.get("result").is_some()
        {
            return Err(format!("failure lost source or leaked rows: {result}"));
        }
        if kind < 5 {
            if result
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(|v| v.is_empty())
            {
                return Err("bad reply lost its problem".into());
            }
            invoke(&env, &words, 4)?;
            if stub.finish()?.len() != 2 {
                return Err("bad reply entered cache".into());
            }
        } else {
            stub.finish()?;
        }
    }
    Ok(())
}
fn disabled() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", |_| {
        Ok(json!({"limit":1,"offset":0,"total":1,"items":[row()]}).to_string())
    })?;
    let env = setup(&stub)?;
    let words = ["get", "phenotype", "HP:0001250", "genes"];
    let directory = env.root().join("disabled");
    super::record(&env, &words, &directory, 0)?;
    for name in ["monarch", "hpo", "orphanet"] {
        let mut args = words.to_vec();
        args.extend(["--disable-sources", name]);
        let denied = invoke(&env, &args, 4)?;
        if denied.pointer("/sources/0/reason") != Some(&json!("disabled"))
            || denied.get("result").is_some()
        {
            return Err("disabled cached association escaped".into());
        }
        args.extend(["--replay", directory.to_str().ok_or("recording path")?]);
        invoke(&env, &args, 4)?;
    }
    let mut allowed = words.to_vec();
    allowed.extend(["--disable-sources", "restricted"]);
    invoke(&env, &allowed, 0)?;
    if stub.finish()?.len() != 1 {
        return Err("disabled source or warm cache made another request".into());
    }
    Ok(())
}
