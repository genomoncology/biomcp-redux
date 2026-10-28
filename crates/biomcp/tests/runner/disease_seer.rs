//! Feature disease-seer: native site choices and independent annual statistics.
use super::{launch::Environment, network_stub::Stub};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
mod failures;
mod native;
pub(super) use native::run as native;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial(
            "disease-seer/stale-valid-rows-retain-rejected-reply-problem",
            || failures::stale_rejection().map_err(Into::into),
        ),
        transcript_trial(
            "disease-seer/wrong-site-drops-before-cache-and-corrected-reply-recovers",
            || failures::wrong_site().map_err(Into::into),
        ),
        transcript_trial(
            "disease-seer/catalog-keeps-native-sites-and-local-pages",
            || catalog().map_err(Into::into),
        ),
        transcript_trial("disease-seer/incidence-preserves-stage-rows", || {
            incidence().map_err(Into::into)
        }),
        transcript_trial(
            "disease-seer/survival-keeps-observed-and-model-only-years",
            || survival().map_err(Into::into),
        ),
        transcript_trial("disease-seer/selectors-refuse-before-io", || {
            failures::selectors().map_err(Into::into)
        }),
        transcript_trial("disease-seer/failures-retain-source-and-terms", || {
            failures::run().map_err(Into::into)
        }),
        transcript_trial(
            "disease-seer/disabled-source-bypasses-retained-data",
            || failures::disabled().map_err(Into::into),
        ),
    ]
}
fn setup(stub: &Stub) -> Result<Environment, String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let ca = env.root().join("seer-ca.pem");
    stub.retain(&env);
    fs::write(&ca, &stub.certificate).map_err(|e| e.to_string())?;
    env.config(&json!({"proxy":format!("http://{}",stub.address),"ca_bundle":ca}))
        .map_err(|e| e.to_string())?;
    Ok(env)
}
fn args(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| (*s).into()).collect()
}
fn invoke(env: &Environment, words: &[String], exit: i32) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), words)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) {
        return Err(format!(
            "wrong exit {:?}: {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    measure(
        &words.iter().map(String::as_str).collect::<Vec<_>>(),
        &value,
    )?;
    Ok(value)
}
fn record(env: &Environment, words: &[&str], directory: &Path, exit: i32) -> Result<Value, String> {
    let mut command = args(words);
    command.extend(["--record".into(), directory.to_string_lossy().into()]);
    let result = invoke(env, &command, exit)?;
    Ok(result)
}
fn measure(words: &[&str], result: &Value) -> Result<(), String> {
    let full = serde_json::to_vec(result).map_err(|e| e.to_string())?.len();
    let comparable = super::source_terms::size(result, "")?;
    writeln!(
        std::io::stderr(),
        "SEER {:?}: full={full}, comparable={comparable}, estimated tokens={}",
        words,
        comparable.div_ceil(4)
    )
    .map_err(|e| e.to_string())
}
fn replay(directory: &Path, steps: Value) -> Result<Value, String> {
    let file = tempfile::NamedTempFile::new().map_err(|e| e.to_string())?;
    fs::write(
        file.path(),
        serde_json::to_vec(&json!(
            {"feature":"disease-seer","story":"Native site and statistic selectors retain source values","replay":directory,"steps":steps}
        ))
        .map_err(|e| e.to_string())?,
    )
    .map_err(|e| e.to_string())?;
    let value = super::run_case(file.path(), &super::schema::Registry::embedded()?)?;
    measure(&["replay-terminal"], &value)?;
    Ok(value)
}
fn wire(request: &str) -> Result<String, String> {
    let name = if request.contains("get_var_formats.php") {
        "223a28d053d7e4bbf1a87ce337a5b800d2839356b691fa24ee0df85d7d52be5c"
    } else if request.contains("data_type=1") {
        "9843491731443048c8fa39a04fa72a5c4cb177ae107e863633c204630e3a53e1"
    } else {
        "6034e658f1d299da7d80b7b51bc8f9751bd9ce674eaa5b4f328a617e4c81c671"
    };
    let path =
        super::launch::fixtures().join(format!("replies/disease/seer-synthetic/seer/{name}.json"));
    let value: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?;
    serde_json::to_string(
        value
            .pointer("/exchanges/0/body/value")
            .ok_or("missing synthetic wire body")?,
    )
    .map_err(|e| e.to_string())
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
    if !output.status.success() || required.iter().any(|r| !text.contains(r)) {
        return Err(format!("Markdown lost SEER facts: {text}"));
    }
    Ok(())
}
fn catalog() -> Result<(), String> {
    let stub = Stub::for_source("seer.cancer.gov", "application/json", wire)?;
    let env = setup(&stub)?;
    let directory = env.root().join("catalog");
    let words = ["search", "disease", "--source", "seer", "--limit", "1"];
    record(&env, &words, &directory, 0)?;
    replay(
        &directory,
        json!(
            [ {"run":words,"mcp":words,"expect":{"/status":"partial","/page/total":2,"/result/items/0":{"equals":{"code":"55",
            "label":"Example site","active":true}},"/problems/0/field":"catalog.publisher_date","/problems/1/field":"catalog.publisher_version",
            "/next/0/kind":"page"},"max_bytes":644},{"run":["search","disease","--source","seer","--limit","1","--offset","1"],"mcp":["search",
            "disease","--source","seer","--limit","1","--offset","1"],"offline":true,"expect":{"/page/offset":1,"/result/items/0/code":"47",
            "/result/items/0/active":false},"max_bytes":532} ]
        ),
    )?;
    markdown(
        &env,
        &words,
        &directory,
        &["Example site", "publisher", "SEER/NCI"],
    )?;
    if stub.finish()?.len() != 1 {
        return Err("catalog page fetched again".into());
    }
    Ok(())
}
fn incidence() -> Result<(), String> {
    let stub = Stub::for_source("seer.cancer.gov", "application/json", wire)?;
    let env = setup(&stub)?;
    let directory = env.root().join("incidence");
    let words = [
        "get",
        "disease",
        "47",
        "incidence",
        "--source",
        "seer",
        "--dataset",
        "4",
        "--period",
        "5",
        "--section-limit",
        "1",
    ];
    record(&env, &words, &directory, 0)?;
    replay(
        &directory,
        json!(
            [ {"run":words,"mcp":words,"expect":{"/query/sex":"1","/query/race":"1","/query/age_range":"1","/page/total":2,
            "/result/items/0/measure":"rate","/result/items/0/observed/count":100,"/result/items/0/returned_selectors/3/code":"104",
            "/next/0/kind":"page"},"max_bytes":1889},{"run":["get","disease","47","incidence","--source","seer","--dataset","4","--period","5",
            "--section-offset","2"],"offline":true,"expect":{"/page/returned":0,"/page/total":2,"/next/0/args":["search","disease","--source",
            "seer"]},"max_bytes":862} ]
        ),
    )?;
    markdown(
        &env,
        &words,
        &directory,
        &["Observed", "100", "registry", "SEER/NCI"],
    )?;
    let requests = stub.finish()?;
    if requests.len() != 1
        || !requests
            .first()
            .is_some_and(|(_, r)| r.contains("sex=1&race=1&age_range=1&year_range=5&data_model=4"))
    {
        return Err(format!("incidence selector drift: {requests:?}"));
    }
    Ok(())
}
fn survival() -> Result<(), String> {
    let stub = Stub::for_source("seer.cancer.gov", "application/json", wire)?;
    let env = setup(&stub)?;
    let directory = env.root().join("survival");
    let words = [
        "get",
        "disease",
        "55",
        "survival",
        "--source",
        "seer",
        "--sex",
        "3",
        "--race",
        "1",
        "--age-range",
        "1",
        "--dataset",
        "4",
        "--interval",
        "5",
        "--section-limit",
        "1",
    ];
    record(&env, &words, &directory, 0)?;
    let mut next = args(&words);
    next.extend(["--section-offset".into(), "1".into()]);
    let last = replay(
        &directory,
        json!(
            [ {"run":words,"mcp":words,"expect":{"/result/items/0/year":2018,"/result/items/0/observed/value":90.0,
            "/result/items/0/modeled/value":90.5,"/result/items/0/returned_selectors/5/code":"31"},"max_bytes":2024},{"run":next,"mcp":next,
            "offline":true,"expect":{"/result/items/0/year":2022,"/result/items/0/modeled/value":93.5,"/page/offset":1,"/page/total":2},
            "max_bytes":1694} ]
        ),
    )?;
    if last.pointer("/result/items/0/observed").is_some() {
        return Err("model-only row acquired an observation".into());
    }
    markdown(
        &env,
        &words,
        &directory,
        &["Observed", "Modeled", "2018", "90&#46;5"],
    )?;
    let requests = stub.finish()?;
    if requests.len() != 1
        || !requests.first().is_some_and(|(_, r)| {
            r.contains("relative_survival_interval=5&year_range=31&data_model=4")
        })
    {
        return Err(format!("survival selector drift: {requests:?}"));
    }
    Ok(())
}
