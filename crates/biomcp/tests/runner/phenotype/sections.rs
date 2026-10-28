//! Feature phenotype-sections: source-owned text and direct link disclosure.
use super::{Stub, invoke, setup};
use libtest_mimic::Trial;
use serde_json::{Value, json};
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "phenotype-sections/refresh-preserves-card-snapshot-and-policy",
            || retained().map_err(Into::into),
        ),
        Trial::test("phenotype-sections/native-classes-and-direct-links", || {
            monarch().map_err(Into::into)
        }),
        Trial::test(
            "phenotype-sections/empty-unavailable-and-missing-retained-reply",
            || absent().map_err(Into::into),
        ),
        Trial::test(
            "phenotype-sections/retained-local-pages-and-executable-card-actions",
            || pages().map_err(Into::into),
        ),
    ]
}
fn pages() -> Result<(), String> {
    let stub = Stub::for_source("ontology.jax.org", "application/json", |request| {
        if !request.starts_with("GET /api/hp/terms/HP%3A0000001 ") {
            return Err(format!("wrong entity: {request}"));
        }
        Ok(json!({"id":"HP:0000001","name":"Synthetic term","synonyms":["First","Second","Third","Fourth","Fifth","Sixth"],"xrefs":[]}).to_string())
    })?;
    let env = setup(&stub)?;
    let card = invoke(
        "sections/pages/invoke1",
        &env,
        &["get", "phenotype", "HP:0000001"],
        0,
    )?;
    let words = step(&card, "section", 0)?;
    let words = words.iter().map(String::as_str).collect::<Vec<_>>();
    let page = invoke("sections/pages/invoke2", &env, &words, 0)?;
    if page.pointer("/page/returned") != Some(&json!(5))
        || page.pointer("/page/total") != Some(&json!(6))
        || page.pointer("/result/items/0/value") != Some(&json!("First"))
        || page.pointer("/result/items/0/relationship").is_some()
    {
        return Err(format!("source rows or five-row default changed: {page}"));
    }
    let words = step(&page, "page", 0)?;
    let words = words.iter().map(String::as_str).collect::<Vec<_>>();
    let terminal = invoke("sections/pages/invoke3", &env, &words, 0)?;
    if terminal.pointer("/page/offset") != Some(&json!(5))
        || terminal.pointer("/result/items/0/value") != Some(&json!("Sixth"))
        || terminal.get("next").is_some()
    {
        return Err(format!("local terminal changed: {terminal}"));
    }
    let explicit = invoke(
        "sections/pages/invoke4",
        &env,
        &["get", "phenotype", "HP:0000001", "--section-limit", "2"],
        0,
    )?;
    let explicit = step(&explicit, "section", 0)?;
    let explicit = explicit.iter().map(String::as_str).collect::<Vec<_>>();
    let limited = invoke("sections/pages/invoke5", &env, &explicit, 0)?;
    if limited.pointer("/query/section_limit") != Some(&json!(2))
        || limited.pointer("/page/returned") != Some(&json!(2))
    {
        return Err("card section action lost the explicit section limit".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("section walk refetched retained entity".into());
    }
    Ok(())
}
fn step(value: &Value, kind: &str, index: usize) -> Result<Vec<String>, String> {
    value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing next")?
        .iter()
        .filter(|row| row.get("kind").and_then(Value::as_str) == Some(kind))
        .nth(index)
        .and_then(|row| row.get("args"))
        .and_then(Value::as_array)
        .ok_or_else(|| format!("missing {kind} step: {value}"))?
        .iter()
        .map(|word| word.as_str().map(String::from).ok_or("step word".into()))
        .collect()
}
fn monarch() -> Result<(), String> {
    let stub = Stub::for_source("api-v3.monarchinitiative.org", "application/json", |_| {
        Ok(json!(
            { "id":"HP:0000001","name":"Synthetic term","category":"biolink:PhenotypicFeature","synonym":["Unclassified"],
            "exact_synonym":["Exact"],"broad_synonym":["Broad"],"narrow_synonym":[],"related_synonym":["Related"],"xref":["UMLS:C0000001"],
            "node_hierarchy":{"super_classes":[{"id":"HP:0000002","name":"Parent"}],"sub_classes":[{"id":"HP:0000003","name":"Child"}]} }
        )
        .to_string())
    })?;
    let env = setup(&stub)?;
    for (section, expected) in [
        ("synonyms", json!([null, "exact", "broad", "related"])),
        ("hierarchy", json!(["super_classes", "sub_classes"])),
    ] {
        let result = invoke(
            &format!("sections/monarch/invoke1@section={section}"),
            &env,
            &[
                "get",
                "phenotype",
                "HP:0000001",
                section,
                "--source",
                "monarch",
                "--section-limit",
                "2",
            ],
            0,
        )?;
        if result.pointer("/query/section_limit") != Some(&json!(2)) {
            return Err("explicit limit changed".into());
        }
        classes(&result, section, &expected)?;
        if section == "synonyms" {
            let words = step(&result, "page", 0)?;
            let words = words.iter().map(String::as_str).collect::<Vec<_>>();
            let next = invoke(
                &format!("sections/monarch/invoke2@section={section}"),
                &env,
                &words,
                0,
            )?;
            if next.pointer("/result/items/0/relationship") != Some(&json!("broad"))
                || next.pointer("/result/items/1/relationship") != Some(&json!("related"))
            {
                return Err("native class walk changed".into());
            }
        }
    }
    let result = invoke(
        "sections/monarch/invoke3",
        &env,
        &[
            "get",
            "phenotype",
            "HP:0000001",
            "xrefs",
            "--source",
            "monarch",
        ],
        0,
    )?;
    if result.pointer("/result/items/0/id") != Some(&json!("UMLS:C0000001"))
        || result.pointer("/result/items/0/predicate").is_some()
    {
        return Err("xref invented predicate".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("source sections bypassed checked entity cache".into());
    }
    Ok(())
}
fn absent() -> Result<(), String> {
    let stub = Stub::for_source("ontology.jax.org", "application/json", |_| {
        Ok(
            json!({"id":"HP:0000001","name":"Synthetic term","synonyms":[],"xrefs":null})
                .to_string(),
        )
    })?;
    let env = setup(&stub)?;
    let empty = invoke(
        "sections/absent/invoke1",
        &env,
        &["get", "phenotype", "HP:0000001", "synonyms"],
        1,
    )?;
    let missing = invoke(
        "sections/absent/invoke2",
        &env,
        &["get", "phenotype", "HP:0000001", "xrefs"],
        1,
    )?;
    if empty.pointer("/result/items").is_some()
        || empty.pointer("/page/total") != Some(&json!(0))
        || missing.get("result").is_some()
        || missing.get("page").is_some()
    {
        return Err("empty and unavailable sections collapsed".into());
    }
    invoke(
        "sections/absent/invoke3",
        &env,
        &[
            "get",
            "phenotype",
            "HP:0000001",
            "hierarchy",
            "--source",
            "hpo",
        ],
        2,
    )?;
    let lost = invoke(
        "sections/absent/invoke4",
        &env,
        &[
            "get",
            "phenotype",
            "HP:0000001",
            "synonyms",
            "--snapshot",
            "unknown",
        ],
        3,
    )?;
    if lost.pointer("/next/0/kind") != Some(&json!("restart")) {
        return Err(format!("missing snapshot refetched: {lost}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("invalid or retained selector made a provider request".into());
    }
    Ok(())
}

fn retained() -> Result<(), String> {
    let mut count = 0;
    let stub = Stub::for_sources(
        &["ontology.jax.org"],
        |_| "application/json",
        move |_| {
            count += 1;
            Ok(json!({"id":"HP:0000001","name":"Term","synonyms":[if count==1 {"Original"} else {"Refreshed"}]}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let card = invoke(
        "sections/retained/invoke1",
        &env,
        &["get", "phenotype", "HP:0000001"],
        0,
    )?;
    let args = step(&card, "section", 0)?;
    let words = args.iter().map(String::as_str).collect::<Vec<_>>();
    invoke(
        "sections/retained/invoke2",
        &env,
        &["--fresh", "get", "phenotype", "HP:0000001"],
        0,
    )?;
    let original = invoke("sections/retained/invoke3", &env, &words, 0)?;
    if original.pointer("/result/items/0/value") != Some(&json!("Original")) {
        return Err("refresh replaced retained source reply".into());
    }
    let mut denied = words.clone();
    denied.extend(["--disable-sources", "hpo"]);
    let denied = invoke("sections/retained/invoke4", &env, &denied, 4)?;
    if denied.get("result").is_some()
        || denied.pointer("/sources/0/reason") != Some(&json!("disabled"))
    {
        return Err("disabled source restored retained content".into());
    }
    let mut foreign = args;
    *foreign.get_mut(2).ok_or("identity arg")? = "HP:0000002".into();
    let foreign = foreign.iter().map(String::as_str).collect::<Vec<_>>();
    invoke("sections/retained/invoke5", &env, &foreign, 3)?;
    if stub.finish()?.len() != 2 {
        return Err("retained page made a provider request".into());
    }
    Ok(())
}

fn classes(result: &Value, section: &str, expected: &Value) -> Result<(), String> {
    let field = if section == "synonyms" {
        "relationship"
    } else {
        "relation"
    };
    let items = result
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("rows")?;
    let values = items
        .iter()
        .map(|v| v.get(field).cloned().unwrap_or(Value::Null))
        .collect::<Vec<_>>();
    if values
        != expected
            .as_array()
            .ok_or("expected")?
            .iter()
            .take(2)
            .cloned()
            .collect::<Vec<_>>()
    {
        return Err(format!("native classes or links changed: {result}"));
    }
    Ok(())
}
