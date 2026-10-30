//! Feature drug-native-sources: a wrong native identity never poisons the exact-ID cache.
use super::{
    configuration::invoke, drug_native::fixture_root, launch::Environment, network_stub::Stub,
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
type Check = Result<(), String>;
tool_only! { mod optional_values; }
pub(super) fn trials() -> Vec<Trial> {
    [
        ("chembl", "010.json", "/molecule_chembl_id", "www.ebi.ac.uk"),
        (
            "opentargets",
            "opentargets-detail-D01-imatinib.json",
            "/data/drug/id",
            "api.platform.opentargets.org",
        ),
    ]
    .into_iter()
    .map(|(source, file, pointer, host)| {
        transcript_trial(
            format!("drug-native-sources/{source}-identity-before-cache"),
            move || check(source, file, pointer, host).map_err(Into::into),
        )
    })
    .collect()
}
fn check(source: &'static str, file: &str, pointer: &str, host: &'static str) -> Check {
    let text = fs::read_to_string(fixture_root().join(file)).map_err(|e| e.to_string())?;
    let mut wrong: Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;
    *wrong
        .pointer_mut(pointer)
        .ok_or("native fixture identity")? = json!("CHEMBL1");
    let bad = serde_json::to_string(&wrong).map_err(|e| e.to_string())?;
    let mut first = true;
    let stub = Stub::for_source_status(
        host,
        "application/json",
        move |_| {
            if first {
                first = false;
                Ok(bad.clone())
            } else {
                Ok(text.clone())
            }
        },
        |_| 200,
    )?;
    let env = Environment::new().map_err(|e| e.to_string())?;
    let cert = env.root().join("ca.pem");
    stub.retain(&env);
    fs::write(&cert, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"ca_bundle":cert,"proxy":format!("http://{}",stub.address)}))
        .map_err(|e| e.to_string())?;
    let args = ["get", "drug", "CHEMBL941", "--source", source]
        .map(str::to_owned)
        .to_vec();
    let failed = invoke(&env, args.clone(), 4)?;
    if failed.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
        || failed.get("result").is_some()
    {
        return Err("mismatched native identity was accepted".into());
    }
    let correct = invoke(&env, args.clone(), 0)?;
    if correct.pointer("/result/header/id") != Some(&json!("CHEMBL941")) {
        return Err("correct native identity was not retrieved".into());
    }
    if super::mcp::call_with_globals(&env, &args, false, &[])? != correct {
        return Err("native cached card changed over MCP".into());
    }
    if stub.finish()?.len() != 2 {
        return Err("a wrong identity entered cache or a valid identity did not cache".into());
    }
    Ok(())
}

tool_only! {
pub(super) fn tool_trials() -> Vec<Trial> {
    vec![
        transcript_trial("drug-native-sources/opentargets-optional-name-keeps-identity", || optional_values::check().map_err(Into::into)),
        transcript_trial("drug-native-sources/opentargets-native-stage-preserved", || optional_values::stages().map_err(Into::into)),
    ]
}
}
