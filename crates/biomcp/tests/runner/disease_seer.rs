//! Feature disease-seer: native site choices and independent annual statistics.
use super::{launch::Environment, network_stub::Stub};
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};

tool_only! { mod native; }
tool_only! { pub(super) use native::run as native; }

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
    let value = super::run_case(file.path(), super::schema::Registry::embedded()?.as_ref())?;
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
