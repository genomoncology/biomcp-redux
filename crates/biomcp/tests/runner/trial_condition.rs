//! Exact trial disease identity stops before trial I/O when resolution cannot supply a filter.
use super::{launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
mod native;
mod words;
pub(super) use native::run as native;

pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("trial-search/condition-complete-native-words", || {
            words::complete_conditions().map_err(Into::into)
        }),
        Trial::test("trial-search/condition-exact-boundary", || {
            words::exact_boundary().map_err(Into::into)
        }),
        Trial::test("trial-search/condition-word-mismatch", || {
            words::mismatch().map_err(Into::into)
        }),
        Trial::test("trial-search/condition-id-source-policy", || {
            policy().map_err(Into::into)
        }),
        Trial::test("trial-search/condition-id-failures-before-trial", || {
            failures().map_err(Into::into)
        }),
    ]
}
fn annotation(id: &str, codes: &[&str]) -> Value {
    json!({"_id":id,"mondo":{"mondo":id,"label":"Synthetic failure concept","xrefs":{"ncit":codes}}})
}
fn failures() -> Result<(), String> {
    for (case, body, expected) in [
        (
            "ambiguous",
            annotation("MONDO:0005105", &["C3224", "C3512"]),
            None,
        ),
        ("missing", annotation("MONDO:0005105", &[]), None),
        (
            "wrong-id",
            annotation("MONDO:0000001", &["C3224"]),
            Some("bad_reply"),
        ),
        (
            "resolver-error",
            json!({"error":"synthetic resolver failure"}),
            Some("bad_reply"),
        ),
    ] {
        let response = body.to_string();
        let stub = Stub::for_source_status(
            "mydisease.info",
            "application/json",
            move |request| {
                if !request
                    .starts_with("GET /v1/disease/MONDO%3A0005105?fields=mondo%2Cdisease_ontology ")
                {
                    return Err(format!("unexpected resolver or trial request: {request}"));
                }
                Ok(response.clone())
            },
            |_| 200,
        )?;
        let env = setup(&stub)?;
        let directory = env.root().join(case);
        let args = [
            "search",
            "trial",
            "--source",
            "nci",
            "--condition-id",
            "MONDO:0005105",
            "--limit",
            "1",
        ];
        let mut record: Vec<String> = args.iter().map(|v| (*v).into()).collect();
        record.extend(["--record".into(), directory.to_string_lossy().into()]);
        let value = invoke(&env, &record, 4)?;
        check_failure(case, expected, &value)?;
        if stub.finish()?.len() != 1 {
            return Err(format!("{case}: resolver fanout or trial request"));
        }
        use std::io::Write;
        writeln!(
            std::io::stderr(),
            "condition failure {case}: full={} comparable={}",
            serde_json::to_vec(&value).map_err(|e| e.to_string())?.len(),
            super::source_terms::size(&value, "")?
        )
        .map_err(|e| e.to_string())?;
        let ceiling = match case {
            "ambiguous" => 1592,
            "missing" => 983,
            _ => 496,
        };
        let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
        fs::write(
            file.path(),
            serde_json::to_vec(&json!(
                {"feature":"trial-search","story":"Unresolved exact identity stops before trial I/O","replay":directory,
                "config":{"keys":{"nci_api_key":"synthetic-trial-condition-key"}},"steps":[{"run":args,"mcp":args,"exit":4,"is_error":true,
                "expect":{"/status":"failed","/result/request":[]},"max_bytes":ceiling,"max_tokens":(ceiling+3)/4}]}
            ))
            .map_err(|e| e.to_string())?,
        )
        .map_err(|e| e.to_string())?;
        super::run_case(file.path(), &super::schema::Registry::embedded()?)?;
    }
    Ok(())
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("ca.pem");
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!(
        {"proxy":format!("http://{}",stub.address),"ca_bundle":ca,"keys":{"nci_api_key":"synthetic-trial-condition-key"}}
    ))
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
            "expected exit {exit}, got {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}

fn check_failure(case: &str, expected: Option<&str>, value: &Value) -> Result<(), String> {
    if value.pointer("/result/request") != Some(&json!([])) || value.get("page").is_some() {
        return Err(format!(
            "{case}: unresolved identity asserted a trial request/count"
        ));
    }
    if let Some(reason) = expected
        && value.pointer("/sources/0/reason") != Some(&json!(reason))
    {
        return Err(format!("{case}: lost resolver failure: {value}"));
    }
    let sources = value
        .get("sources")
        .and_then(Value::as_array)
        .ok_or("missing resolver outcome")?;
    if sources.len() != 1
        || sources.first().and_then(|s| s.get("source")) != Some(&json!("mydisease"))
    {
        return Err(format!("{case}: trial source ran after failed resolution"));
    }
    if case == "ambiguous"
        && value
            .pointer("/result/condition_resolution/ncit_candidates")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(2)
    {
        return Err("ambiguous candidates lost".into());
    }
    if value
        .pointer("/result/condition_resolution/selected_ncit")
        .is_some()
    {
        return Err("failed mapping selected an NCIt code".into());
    }
    Ok(())
}

fn policy() -> Result<(), String> {
    let stub = Stub::for_source("mydisease.info", "application/json", |_| {
        Err("disabled command reached a provider".into())
    })?;
    let env = setup(&stub)?;
    for (source, disabled, expected) in [
        ("nci", "nci", "nci"),
        ("ctgov", "ctgov", "ctgov"),
        ("nci", "mydisease", "mydisease"),
        ("ctgov", "mondo,doid", "mydisease"),
    ] {
        let args: Vec<String> = [
            "search",
            "trial",
            "--source",
            source,
            "--condition-id",
            "MONDO:0005105",
            "--disable-sources",
            disabled,
        ]
        .iter()
        .map(|v| (*v).into())
        .collect();
        let value = invoke(&env, &args, 4)?;
        if value.pointer("/sources/0/source") != Some(&json!(expected))
            || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
        {
            return Err(format!(
                "source policy did not stop before resolution/trial I/O: {value}"
            ));
        }
    }
    if !stub.finish()?.is_empty() {
        return Err("disabled condition source reached the wire".into());
    }
    Ok(())
}
