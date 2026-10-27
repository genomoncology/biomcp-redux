//! Feature disease-ols: literal ontology discovery and exact source concepts.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::io::Write;
use std::{fs, path::Path};
mod native;
pub(super) use native::run as native;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("disease-ols/retired-spellings-never-open-source", || {
            retired_requests().map_err(Into::into)
        }),
        Trial::test(
            "disease-ols/literal-zero-never-repairs-to-exact-get",
            || literal_zero().map_err(Into::into),
        ),
        Trial::test("disease-ols/max-page-keeps-all-candidate-actions", || {
            maximum().map_err(Into::into)
        }),
        Trial::test("disease-ols/native-order-and-literal-selectors", || {
            search().map_err(Into::into)
        }),
        Trial::test(
            "disease-ols/exact-identity-and-independent-sections",
            || exact().map_err(Into::into),
        ),
        Trial::test("disease-ols/refuses-wrong-record-before-cache", || {
            wrong().map_err(Into::into)
        }),
        Trial::test("disease-ols/empty-is-not-provider-failure", || {
            failures().map_err(Into::into)
        }),
        Trial::test("disease-ols/timeout-retains-source-terms", || {
            timeout().map_err(Into::into)
        }),
        Trial::test("disease-ols/disabled-component-bypasses-cache", || {
            disabled().map_err(Into::into)
        }),
    ]
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ols-ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn invoke(env: &Environment, args: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "wrong exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| (*s).into()).collect()
}
fn record(env: &Environment, words: &[&str], directory: &Path) -> Result<Value, String> {
    let mut command = args(words);
    command.extend(["--record".into(), directory.to_string_lossy().into()]);
    let result = invoke(env, &command, 0)?;
    writeln!(
        std::io::stderr(),
        "disease measurement {:?}: {} JSON bytes",
        words,
        serde_json::to_vec(&result)
            .map_err(|e| e.to_string())?
            .len()
    )
    .map_err(|e| e.to_string())?;
    Ok(result)
}
fn replay(directory: &Path, steps: Value) -> Result<Value, String> {
    let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    fs::write(
        file.path(),
        serde_json::to_vec(&json!(
            {"feature":"disease-ols","story":"Source disease facts retain identity, order and pagination","replay":directory,"steps":steps}
        ))
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let value = super::run_case(file.path(), &super::schema::Registry::embedded()?)?;
    writeln!(
        std::io::stderr(),
        "OLS replay: full={} comparable={}",
        serde_json::to_vec(&value).map_err(|e| e.to_string())?.len(),
        super::source_terms::size(&value, "")?
    )
    .map_err(|e| e.to_string())?;
    Ok(value)
}
fn markdown(
    env: &Environment,
    words: &[&str],
    directory: &Path,
    required: &[&str],
) -> Result<(), String> {
    let mut command = args(words);
    command.extend([
        "--replay".into(),
        directory.to_string_lossy().into(),
        "--format".into(),
        "md".into(),
    ]);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &command)
        .output()
        .map_err(|e| e.to_string())?;
    let text = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    if !output.status.success() || required.iter().any(|value| !text.contains(value)) {
        return Err(format!("Markdown lost source facts: {text}"));
    }
    Ok(())
}

fn search_reply(request: &str) -> Result<String, String> {
    let offset = if request.contains("start=5") {
        5
    } else if request.contains("start=10") {
        10
    } else {
        0
    };
    let ontology = if request.contains("ontology=doid") {
        "doid"
    } else if request.contains("ontology=ncit") {
        "ncit"
    } else {
        "mondo"
    };
    let prefix = ontology.to_uppercase();
    let (limit, total) = if request.contains("rows=50") {
        (50, 51)
    } else {
        (5, 11)
    };
    let rows = (offset..(offset + limit).min(total))
        .map(|i| {
            json!(
                {"obo_id":format!("{prefix}:{:07}",i+1),"label":format!("Synthetic cancer {i}"),"ontology_name":ontology,
                "iri":format!("http://example.org/{i}"),"is_obsolete":i==3}
            )
        })
        .collect::<Vec<_>>();
    Ok(json!({"response":{"numFound":total,"start":offset,"docs":rows}}).to_string())
}
fn search() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", search_reply)?;
    let env = setup(&stub)?;
    for (index, ontology) in ["mondo", "doid", "ncit"].iter().enumerate() {
        let directory = env.root().join(format!("search-{index}"));
        let mut words = vec![
            "search",
            "disease",
            "  literal MF : cancer  ",
            "--ontology",
            ontology,
        ];
        record(&env, &words, &directory)?;
        let result = replay(
            &directory,
            json!(
                [{"run":words,"mcp":words,"expect":{"/query/text":"  literal MF : cancer  ","/query/ontology":ontology,"/page/total":11,
                "/result/items/0/concept/id":format!("{}:0000001",ontology.to_uppercase()),"/next/5/kind":"page"},
                "max_bytes":match *ontology {"mondo"=>2129,"doid"=>2244,_=>2263}}]
            ),
        )?;
        if result
            .pointer("/result/items")
            .unwrap_or(&Value::Null)
            .as_array()
            .is_none_or(|r| r.len() != 5)
        {
            return Err("native page changed".into());
        }
        words.clear();
    }
    let requests = stub.finish()?;
    if requests.len() != 3
        || requests.iter().any(|(_, r)| {
            !r.contains("q=%20%20literal%20MF%20%3A%20cancer%20%20")
                || !r.contains("groupField=iri")
        })
    {
        return Err(format!("literal or ontology request changed: {requests:?}"));
    }
    Ok(())
}
fn term(id: &str) -> Value {
    json!(
        {"obo_id":id,"label":"Synthetic obsolete cancer","ontology_name":"mondo","iri":"http://purl.obolibrary.org/obo/MONDO_0007607",
        "description":["Complete synthetic definition."],"is_obsolete":true,"term_replaced_by":null,
        "annotation":{"consider":["http://purl.obolibrary.org/obo/MONDO_0800444","http://purl.obolibrary.org/obo/MONDO_0800445"]},
        "synonyms":["native one","native two"],"obo_synonym":[{"name":"native one","scope":"hasExactSynonym"}],
        "obo_xref":[{"database":"DOID","id":"3910","description":"native text without inferred equivalence"},{"database":"NCI","id":"C3512",
        "description":null}]}
    )
}
fn exact_reply(_: &str) -> Result<String, String> {
    Ok(json!({"_embedded":{"terms":[term("MONDO:0007607")]}}).to_string())
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", exact_reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("exact");
    let words = ["get", "disease", "MONDO:0007607", "--source", "ols"];
    record(&env, &words, &directory)?;
    markdown(
        &env,
        &words,
        &directory,
        &[
            "## Result",
            "MONDO:0007607",
            "Complete synthetic definition.",
            "MONDO:0800444",
            "MONDO:0800445",
            "biomcp get disease",
        ],
    )?;
    let mut steps = vec![json!(
        {"run":words,"mcp":words,"expect":{"/result/requested_id":"MONDO:0007607","/result/replaced_by":null,
        "/result/consider":["MONDO:0800444","MONDO:0800445"],"/result/obsolete":true,"/result/sections":["synonyms","xrefs"]},
        "max_bytes":1086}
    )];
    for section in ["synonyms", "xrefs"] {
        let command = [
            "get",
            "disease",
            "MONDO:0007607",
            section,
            "--source",
            "ols",
            "--section-limit",
            "1",
        ];
        let mut expect = json!({"/page/total":2,"/page/returned":1,"/next/0/kind":"page"});
        let fields = expect.as_object_mut().ok_or("missing expectations")?;
        if section == "synonyms" {
            fields.insert(
                "/result/items/0/scope".into(),
                json!("oboInOwl:hasExactSynonym"),
            );
        } else {
            fields.insert(
                "/result/items/0/object".into(),
                json!({"equals":{"id":"DOID:3910"}}),
            );
            fields.insert(
                "/result/items/0/description".into(),
                json!("native text without inferred equivalence"),
            );
        }
        steps.push(json!({"run":command,"mcp":command,"expect":expect,"max_bytes":if section=="synonyms" {862}else{1002}}));
        let mut command = args(&command);
        command.extend(["--section-offset".into(), "1".into()]);
        steps
            .push(json!({"run":command,"mcp":command,"expect":{"/page/offset":1,"/page/total":2}}));
    }
    let terminal = replay(&directory, Value::Array(steps))?;
    if terminal.get("next").is_some() {
        return Err("terminal section advertised another page".into());
    }
    if stub.finish()?.len() != 1 {
        return Err("exact section paging fetched again".into());
    }
    Ok(())
}
fn wrong_reply(_: &str) -> Result<String, String> {
    Ok(json!({"_embedded":{"terms":[term("MONDO:9999999")]}}).to_string())
}
fn wrong() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", wrong_reply)?;
    let env = setup(&stub)?;
    for _ in 0..2 {
        let value = invoke(
            &env,
            &args(&["get", "disease", "MONDO:0007607", "--source", "ols"]),
            4,
        )?;
        if super::source_terms::size(&value, "")? > 365 {
            return Err("wrong OLS identity exceeds its measured bound".into());
        }
        if value.pointer("/status").unwrap_or(&Value::Null) != "failed"
            || value.get("result").is_some()
            || value.pointer("/problems/0/field").unwrap_or(&Value::Null) != "record"
        {
            return Err(format!("wrong record admitted: {value}"));
        }
    }
    if stub.finish()?.len() != 2 {
        return Err("wrong identity entered cache".into());
    }
    Ok(())
}
fn failures() -> Result<(), String> {
    for (status, body) in [
        (
            200,
            "{\"response\":{\"numFound\":0,\"start\":0,\"docs\":[]}}",
        ),
        (429, "{}"),
        (500, "{}"),
        (200, "broken JSON"),
    ] {
        let stub = Stub::for_source_status(
            "www.ebi.ac.uk",
            "application/json",
            move |_| Ok(body.into()),
            move |_| status,
        )?;
        let env = setup(&stub)?;
        let directory = env.root().join("failure-record");
        let absent = status == 200 && body.starts_with('{');
        let exit = if absent { 1 } else { 4 };
        let words = ["search", "disease", "empty"];
        let mut q = args(&words);
        q.extend(["--record".into(), directory.to_string_lossy().into()]);
        invoke(&env, &q, exit)?;
        stub.finish()?;
        let expected = if absent { "absent" } else { "failed" };
        let reason = match status {
            429 => Some("rate_limited"),
            500 => Some("unavailable"),
            _ if !absent => Some("bad_reply"),
            _ => None,
        };
        let mut expect = json!({"/status":expected,"/sources/0/source":"ols","/sources/0/terms":"EMBL-EBI Terms of Use; ontology-specific rights"});
        if let Some(reason) = reason {
            expect
                .as_object_mut()
                .ok_or("missing expectation")?
                .insert("/sources/0/reason".into(), json!(reason));
        }
        replay(
            &directory,
            json!(
                [{"run":words,"mcp":words,"exit":exit,"is_error":!absent,"expect":expect,"max_bytes":if absent {454} else {match status {429=>234,
                500=>233,_=>230}}}]
            ),
        )?;
    }
    Ok(())
}

fn timeout() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", search_reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("timeout");
    let words = ["search", "disease", "synthetic"];
    record(&env, &words, &directory)?;
    stub.finish()?;
    for file in fs::read_dir(directory.join("ols")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        *value
            .pointer_mut("/exchanges/0/delay_ms")
            .ok_or("missing exchange delay")? = json!(20000);
        fs::write(
            file.path(),
            serde_json::to_vec(&value).map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
    }
    replay(
        &directory,
        json!(
            [{"run":words,"mcp":words,"exit":4,"is_error":true,"expect":{"/status":"failed","/sources/0/source":"ols",
            "/sources/0/state":"failed","/sources/0/reason":"deadline_exceeded"},"max_bytes":244}]
        ),
    )?;
    Ok(())
}
fn disabled() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", exact_reply)?;
    let env = setup(&stub)?;
    let words = ["get", "disease", "MONDO:0007607", "--source", "ols"];
    invoke(&env, &args(&words), 0)?;
    for disabled in ["ols", "mondo"] {
        let mut q = args(&words);
        q.extend(["--disable-sources".into(), disabled.into()]);
        let value = invoke(&env, &q, 4)?;
        if value.pointer("/sources/0/reason").and_then(Value::as_str) != Some("disabled")
            || value.get("result").is_some()
        {
            return Err(format!("disabled component leaked stored rows: {value}"));
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("disabled component made a request".into());
    }
    Ok(())
}

fn maximum() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", search_reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("maximum");
    let words = ["search", "disease", "synthetic", "--limit", "50"];
    record(&env, &words, &directory)?;
    replay(
        &directory,
        json!(
            [{"run":words,"mcp":words,"expect":{"/page/returned":50,"/page/total":51,"/next":{"count":51},"/next/49/kind":"disambiguate",
            "/next/50/kind":"page"},"max_bytes":16130}]
        ),
    )?;
    invoke(
        &env,
        &args(&["search", "disease", "synthetic", "--limit", "51"]),
        2,
    )?;
    if stub.finish()?.len() != 1 {
        return Err("invalid page limit reached source".into());
    }
    Ok(())
}

fn literal_zero() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", |request| {
        if request.starts_with("GET /ols4/api/search?") {
            Ok(json!({"response":{"numFound":0,"start":0,"docs":[]}}).to_string())
        } else {
            exact_reply(request)
        }
    })?;
    let env = setup(&stub)?;
    let directory = env.root().join("literal-zero");
    let mut words = args(&["search", "disease", "MONDO:0007607"]);
    words.extend(["--record".into(), directory.to_string_lossy().into()]);
    let value = invoke(&env, &words, 1)?;
    if value.pointer("/query/text") != Some(&json!("MONDO:0007607")) {
        return Err("literal input changed".into());
    }
    let exact_dir = env.root().join("explicit-exact");
    record(
        &env,
        &["get", "disease", "MONDO:0007607", "--source", "ols"],
        &exact_dir,
    )?;
    replay(
        &directory,
        json!(
            [{"run":["search","disease","MONDO:0007607"],"mcp":["search","disease","MONDO:0007607"],"exit":1,"expect":{"/status":"absent",
            "/page/total":0,"/result/items":[]},"max_bytes":463}]
        ),
    )?;
    replay(
        &exact_dir,
        json!(
            [{"run":["get","disease","MONDO:0007607","--source","ols"],"mcp":["get","disease","MONDO:0007607","--source","ols"],
            "expect":{"/result/concept/id":"MONDO:0007607"},"max_bytes":1087}]
        ),
    )?;
    let requests = stub.finish()?;
    if !matches!(requests.as_slice(), [(_, search), (_, get)] if search.contains("/search?") && get.contains("/terms?obo_id="))
    {
        return Err(format!(
            "zero result triggered repair or hidden lookup: {requests:?}"
        ));
    }
    Ok(())
}

fn retired_requests() -> Result<(), String> {
    let stub = Stub::for_sources(
        &[
            "www.ebi.ac.uk",
            "mydisease.info",
            "api-v3.monarchinitiative.org",
            "seer.cancer.gov",
        ],
        |_| "application/json",
        |_| Err("retired input opened a source".into()),
    )?;
    let env = setup(&stub)?;
    let path = super::launch::fixtures().join("cases/disease/retired.yaml");
    let case: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    for step in case
        .get("steps")
        .and_then(Value::as_array)
        .ok_or("missing cases")?
    {
        let words = step
            .get("run")
            .and_then(Value::as_array)
            .ok_or("missing command")?
            .iter()
            .map(|w| w.as_str().map(String::from).ok_or("invalid word"))
            .collect::<Result<Vec<_>, _>>()?;
        let output = invoke(&env, &words, 2)?;
        if output.get("sources").is_some() {
            return Err("retired input reached source outcomes".into());
        }
    }
    if !stub.finish()?.is_empty() {
        return Err("retired input made a request".into());
    }
    Ok(())
}
