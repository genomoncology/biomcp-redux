//! Feature gwas-search: source selectors cross the real command boundary.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
mod mechanics;
mod native;
mod wire;
pub(super) fn trials() -> Vec<Trial> {
    let native_directory = super::launch::fixtures().join("gwas-native-cases");
    let absent = !native_directory.is_dir();
    vec![
        Trial::test(
            "gwas-search/retained-native-witnesses-and-costs",
            move || native::run(&native_directory).map_err(Into::into),
        )
        .with_ignored_flag(absent),
        Trial::test("gwas-search/restricted-policy-before-io", || {
            policy().map_err(Into::into)
        }),
        Trial::test("gwas-search/checked-native-selectors-and-aliases", || {
            wire::selectors().map_err(Into::into)
        }),
        Trial::test("gwas-search/native-offset-and-continuation", || {
            wire::paging().map_err(Into::into)
        }),
        Trial::test("gwas-search/live-warm-cache-and-explicit-fresh", || {
            wire::warm().map_err(Into::into)
        }),
        Trial::test("gwas-search/reject-before-cache-admission", || {
            mechanics::admission().map_err(Into::into)
        }),
        Trial::test("gwas-search/failures-are-never-empty", || {
            mechanics::failures().map_err(Into::into)
        }),
        Trial::test("gwas-search/cli-mcp-help-markdown", || {
            wire::surfaces().map_err(Into::into)
        }),
    ]
}
fn policy() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("empty-replay");
    fs::create_dir(&replay).map_err(|e| e.to_string())?;
    let mut args = env.globals(Some(&replay), false);
    args.extend(
        [
            "search",
            "gwas",
            "--pubmed",
            "17529967",
            "--disable-sources",
            "restricted",
        ]
        .map(str::to_owned),
    );
    let value = invoke(&env, args, 4)?;
    if value.pointer("/sources/0/reason") != Some(&json!("disabled")) {
        return Err(format!("restricted source was not refused: {value}"));
    }
    Ok(())
}

const BASE: &str = "https://www.ebi.ac.uk/gwas/rest/api/v2/associations";
fn url(pairs: &[(&str, &str)]) -> String {
    let params: std::collections::BTreeMap<_, _> = pairs.iter().copied().collect();
    let encoded = params
        .into_iter()
        .map(|(k, v)| format!("{k}={}", v.replace('%', "%25").replace(' ', "%20")))
        .collect::<Vec<_>>()
        .join("&");
    format!("{BASE}?{encoded}")
}
fn body(url: &str, page: u32, total: u32) -> Value {
    let rows = (page * 3..(page * 3 + 3).min(total)).map(|n| json!({
        "association_id":6110+n,"accession_id":"GCST000035","reported_trait":["Synthetic reported trait"],
        "efo_traits":[{"efo_id":"MONDO_0004989","efo_trait":"breast carcinoma"}],
        "snp_allele":[{"rs_id":"rs2981582","effect_allele":"A"}],
        "p_value":4e-254,"pvalue_mantissa":4,"pvalue_exponent":-254,
        "or_per_copy_num":1.26,"pubmed_id":"17529967","risk_frequency":"NR"
    })).collect::<Vec<_>>();
    json!({"_embedded":{"associations":rows},"_links":{"self":{"href":url}},
        "page":{"size":3,"number":page,"totalElements":total,"totalPages":total.div_ceil(3)}})
}
fn seed(
    env: &Environment,
    args: &[&str],
    body: Value,
    status: u16,
    delay: u64,
) -> Result<PathBuf, String> {
    seed_text(env, args, body.to_string(), status, delay)
}
fn seed_text(
    env: &Environment,
    args: &[&str],
    body: String,
    status: u16,
    delay: u64,
) -> Result<PathBuf, String> {
    let stub = super::network_stub::Stub::for_source(
        "www.ebi.ac.uk", "application/json", wire::live_body,
    )?;
    let recording_env = super::variant::source_environment(&stub)?;
    let temporary = recording_env.root().join("recorded");
    let mut command = args.iter().map(|s| (*s).to_owned()).collect::<Vec<_>>();
    command.extend(["--record".into(), temporary.to_string_lossy().into()]);
    invoke(&recording_env, command, 0)?;
    if stub.finish()?.len() != 1 {
        return Err("synthetic recording made an unexpected request count".into());
    }
    let directory = env.root().join("replay/gwas");
    fs::create_dir_all(&directory).map_err(|e| e.to_string())?;
    for file in fs::read_dir(temporary.join("gwas")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let mut recording: Value = serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let exchange = json!({"status":status,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text","value":body}});
        *recording.get_mut("exchanges").ok_or("recorded exchanges")? = json!([exchange.clone(),exchange.clone(),exchange]);
        fs::write(directory.join(file.file_name()), recording.to_string()).map_err(|e| e.to_string())?;
    }
    Ok(env.root().join("replay"))
}
fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).to_owned()));
    invoke(env, command, exit)
}
fn pairs() -> Vec<(&'static str, &'static str)> {
    vec![
        ("pubmed_id", "17529967"),
        ("direction", "asc"),
        ("page", "0"),
        ("size", "3"),
        ("sort", "p_value"),
    ]
}
