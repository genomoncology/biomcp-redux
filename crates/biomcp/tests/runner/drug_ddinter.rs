//! Feature drug-ddinter: reads never fetch, and failed parts preserve the complete set.
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("drug-ddinter/default-five-and-explicit-ten", || {
            default_pages().map_err(Into::into)
        }),
        transcript_trial(
            "drug-ddinter/failed-parts-preserve-stored-set-and-policy",
            || check().map_err(Into::into),
        ),
        transcript_trial(
            "drug-ddinter/missing-local-set-offers-explicit-sync",
            || missing().map_err(Into::into),
        ),
    ]
}
fn missing() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = env.root().join("empty-replay");
    fs::create_dir(&replay).map_err(|e| e.to_string())?;
    for args in [
        vec!["search", "drug", "Synthetic", "--source", "ddinter"],
        vec![
            "get",
            "drug",
            "DDInter1",
            "interactions",
            "--source",
            "ddinter",
        ],
    ] {
        let value = call(&env, &replay, &args, 4)?;
        if value.pointer("/sources/0/state") != Some(&json!("syncing"))
            || value.pointer("/next/0/args") != Some(&json!(["sync", "ddinter"]))
        {
            return Err(format!(
                "read fetched or hid the missing local set: {value}"
            ));
        }
    }
    Ok(())
}

fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    invoke(env, command, exit)
}
fn seed(env: &Environment) -> Result<PathBuf, String> {
    let source = fixtures().join("replies/drug/ddinter-synthetic/ddinter");
    let replay = env.root().join("replay");
    fs::create_dir_all(replay.join("ddinter")).map_err(|e| e.to_string())?;
    for entry in fs::read_dir(source).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        fs::copy(entry.path(), replay.join("ddinter").join(entry.file_name()))
            .map_err(|e| e.to_string())?;
    }
    Ok(replay)
}
fn check() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env)?;
    call(&env, &replay, &["sync", "ddinter"], 0)?;
    let args = [
        "get",
        "drug",
        "DDInter3",
        "interactions",
        "--source",
        "ddinter",
    ];
    let before = call(&env, &replay, &args, 0)?;
    let paths: Vec<_> = fs::read_dir(replay.join("ddinter"))
        .map_err(|e| e.to_string())?
        .map(|r| r.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<_, _>>()?;
    let (path, original) = paths
        .iter()
        .filter_map(|path| fs::read(path).ok().map(|bytes| (path, bytes)))
        .find(|(_, bytes)| String::from_utf8_lossy(bytes).contains("code_L.csv"))
        .ok_or("missing L partition")?;
    corrupted(&env, &replay, &args, &before, path, &original)?;
    fs::remove_file(path).map_err(|e| e.to_string())?;
    call(&env, &replay, &["sync", "ddinter"], 5)?;
    if call(&env, &replay, &args, 0)? != before {
        return Err("missing partition changed the complete set".into());
    }
    // No recording remains. All subsequent successful commands must be stored-only.
    for path in paths {
        if path.exists() {
            fs::remove_file(path).map_err(|e| e.to_string())?;
        }
    }
    let mut fresh = args.to_vec();
    fresh.push("--fresh");
    if call(&env, &replay, &fresh, 0)? != before {
        return Err("fresh read changed or fetched stored rows".into());
    }
    env.config(&json!({"disable_sources":["noncommercial"]}))
        .map_err(|e| e.to_string())?;
    for query in [
        &args[..],
        &["search", "drug", "Synthetic", "--source", "ddinter"][..],
        &["sync", "ddinter"][..],
    ] {
        let disabled = call(&env, &replay, query, 4)?;
        if disabled.pointer("/sources/0/reason") != Some(&json!("disabled")) {
            return Err(format!("policy admitted stored or replay data: {disabled}"));
        }
    }
    Ok(())
}

fn corrupted(
    env: &Environment,
    replay: &Path,
    args: &[&str],
    before: &Value,
    path: &Path,
    original: &[u8],
) -> Check {
    let header = "DDInterID_A,Drug_A,DDInterID_B,Drug_B,Level\n";
    for row in [
        "wrong,Synthetic A,DDInter1,Synthetic B,Major\n",
        "DDInter1,Synthetic A,DDInter,Synthetic B,Major\n",
        "DDInter1,,DDInter2,Synthetic B,Major\n",
        "DDInter1,Synthetic A,DDInter2, ,Major\n",
        "DDInter1,Synthetic A,DDInter2,Synthetic B,\n",
        "+++ ,+++,+++,+++,+++\n",
        "+++,+++,+++,+++,+++\n",
        "DDInter1,Synthetic A,DDInter2,\"unterminated\n",
        "DDInter1,Synthetic A,DDInter2,Synthetic B\n",
        "",
    ] {
        let mut bad: Value = serde_json::from_slice(original).map_err(|e| e.to_string())?;
        *bad.pointer_mut("/exchanges/0/body/value")
            .ok_or("missing synthetic body")? = format!("{header}{row}").into();
        fs::write(path, bad.to_string()).map_err(|e| e.to_string())?;
        if row.starts_with("DDInter1,,")
            || row.contains("DDInter2, ,")
            || row.ends_with("Synthetic B,\n")
        {
            optional_cells(env, replay, args, before, path, original)?;
            continue;
        }
        let failed = call(env, replay, &["sync", "ddinter"], 4)?;
        if failed.pointer("/sources/0/reason") != Some(&json!("bad-reply"))
            || call(env, replay, args, 0)? != *before
        {
            return Err(format!(
                "malformed partition changed the complete set: {failed}"
            ));
        }
    }
    Ok(())
}

fn optional_cells(
    env: &Environment,
    replay: &Path,
    args: &[&str],
    before: &Value,
    path: &Path,
    original: &[u8],
) -> Check {
    let synchronized = call(env, replay, &["sync", "ddinter"], 0)?;
    if synchronized.pointer("/result/rows") != Some(&json!(8)) {
        return Err(format!(
            "optional DDInter cells rejected a native pair: {synchronized}"
        ));
    }
    let retained = call(env, replay, args, 0)?;
    for field in ["anchor", "partner", "level"] {
        let pointer = format!("/result/interactions/data/records/0/{field}");
        if retained.pointer(&pointer).is_none()
            || retained.pointer(&pointer) != before.pointer(&pointer)
        {
            return Err(format!(
                "optional DDInter cells changed a valid sibling {field}"
            ));
        }
    }
    fs::write(path, original).map_err(|error| error.to_string())?;
    call(env, replay, &["sync", "ddinter"], 0)?;
    if call(env, replay, args, 0)? != *before {
        return Err("restored DDInter partition changed the complete set".into());
    }
    Ok(())
}

fn seed_many(env: &Environment) -> Result<PathBuf, String> {
    let replay = seed(env)?;
    for entry in fs::read_dir(replay.join("ddinter")).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let mut capture: Value =
            serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        if capture
            .get("url")
            .and_then(Value::as_str)
            .is_some_and(|url| url.ends_with("code_A.csv"))
        {
            let body = capture
                .pointer_mut("/exchanges/0/body/value")
                .ok_or("body")?;
            let mut text = body.as_str().ok_or("text body")?.to_owned();
            for partner in 100..108 {
                text.push_str(&format!("DDInter1,Synthetic Anchor One,DDInter{partner},Synthetic Partner {partner},Major\r\n"));
            }
            *body = text.into();
            fs::write(path, capture.to_string()).map_err(|e| e.to_string())?;
        }
    }
    call(env, &replay, &["sync", "ddinter"], 0)?;
    Ok(replay)
}
fn interaction_page(env: &Environment, replay: &Path, args: &[String]) -> Result<Value, String> {
    let globals = env.globals(Some(replay), false);
    let mut cli = globals.clone();
    cli.extend_from_slice(args);
    let value = invoke(env, cli, 0)?;
    if super::mcp::call_with_globals(env, args, false, &globals)? != value {
        return Err("DDInter default page differs over MCP".into());
    }
    super::schema::Registry::embedded()?.validate(&value, args, true)?;
    use std::io::Write;
    writeln!(
        std::io::stdout(),
        "DDINTER_PAGE args={args:?} full={} comparable={}",
        value.to_string().len(),
        super::source_terms::size(&value, "")?
    )
    .map_err(|e| e.to_string())?;
    let offset = args
        .windows(2)
        .find(|pair| pair.first().is_some_and(|flag| flag == "--section-offset"))
        .and_then(|pair| pair.get(1))
        .map_or("0", String::as_str);
    let (measured_full, measured_comparable): (usize, usize) =
        if args.iter().any(|arg| arg == "--section-limit") {
            (5546, 5443)
        } else {
            match offset {
                "0" => (3133, 3030),
                "5" => (3152, 3049),
                "10" => (1141, 1038),
                _ => return Err("unexpected default offset".into()),
            }
        };
    if value.to_string().len() > (measured_full * 115).div_ceil(100)
        || super::source_terms::size(&value, "")? > (measured_comparable * 115).div_ceil(100)
    {
        return Err("DDInter page exceeds measured full/comparable ceilings".into());
    }
    Ok(value)
}
fn default_pages() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed_many(&env)?;
    let base: Vec<String> = [
        "get",
        "drug",
        "DDInter1",
        "interactions",
        "--source",
        "ddinter",
    ]
    .map(str::to_owned)
    .into();
    let mut args = base.clone();
    let mut records = Vec::new();
    for (offset, count) in [(0, 5), (5, 5), (10, 1)] {
        let value = interaction_page(&env, &replay, &args)?;
        if value.get("page") != Some(&json!({"offset":offset,"returned":count,"total":11})) {
            return Err(format!(
                "DDInter omitted limit lost its five-row page: {value}"
            ));
        }
        records.extend(
            value
                .pointer("/result/interactions/data/records")
                .and_then(Value::as_array)
                .ok_or("records")?
                .iter()
                .cloned(),
        );
        let next = value
            .get("next")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .find(|next| next.get("kind") == Some(&json!("page")));
        if offset == 10 {
            if next.is_some() {
                return Err("DDInter walk continued past total".into());
            }
        } else {
            args = serde_json::from_value(
                next.ok_or("missing next page")?
                    .get("args")
                    .ok_or("next args")?
                    .clone(),
            )
            .map_err(|e| e.to_string())?;
            if args.iter().any(|arg| arg == "--section-limit") {
                return Err("DDInter omitted limit became an explicit override".into());
            }
        }
    }
    explicit_ten(&env, &replay, base, &records)
}
fn explicit_ten(env: &Environment, replay: &Path, base: Vec<String>, records: &[Value]) -> Check {
    let mut explicit = base;
    explicit.extend(["--section-limit".into(), "10".into()]);
    let page = interaction_page(env, replay, &explicit)?;
    if page.get("page") != Some(&json!({"offset":0,"returned":10,"total":11}))
        || page.pointer("/result/interactions/data/records")
            != Some(&json!(records.get(..10).ok_or("ten default records")?))
    {
        return Err("DDInter explicit ten changed rows or order".into());
    }
    let next: Vec<String> = serde_json::from_value(
        page.pointer("/next/0/args")
            .ok_or("explicit next page")?
            .clone(),
    )
    .map_err(|e| e.to_string())?;
    if !next
        .windows(2)
        .any(|pair| pair == ["--section-limit", "10"])
        || !next
            .windows(2)
            .any(|pair| pair == ["--section-offset", "10"])
    {
        return Err("DDInter explicit continuation lost its limit or offset".into());
    }
    Ok(())
}
