//! Feature disease-mydisease: exact annotation without fallback or hidden enrichment.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, path::Path};
mod cases;
tool_only! { mod native; }
tool_only! { pub(super) use native::run as native; }
pub(super) fn trials() -> Vec<Trial> {
    let mut tests = vec![transcript_trial(
        "disease-mydisease/exact-alias-and-independent-pages",
        || exact().map_err(Into::into),
    )];
    tests.extend(cases::trials());
    tests
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("mydisease-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| (*s).into()).collect()
}
fn insert(value: &mut Value, key: &str, item: Value) -> Result<(), String> {
    value
        .as_object_mut()
        .ok_or("missing object")?
        .insert(key.into(), item);
    Ok(())
}
fn invoke(env: &Environment, words: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), words)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "unexpected exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn replay(directory: &Path, mut steps: Value) -> Result<Value, String> {
    for step in steps.as_array_mut().ok_or("missing case steps")? {
        let ceiling = ceiling(step)?;
        insert(step, "max_bytes", json!(ceiling))?;
        insert(step, "max_tokens", json!(ceiling.div_ceil(4)))?;
    }
    let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    fs::write(
        file.path(),
        serde_json::to_vec(&json!({"feature":"disease-mydisease",
"story":"Selected source facts stay complete and independently paged",
"replay":directory,
"steps":steps}))
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    super::run_case(file.path(), super::schema::Registry::embedded()?.as_ref())
}
fn ceiling(step: &Value) -> Result<usize, String> {
    // Ceil(max measured comparable bytes by command/page shape * 1.15), j12127/j12134.
    let words = step
        .get("run")
        .and_then(Value::as_array)
        .ok_or("missing command")?;
    let section = words.get(3).and_then(Value::as_str);
    let limit = words.windows(2).find_map(|w| {
        let (key, value) = (w.first()?.as_str()?, w.get(1)?.as_str()?);
        (key == "--section-limit").then_some(value)
    });
    Ok(match (section, limit) {
        (Some("synonyms"), Some("2")) => {
            crate::runner::output_limit("disease_mydisease-measured-1237")?
        }
        (Some("xrefs"), Some("2")) => {
            crate::runner::output_limit("disease_mydisease-measured-1374")?
        }
        (Some("hierarchy"), Some("2")) => {
            crate::runner::output_limit("disease_mydisease-measured-1385")?
        }
        (Some("synonyms"), Some("50")) => {
            crate::runner::output_limit("disease_mydisease-measured-2230")?
        }
        (Some("xrefs"), Some("50")) => {
            crate::runner::output_limit("disease_mydisease-measured-5619")?
        }
        (Some("hierarchy"), Some("50")) => {
            crate::runner::output_limit("disease_mydisease-measured-1796")?
        }
        (Some("synonyms"), _) => crate::runner::output_limit("disease_mydisease-measured-1889")?,
        (Some("xrefs"), _) => crate::runner::output_limit("disease_mydisease-measured-2200")?,
        (Some("hierarchy"), _) => crate::runner::output_limit("disease_mydisease-measured-1794")?,
        _ => crate::runner::output_limit("disease_mydisease-measured-2024")?,
    })
}
fn record(env: &Environment, words: &[&str], directory: &Path, exit: i32) -> Result<Value, String> {
    let mut command = args(words);
    command.extend(["--record".into(), directory.to_string_lossy().into()]);
    let value = invoke(env, &command, exit)?;
    measure(words, &value)?;
    Ok(value)
}
fn measure(words: &[&str], value: &Value) -> Result<(), String> {
    use std::io::Write;
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    writeln!(
        std::io::stderr(),
        "MyDisease {:?}: full={} comparable={}",
        words,
        bytes,
        super::source_terms::size(value, "")?
    )
    .map_err(|e| e.to_string())
}
fn annotation() -> Value {
    json!(
        {"_id":"MONDO:0005061","_version":1,"mondo":{"mondo":"MONDO:0005061","label":"Synthetic lung concept",
        "definition":"Complete synthetic definition, including its original punctuation. [Native reference]",
        "synonym":{"exact":["first native synonym","second native synonym"],"related":["related wording"]},"xrefs":{"doid":["DOID:3910",
        "DOID:0111503"],"efo":["EFO:0000571"],"icd10cm":["C34.9"],"oncotree":["LUAD"]},"parents":["MONDO:0004970","MONDO:0005233"]},
        "disease_ontology":{"doid":"DOID:3910","name":"Different native DOID label","def":"Separate native DOID definition.",
        "synonyms":{"exact":["DOID synonym"]},"xrefs":{"ncit":["C27745","C3512"],"snomedct_us_2025_09_01":"254626006"},
        "parents":["DOID:299","DOID:3908"]}}
    )
}
fn reply(request: &str) -> Result<String, String> {
    if !request.starts_with("GET /v1/disease/DOID%3A3910?fields=mondo%2Cdisease_ontology ") {
        return Err(format!("unexpected source request: {request}"));
    }
    Ok(annotation().to_string())
}
fn record_exact_card(env: &Environment, directory: &std::path::Path) -> Result<(), String> {
    record(env, &["get", "disease", "DOID:3910"], directory, 0)?;
    replay(
        directory,
        json!(
            [{"run":["get","disease","DOID:3910"],"mcp":["get","disease","DOID:3910"],"expect":{"/result/requested_id":"DOID:3910",
            "/result/returned_id":"MONDO:0005061","/result/alias/object/id":"DOID:3910",
            "/result/definition":"Complete synthetic definition, including its original punctuation. [Native reference]",
            "/result/sections":["synonyms","xrefs","hierarchy"]}}]
        ),
    )?;
    Ok(())
}
fn exact() -> Result<(), String> {
    let stub = Stub::for_source("mydisease.info", "application/json", reply)?;
    let env = setup(&stub)?;
    let directory = env.root().join("exact");
    record_exact_card(&env, &directory)?;
    for (section, total, pointer, expected) in [
        (
            "synonyms",
            4,
            "/result/items/0/scope",
            json!("mydisease:mondo.synonym.exact"),
        ),
        ("xrefs", 8, "/result/items/0/object/id", json!("DOID:3910")),
        (
            "hierarchy",
            4,
            "/result/items/0/object/id",
            json!("MONDO:0004970"),
        ),
    ] {
        let words = [
            "get",
            "disease",
            "DOID:3910",
            section,
            "--source",
            "mydisease",
            "--section-limit",
            "2",
        ];
        let mut expect = json!({"/page/total":total,"/page/returned":2,"/next/0/kind":"page"});
        expect
            .as_object_mut()
            .ok_or("missing expectations")?
            .insert(pointer.into(), expected);
        let value = replay(
            &directory,
            json!([{"run":words,"mcp":words,"expect":expect}]),
        )?;
        measure(&words, &value)?;
        let terminal = (total - 2).to_string();
        let words = [
            "get",
            "disease",
            "DOID:3910",
            section,
            "--source",
            "mydisease",
            "--section-limit",
            "2",
            "--section-offset",
            &terminal,
        ];
        let value = replay(
            &directory,
            json!([{"run":words,"mcp":words,"expect":{"/page/offset":total-2,"/page/returned":2,"/page/total":total}}]),
        )?;
        if value.get("next").is_some() {
            return Err("terminal page advertised continuation".into());
        }
        if section == "hierarchy"
            && value
                .pointer("/result/items/0/subject/id")
                .and_then(Value::as_str)
                != Some("DOID:3910")
        {
            return Err("component parent provenance was merged".into());
        }
    }
    if stub.finish()?.len() != 1 {
        return Err("unexpected source fanout".into());
    }
    Ok(())
}

tool_only! {}
