//! Native bodies stay in the external evidence store and replay through transport recordings.
use super::*;
use std::{collections::BTreeMap, io::Write};
pub(super) fn run() -> Result<(), String> {
    let root = super::super::launch::fixtures().join("gwas-get-native");
    let inputs: Vec<Value> =
        serde_json::from_slice(&fs::read(root.join("input.json")).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let mut replies = BTreeMap::new();
    for input in &inputs {
        let url = input["url"].as_str().ok_or("native URL")?;
        replies.insert(
            target(url),
            input["body"].as_str().ok_or("native body")?.to_owned(),
        );
    }
    let stub = Stub::for_sources(
        &["www.ebi.ac.uk"],
        |_| "application/json",
        move |request| {
            let path = request
                .lines()
                .next()
                .and_then(|line| line.split_whitespace().nth(1))
                .ok_or("request target")?;
            replies
                .get(&target(&format!("https://www.ebi.ac.uk{path}")))
                .cloned()
                .ok_or_else(|| format!("unexpected source request {path}"))
        },
    )?;
    let env = super::super::variant::source_environment(&stub)?;
    let recordings = env.root().join("recordings");
    let output = root.join("outputs");
    fs::create_dir_all(&output).map_err(|e| e.to_string())?;
    for input in &inputs {
        record(&env, &recordings, input)?;
    }
    if stub.finish()?.len() != inputs.len() {
        return Err("native seed request count differs".into());
    }
    for input in &inputs {
        measure(&root, &recordings, &output, input)?;
    }
    walks(&root, &recordings)?;

    Ok(())
}
fn words(value: &Value) -> Result<Vec<String>, String> {
    value
        .as_array()
        .ok_or("command arguments")?
        .iter()
        .map(|v| {
            v.as_str()
                .map(str::to_owned)
                .ok_or_else(|| "argument".into())
        })
        .collect()
}
fn target(url: &str) -> String {
    let Some((path, query)) = url.split_once('?') else {
        return url.to_owned();
    };
    let mut params = query.split('&').collect::<Vec<_>>();
    params.sort_unstable();
    format!("{path}?{}", params.join("&"))
}

fn measure(root: &Path, recordings: &Path, output: &Path, input: &Value) -> Result<(), String> {
    let args = words(&input["args"])?;
    let name = input["name"].as_str().ok_or("case name")?;
    let path = root.join(format!("{name}.yaml"));
    let mut step =
        json!({"run":args,"mcp":args,"expect":input["expect"],"max_bytes":20000,"max_tokens":5000});
    let budgets: Value = serde_json::from_str(&crate::runner::launch::fixture_text(
        "gwas-get-budgets.json",
    )?)
    .map_err(|e| e.to_string())?;
    if let Some(comparable) = budgets
        .get(name)
        .and_then(|v| v.get(1))
        .and_then(Value::as_u64)
    {
        *step.get_mut("max_bytes").ok_or("byte ceiling")? =
            json!(((comparable + 1) * 115).div_ceil(100));
        *step.get_mut("max_tokens").ok_or("token ceiling")? =
            json!(((comparable + 1) * 115).div_ceil(400));
    }
    fs::write(
        &path,
        json!({"feature":"gwas-get","story":name,"replay":recordings,"steps":[step]}).to_string(),
    )
    .map_err(|e| e.to_string())?;
    let value = super::super::run_case(&path, &super::super::schema::Registry::embedded()?)?;
    let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() + 1;
    let comparable = super::super::source_terms::size(&value, "")?;
    if let Some(measured) = budgets
        .get(name)
        .and_then(|v| v.get(0))
        .and_then(Value::as_u64)
        && full as u64 > (measured * 115).div_ceil(100)
    {
        return Err(format!("{name} full ceiling"));
    }
    fs::write(output.join(format!("{name}.json")), value.to_string()).map_err(|e| e.to_string())?;
    writeln!(
        std::io::stdout(),
        "GWAS get {name}: CLI={full}, MCP={}, comparable={comparable}, tokens={}",
        full - 1,
        comparable.div_ceil(4)
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn walks(root: &Path, recordings: &Path) -> Result<(), String> {
    for (name, args, count, identity) in [
        (
            "study-walk",
            vec!["get", "gwas", "GCST000035", "associations"],
            6,
            "/result/items/*/id",
        ),
        (
            "ancestry-walk",
            vec![
                "get",
                "gwas",
                "GCST000035",
                "ancestries",
                "--section-limit",
                "1",
            ],
            3,
            "/result/items/*/source_url",
        ),
    ] {
        let path = root.join(format!("{name}.yaml"));
        let mut expected = serde_json::Map::new();
        expected.insert("/result/items".into(), json!({"count":count}));
        expected.insert(identity.into(), json!({"unique":true}));
        fs::write(
            &path,
            json!({"feature":"gwas-get","story":name,"replay":recordings,"steps":[{
            "run":args,"mcp":args,"walk":{"items":"/result/items","max_pages":3},"expect":expected}]})
            .to_string(),
        )
        .map_err(|e| e.to_string())?;
        super::super::run_case(&path, &super::super::schema::Registry::embedded()?)?;
    }
    Ok(())
}

fn record(env: &Environment, recordings: &Path, input: &Value) -> Result<(), String> {
    let args = words(&input["args"])?;
    let name = input["name"].as_str().ok_or("case name")?;
    let original = env.root().join(format!("record-{name}"));
    let mut command = args.clone();
    command.extend([
        "--fresh".into(),
        "--record".into(),
        original.to_string_lossy().into(),
    ]);
    invoke(env, command, 0).map_err(|error| format!("native seed {args:?}: {error}"))?;
    fs::create_dir_all(recordings.join("gwas")).map_err(|e| e.to_string())?;
    for file in fs::read_dir(original.join("gwas")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        fs::copy(file.path(), recordings.join("gwas").join(file.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(())
}
