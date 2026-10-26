//! Feature phenotype-identity: literal candidates and independent source term cards.
mod native;
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
pub(super) use native::run as native;
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test(
            "phenotype-identity/literal-native-pages-and-default-card-pivots",
            || search().map_err(Into::into),
        ),
        Trial::test(
            "phenotype-identity/independent-complete-cards-and-native-deprecation",
            || cards().map_err(Into::into),
        ),
        Trial::test(
            "phenotype-identity/bad-identities-pages-and-outages-never-become-empty",
            || failures().map_err(Into::into),
        ),
    ]
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("phenotype-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn invoke(env: &Environment, words: &[&str], exit: i32) -> Result<Value, String> {
    let output = env
        .command(
            env!("CARGO_BIN_EXE_biomcp"),
            &words.iter().map(|s| s.to_string()).collect::<Vec<_>>(),
        )
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn node() -> Value {
    json!({"id":"HP:0000001","name":"Synthetic term","category":"biolink:PhenotypicFeature","namespace":"HP","description":"Complete synthetic description","deprecated":false,"has_descendant_count":3,"association_counts":[],"synonym":[],"xref":[]})
}
fn search() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            if !request.starts_with("GET /v3/api/search?q=literal%2C%20unchanged&category=biolink%3APhenotypicFeature&namespace=HP&limit=5&offset=0 ") {return Err(format!("literal query changed: {request}"));}
            let mut second = node();
            second
                .as_object_mut()
                .ok_or("node")?
                .insert("id".into(), json!("HP:0000002"));
            Ok(json!({"limit":5,"offset":0,"total":8,"items":[node(),second]}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let result = invoke(&env, &["search", "phenotype", "literal, unchanged"], 0)?;
    if result.pointer("/page/total") != Some(&json!(8))
        || result.pointer("/next/0/args") != Some(&json!(["get", "phenotype", "HP:0000001"]))
        || result.pointer("/next/1/args") != Some(&json!(["get", "phenotype", "HP:0000002"]))
    {
        return Err(format!("candidate order or default get changed: {result}"));
    }
    if result.pointer("/next/2/args")
        != Some(&json!([
            "search",
            "phenotype",
            "literal, unchanged",
            "--offset",
            "2"
        ]))
    {
        return Err(format!("native continuation changed: {result}"));
    }
    if stub.finish()?.len() != 1 {
        return Err("search fetched a candidate".into());
    }
    Ok(())
}
fn cards() -> Result<(), String> {
    for monarch in [false, true] {
        let stub = Stub::for_source_status(
            if monarch {
                "api-v3.monarchinitiative.org"
            } else {
                "ontology.jax.org"
            },
            "application/json",
            move |request| {
                let expected = if monarch {
                    "GET /v3/api/entity/HP:0000001 "
                } else {
                    "GET /api/hp/terms/HP:0000001 "
                };
                if !request.starts_with(expected) {
                    return Err(format!("wrong exact term URL: {request}"));
                }
                Ok(if monarch {node()} else {json!({"id":"HP:0000001","name":"Synthetic term","definition":"","comment":"Complete synthetic comment","publicationReferences":["PMID:1"],"descendantCount":2,"synonyms":[],"xrefs":[]})}.to_string())
            },
            |_| 200,
        )?;
        let env = setup(&stub)?;
        let source = if monarch { "monarch" } else { "hpo" };
        let result = invoke(
            &env,
            &["get", "phenotype", "HP:0000001", "--source", source],
            0,
        )?;
        if result.pointer("/result/term/source") != Some(&json!(source))
            || result.pointer("/result/term/returned_id") != Some(&json!("HP:0000001"))
        {
            return Err("source card merged or changed identity".into());
        }
        let (field, expected) = if monarch {
            ("description", json!("Complete synthetic description"))
        } else {
            ("definition", json!(""))
        };
        if result.pointer(&format!("/result/term/{field}")) != Some(&expected) {
            return Err("complete native text changed".into());
        }
        if stub.finish()?.len() != 1 {
            return Err("card fell back to another source".into());
        }
    }
    Ok(())
}
fn failures() -> Result<(), String> {
    for kind in 0..7 {
        let stub = Stub::for_source_status(
            "api-v3.monarchinitiative.org",
            "application/json",
            move |_| {
                let mut term = node();
                if kind == 0 {
                    term.as_object_mut()
                        .ok_or("node")?
                        .insert("id".into(), json!("MONDO:0000001"));
                }
                if kind >= 5 {
                    if kind == 6 {
                        term.as_object_mut()
                            .ok_or("node")?
                            .insert("id".into(), json!(null));
                    }
                    return Ok(term.to_string());
                }
                let mut page = json!({"limit":5,"offset":0,"total":1,"items":[term]});
                if kind == 1 {
                    page.as_object_mut()
                        .ok_or("page")?
                        .insert("offset".into(), json!(5));
                }
                if kind == 2 {
                    return Ok("{broken".into());
                }
                Ok(page.to_string())
            },
            move |_| match kind {
                3 => 429,
                4 => 500,
                _ => 200,
            },
        )?;
        let env = setup(&stub)?;
        let words = if kind >= 5 {
            vec!["get", "phenotype", "HP:0000002", "--source", "monarch"]
        } else {
            vec!["search", "phenotype", "synthetic"]
        };
        let result = invoke(&env, &words, 4)?;
        if result.get("status") != Some(&json!("failed")) {
            return Err(format!("source failure became absence: {result}"));
        }
        if matches!(kind, 0 | 1 | 2 | 5 | 6)
            && result
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(|p| p.is_empty())
        {
            return Err("bad reply lost its problem".into());
        }
        if stub.finish()?.is_empty() {
            return Err("failure did not reach the intended boundary".into());
        }
    }
    Ok(())
}
