//! Feature phenotype-identity: literal candidates and independent source term cards.
mod association_native;
mod associations;
mod budgets;
mod faults;
mod native;
mod section_failures;
mod section_native;
mod sections;
mod states;
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
tool_only! { pub(super) use native::run as native; }
use serde_json::{Value, json};
use std::fs;
pub(super) fn trials() -> Vec<Trial> {
    let directory = super::launch::fixtures().join("phenotype-section-native");
    let absent = !directory.is_dir();
    let mut rows = vec![
        transcript_trial(
            "phenotype-sections/retained-native-witnesses-and-costs",
            move || section_native::run(&directory).map_err(Into::into),
        )
        .with_ignored_flag(absent),
        transcript_trial("phenotype-identity/generated-help-matches-markdown", || {
            help().map_err(Into::into)
        }),
        transcript_trial(
            "phenotype-identity/literal-native-pages-and-default-card-pivots",
            || search().map_err(Into::into),
        ),
        transcript_trial(
            "phenotype-identity/independent-complete-cards-and-native-deprecation",
            || cards().map_err(Into::into),
        ),
        transcript_trial(
            "phenotype-identity/bad-identities-pages-and-outages-never-become-empty",
            || failures().map_err(Into::into),
        ),
        transcript_trial(
            "phenotype-identity/disabled-before-cache-and-replay",
            || disabled().map_err(Into::into),
        ),
        transcript_trial(
            "phenotype-identity/recorded-timeout-retains-source-and-terms",
            || timeout().map_err(Into::into),
        ),
    ];
    rows.extend(faults::trials());
    rows.extend(states::trials());
    rows.extend(sections::trials());
    rows.extend(associations::trials());
    let directory = super::launch::fixtures().join("phenotype-association-native");
    let absent = !directory.is_dir();
    rows.push(
        transcript_trial(
            "phenotype-associations/retained-reverse-pages-and-costs",
            move || association_native::run(&directory).map_err(Into::into),
        )
        .with_ignored_flag(absent),
    );
    rows.push(transcript_trial(
        "phenotype-sections/recorded-failures-retain-source-and-terms",
        || section_failures::run().map_err(Into::into),
    ));
    rows
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("phenotype-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn invoke(phase: &str, env: &Environment, words: &[&str], exit: i32) -> Result<Value, String> {
    let value = raw_invoke(env, words, exit)?;
    budgets::check(&value, phase, "cli")?;
    Ok(value)
}
fn raw_invoke(env: &Environment, words: &[&str], exit: i32) -> Result<Value, String> {
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
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    Ok(value)
}
fn node() -> Value {
    json!(
        {"id":"HP:0000001","name":"Synthetic term","category":"biolink:PhenotypicFeature","namespace":"HP",
        "description":"Complete synthetic description","deprecated":false,"has_descendant_count":3,"association_counts":[],"synonym":[],
        "xref":[]}
    )
}
fn search() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        |request| {
            if !request.starts_with(
            "GET \
                /v3/api/search?q=literal%2C%20unchanged&category=biolink%3APhenotypicFeature&namespace=HP&limit=5&offset=0 ",
        ) {
            return Err(format!("literal query changed: {request}"));
        }
            let mut second = node();
            second
                .as_object_mut()
                .ok_or("node")?
                .insert("id".into(), json!("HP:0000002"));
            Ok(json!({"limit":5,"offset":0,"total":8,"items":[node(),second]}).to_string())
        },
    )?;
    let env = setup(&stub)?;
    let result = invoke(
        "mod/search/invoke1",
        &env,
        &["search", "phenotype", "literal, unchanged"],
        0,
    )?;
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
                    "GET /v3/api/entity/HP%3A0000001 "
                } else {
                    "GET /api/hp/terms/HP%3A0000001 "
                };
                if !request.starts_with(expected) {
                    return Err(format!("wrong exact term URL: {request}"));
                }
                Ok(if monarch {
                    node()
                } else {
                    json!(
                        {"id":"HP:0000001","name":"Synthetic term","definition":"","comment":"Complete synthetic comment",
                        "publicationReferences":["PMID:1"],"descendantCount":2,"synonyms":[],"xrefs":[]}
                    )
                }
                .to_string())
            },
            |_| 200,
        )?;
        let env = setup(&stub)?;
        let source = if monarch { "monarch" } else { "hpo" };
        let result = invoke(
            &format!("mod/cards/invoke1@source={source}"),
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
                let fields = term.as_object_mut().ok_or("node")?;
                if kind == 0 {
                    fields.insert("id".into(), json!("MONDO:0000001"));
                }
                if kind >= 5 {
                    if kind == 6 {
                        fields.insert("id".into(), json!(null));
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
        let directory = env.root().join("failure");
        let result = record(
            &format!("mod/failures/record1@kind={kind}"),
            &env,
            &words,
            &directory,
            if kind == 0 { 0 } else { 4 },
        )?;
        budgets::mcp(
            &format!("mod/failures/mcp1@kind={kind}"),
            &directory,
            &words,
            &result,
            if kind == 0 { 0 } else { 4 },
        )?;
        if kind == 5 {
            invoke(
                &format!("mod/failures/invoke1@kind={kind}"),
                &env,
                &words,
                4,
            )?;
        }
        faults::outcome(&result, "monarch", kind == 0)?;
        if matches!(kind, 0 | 1 | 2 | 5 | 6)
            && result
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(|p| p.is_empty())
        {
            return Err("bad reply lost its problem".into());
        }
        let count = stub.finish()?.len();
        if (kind == 5 && count != 2) || count == 0 {
            return Err("failure entered the cache or changed its bounded request count".into());
        }
    }
    Ok(())
}

fn record(
    phase: &str,
    env: &Environment,
    words: &[&str],
    directory: &std::path::Path,
    exit: i32,
) -> Result<Value, String> {
    let mut args = words.to_vec();
    args.extend(["--record", directory.to_str().ok_or("record path")?]);
    invoke(phase, env, &args, exit)
}
fn successful_page(_: &str) -> Result<String, String> {
    Ok(json!({"limit":5,"offset":0,"total":1,"items":[node()]}).to_string())
}
fn disabled() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        successful_page,
    )?;
    let env = setup(&stub)?;
    let words = ["search", "phenotype", "synthetic"];
    let directory = env.root().join("recorded");
    record("mod/disabled/record1", &env, &words, &directory, 0)?;
    for component in ["monarch", "hpo", "orphanet"] {
        let mut args = words.to_vec();
        args.extend(["--disable-sources", component]);
        let result = invoke(
            &format!("mod/disabled/invoke1@component={component}"),
            &env,
            &args,
            4,
        )?;
        if result.pointer("/sources/0/state") != Some(&json!("skipped"))
            || result.get("result").is_some()
        {
            return Err("disabled source leaked cached headers".into());
        }
        args.extend(["--replay", directory.to_str().ok_or("record path")?]);
        let result = invoke(
            &format!("mod/disabled/invoke2@component={component}"),
            &env,
            &args,
            4,
        )?;
        if result.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err("disabled replay leaked".into());
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled source reached network".into());
    }
    Ok(())
}
fn timeout() -> Result<(), String> {
    let stub = Stub::for_source(
        "api-v3.monarchinitiative.org",
        "application/json",
        successful_page,
    )?;
    let env = setup(&stub)?;
    let words = ["search", "phenotype", "synthetic"];
    let directory = env.root().join("recorded");
    record("mod/timeout/record1", &env, &words, &directory, 0)?;
    stub.finish()?;
    for entry in fs::read_dir(directory.join("monarch")).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(entry.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        *value
            .pointer_mut("/exchanges/0/delay_ms")
            .ok_or("recorded delay")? = json!(20000);
        fs::write(
            entry.path(),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    let mut args = words.to_vec();
    args.extend(["--replay", directory.to_str().ok_or("record path")?]);
    let result = invoke("mod/timeout/invoke1", &env, &args, 4)?;
    if result.get("status") != Some(&json!("failed"))
        || result.pointer("/sources/0/reason") != Some(&json!("deadline-exceeded"))
        || result.pointer("/sources/0/terms") != Some(&json!("Monarch source-specific licensing"))
    {
        return Err("timeout lost source failure or terms".into());
    }
    Ok(())
}

fn help() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    use std::io::Write;
    let whole = invoke("mod/help/invoke1", &env, &["help"], 0)?;
    writeln!(
        std::io::stderr(),
        "phenotype whole-help comparable={}",
        super::source_terms::size(&whole, "")?
    )
    .map_err(|e| e.to_string())?;
    for verb in ["search", "get"] {
        let result = invoke(
            &format!("mod/help/invoke2@verb={verb}"),
            &env,
            &["help", verb, "phenotype"],
            0,
        )?;
        let words = ["help", verb, "phenotype", "--format", "md"].map(String::from);
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &words)
            .output()
            .map_err(|e| e.to_string())?;
        if !output.status.success() {
            return Err("phenotype Markdown help failed".into());
        }
        let markdown = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
        budgets::bytes(
            markdown.len(),
            &format!("mod/help/markdown@verb={verb}"),
            "markdown",
        )?;
        writeln!(
            std::io::stderr(),
            "phenotype {verb} help comparable={} markdown={}",
            super::source_terms::size(&result, "")?,
            markdown.len()
        )
        .map_err(|e| e.to_string())?;
        for argument in result
            .pointer("/result/commands/0/arguments")
            .and_then(Value::as_array)
            .ok_or("help arguments")?
        {
            let name = argument
                .get("name")
                .and_then(Value::as_str)
                .ok_or("argument name")?;
            if !markdown.contains(name) {
                return Err(format!("Markdown help omitted {name}"));
            }
        }
    }
    Ok(())
}
