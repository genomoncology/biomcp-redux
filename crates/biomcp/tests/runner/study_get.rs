//! Feature study-get: exact stored cards and bounded source metadata without member IDs.
use super::{rows, seed};
use crate::runner::launch::Environment;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{fs, io::Write, path::Path};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    let native = crate::runner::launch::fixtures().join("study-get-cases");
    let available = native.is_dir();
    vec![
        transcript_trial("study-get/retained-native", move || {
            retained(&native).map_err(Into::into)
        })
        .with_ignored_flag(!available),
        transcript_trial("study-get/complete-pages-and-native-counts", || {
            pages().map_err(Into::into)
        }),
        transcript_trial("study-get/identity-failures-and-policy", || {
            failures().map_err(Into::into)
        }),
        transcript_trial("study-get/admission-and-absent-count", || {
            admission().map_err(Into::into)
        }),
    ]
}
fn profiles() -> Value {
    json!(
        (0..17)
            .map(|n| json!({"molecularProfileId":format!("native_profile_{n}"),"studyId":"synthetic_0",
        "molecularAlterationType":"MRNA_EXPRESSION","datatype":"CONTINUOUS","name":"Synthetic profile",
        "description":"Native profile description","patientLevel":false,"showProfileInAnalysisTab":true}))
            .collect::<Vec<_>>()
    )
}
fn lists() -> Value {
    json!(
        (0..13)
            .map(
                |n| json!({"sampleListId":format!("native_list_{n}"),"studyId":"synthetic_0","category":"all_cases_in_study",
        "name":"Synthetic list","description":"Native list description","sampleCount":n})
            )
            .collect::<Vec<_>>()
    )
}
fn record(replay: &Path, section: &str, body: Value, status: u16) -> Check {
    recording(replay, section, body, status, 0)
}
fn recording(replay: &Path, section: &str, body: Value, status: u16, delay: u64) -> Check {
    recording_raw(replay, section, &body.to_string(), status, delay)
}
pub(super) fn recording_raw(
    replay: &Path,
    section: &str,
    body: &str,
    status: u16,
    delay: u64,
) -> Check {
    let stub = crate::runner::network_stub::Stub::for_source(
        "www.cbioportal.org",
        "application/json",
        synthetic_reply,
    )?;
    let env = crate::runner::variant::source_environment(&stub)?;
    crate::runner::configuration::invoke(
        &env,
        vec!["sync".into(), "cbioportal-studies".into()],
        0,
    )?;
    let directory = env.root().join("recorded");
    let selected = if section == "molecular-profiles" {
        "profiles"
    } else {
        section
    };
    let acquired = crate::runner::configuration::invoke(
        &env,
        [
            "get",
            "study",
            "synthetic_0",
            selected,
            "--record",
            directory.to_str().ok_or("recording path")?,
        ]
        .map(str::to_owned)
        .to_vec(),
        0,
    )?;
    if stub.finish()?.len() != 2 {
        return Err("synthetic study recording changed request count".into());
    }
    for file in fs::read_dir(directory.join("cbioportal")).map_err(|e| e.to_string())? {
        let file = file.map_err(|e| e.to_string())?;
        let mut value: Value =
            serde_json::from_slice(&fs::read(file.path()).map_err(|e| e.to_string())?)
                .map_err(|e| e.to_string())?;
        if value.pointer("/exchanges/0/fetched") != acquired.pointer("/result/acquired")
            || acquired.pointer("/result/acquired").and_then(Value::as_u64).is_none()
        {
            return Err("live study acquisition differs from its recording receipt".into());
        }
        let exchange = json!(
            {"status":status,"fetched":42,"content_type":"application/json","delay_ms":delay,"retry_after_ms":null,"body":{"kind":"text",
            "value":body}}
        );
        let exchanges = if matches!(status, 429 | 500) {
            vec![exchange; 3]
        } else {
            vec![exchange]
        };
        *value.get_mut("exchanges").ok_or("recorded exchanges")? = json!(exchanges);
        fs::write(
            replay.join("cbioportal").join(file.file_name()),
            value.to_string(),
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}
fn synthetic_reply(request: &str) -> Result<String, String> {
    Ok(if request.contains("/molecular-profiles?") {
        profiles()
    } else if request.contains("/sample-lists?") {
        lists()
    } else {
        rows()
    }
    .to_string())
}

fn pages() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    call(
        &env,
        &replay,
        &["sync", "cbioportal-studies"],
        0,
        "get/pages/1",
    )?;
    record(&replay, "molecular-profiles", profiles(), 200)?;
    record(&replay, "sample-lists", lists(), 200)?;
    for (section, total, id) in [
        ("profiles", 17, "molecularProfileId"),
        ("sample-lists", 13, "sampleListId"),
    ] {
        let mut count = 0;
        let mut seen = std::collections::BTreeSet::new();
        loop {
            let offset = count.to_string();
            let value = call(
                &env,
                &replay,
                &[
                    "get",
                    "study",
                    "synthetic_0",
                    section,
                    "--section-offset",
                    &offset,
                ],
                0,
                "get/pages/2",
            )?;
            let items = value
                .pointer("/result/items")
                .and_then(Value::as_array)
                .ok_or_else(|| format!("missing section items: {value}"))?;
            if value.pointer("/page/total") != Some(&json!(total)) || items.len() > 5
                || value.pointer("/result/acquired") != Some(&json!(42))
            {
                return Err(format!("native page bounds: {value}"));
            }
            for row in items {
                let identity = row.get(id).and_then(Value::as_str).ok_or("native ID")?;
                if !seen.insert(identity.to_owned())
                    || row.get("studyId") != Some(&json!("synthetic_0"))
                    || row.get("sampleIds").is_some()
                {
                    return Err("collection changed identity or exposed members".into());
                }
            }
            count += items.len();
            if count == total {
                break;
            }
            if items.is_empty() {
                return Err("page walk stopped early".into());
            }
        }
    }
    super::study_files::seed_files(&replay)?;
    let all = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0", "all"],
        0,
        "get/pages/3",
    )?;
    bounded_all(&all)?;
    navigation(&env, &replay)?;
    Ok(())
}
fn bounded_all(all: &Value) -> Check {
    if all
        .pointer("/result/profiles/data/items")
        .and_then(Value::as_array)
        .map(Vec::len)
        != Some(5)
        || all
            .pointer("/result/sample_lists/data/items")
            .and_then(Value::as_array)
            .map(Vec::len)
            != Some(5)
    {
        return Err(format!("all lost independent bounded sections: {all}"));
    }
    Ok(())
}
fn failures() -> Check {
    for section in ["profiles", "sample-lists"] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let replay = seed(&env, rows(), 200)?;
        call(
            &env,
            &replay,
            &["sync", "cbioportal-studies"],
            0,
            "get/failures/1",
        )?;
        let route = if section == "profiles" {
            "molecular-profiles"
        } else {
            section
        };
        let mut body = if section == "profiles" {
            profiles()
        } else {
            lists()
        };
        *body.pointer_mut("/0/studyId").ok_or("scope")? = json!("other_study");
        record(&replay, route, body, 200)?;
        let value = call(
            &env,
            &replay,
            &["get", "study", "synthetic_0", section],
            0,
            "get/failures/2",
        )?;
        rejected(&env, &replay, section, &value)?;
        if value.get("status") != Some(&json!("partial"))
            || value
                .get("problems")
                .and_then(Value::as_array)
                .is_none_or(Vec::is_empty)
        {
            return Err(format!("wrong scope not disclosed: {value}"));
        }
        for disabled in ["cbioportal", "restricted"] {
            let value = call(
                &env,
                &replay,
                &[
                    "get",
                    "study",
                    "synthetic_0",
                    section,
                    "--disable-sources",
                    disabled,
                ],
                4,
                "get/failures/3",
            )?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            {
                return Err(format!("policy bypass: {value}"));
            }
        }
    }
    Ok(())
}

fn admission() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = seed(&env, rows(), 200)?;
    let mut body = lists();
    let row = body
        .get_mut(0)
        .and_then(Value::as_object_mut)
        .ok_or("row")?;
    row.remove("sampleCount");
    row.insert("sampleIds".into(), json!([null,{"ignored":true}]));
    record(&replay, "sample-lists", body, 200)?;
    let value = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0", "sample-lists"],
        0,
        "get/admission/1",
    )?;
    if value.pointer("/result/items/0/sampleCount").is_some()
        || value.to_string().contains("sampleIds")
    {
        return Err(format!(
            "missing count inferred or members retained: {value}"
        ));
    }
    let mut body = lists();
    *body.pointer_mut("/0/description").ok_or("description")? = json!("x".repeat(1_526_432));
    record(&replay, "sample-lists", body, 200)?;
    let value = call(
        &env,
        &replay,
        &["get", "study", "synthetic_0", "sample-lists", "--fresh"],
        4,
        "get/admission/2",
    )?;
    if value.get("result").is_some()
        || value.pointer("/problems/0/code") != Some(&json!("collection.incomplete"))
    {
        return Err(format!("over-bound response returned a cut page: {value}"));
    }
    Ok(())
}

fn retained(directory: &Path) -> Check {
    let registry = crate::runner::schema::Registry::embedded()?;
    let mut paths = fs::read_dir(directory)
        .map_err(|e| e.to_string())?
        .map(|e| e.map(|e| e.path()).map_err(|e| e.to_string()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    if paths.len() < 20 {
        return Err("retained study cases are incomplete".into());
    }
    for path in paths {
        let value = crate::runner::run_case(&path, &registry)?;
        if value.to_string().contains("sampleIds")
            || value.pointer("/result/study/patientCount").is_some()
        {
            return Err("study reply retained members or invented patient count".into());
        }
        reject_members(directory, &value)?;
        let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
        let case: Value = serde_json::from_slice(&fs::read(&path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        let ceiling = case
            .pointer("/steps/1/expect//max_bytes")
            .and_then(Value::as_u64)
            .and_then(|v| usize::try_from(v).ok())
            .ok_or("full response ceiling")?;
        if full + 1 > ceiling {
            return Err("full CLI/MCP response exceeds its measured ceiling".into());
        }
        let comparable = crate::runner::source_terms::size(&value, "")?;
        writeln!(
            std::io::stdout(),
            "Study native {}: CLI={}, MCP={}, comparable={}, estimated_tokens={}",
            path.file_stem().ok_or("case name")?.to_string_lossy(),
            full + 1,
            full,
            comparable,
            comparable.div_ceil(4)
        )
        .map_err(|e| e.to_string())?;
    }
    Ok(())
}

fn reject_members(directory: &Path, value: &Value) -> Check {
    if value.get("schema") != Some(&json!("biomcp.study-sample-lists/2")) {
        return Ok(());
    }
    let id = value
        .pointer("/query/id")
        .and_then(Value::as_str)
        .ok_or("study ID")?;
    let replay = directory
        .parent()
        .ok_or("fixture parent")?
        .join("replay/cbioportal");
    let expected =
        format!("https://www.cbioportal.org/api/studies/{id}/sample-lists?projection=DETAILED");
    let output = value.to_string();
    for entry in fs::read_dir(replay).map_err(|e| e.to_string())? {
        let path = entry.map_err(|e| e.to_string())?.path();
        let recording: Value = serde_json::from_slice(&fs::read(path).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        if recording.get("url").and_then(Value::as_str) != Some(expected.as_str()) {
            continue;
        }
        let raw = recording
            .pointer("/exchanges/0/body/value")
            .and_then(Value::as_str)
            .ok_or("raw body")?;
        let rows: Value = serde_json::from_str(raw).map_err(|e| e.to_string())?;
        for row in rows.as_array().ok_or("raw rows")? {
            for member in row
                .get("sampleIds")
                .and_then(Value::as_array)
                .ok_or("native member IDs")?
            {
                if output.contains(&member.to_string()) {
                    return Err("native sample member entered product output".into());
                }
            }
        }
        return Ok(());
    }
    Err("matching retained sample-list request missing".into())
}

fn rejected(env: &Environment, replay: &Path, section: &str, value: &Value) -> Check {
    if value.to_string().contains("other_study") {
        return Err("wrong-study row survived filtering".into());
    }
    let expected = if section == "profiles" { 16 } else { 12 };
    if value.pointer("/page/total") != Some(&json!(expected)) {
        return Err("rejected row changed accepted total".into());
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    let cached: i64 = db
        .query_row(
            "SELECT count(*) FROM reply WHERE url LIKE '%/studies/synthetic_0/%'",
            [],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    if cached != 0 {
        return Err("wrong-study collection entered checked cache".into());
    }
    let mut args = env.globals(None, true);
    args.extend(["get", "study", "synthetic_0", section].map(str::to_owned));
    let offline = crate::runner::configuration::invoke(env, args, 4)?;
    if offline.get("result").is_some() {
        return Err("wrong-study collection survived offline cache reuse".into());
    }
    if !replay.is_dir() {
        return Err("replay directory disappeared".into());
    }
    Ok(())
}
fn follow(env: &Environment, replay: &Path, value: &Value, kind: &str) -> Result<Value, String> {
    let next = value
        .get("next")
        .and_then(Value::as_array)
        .and_then(|steps| {
            steps
                .iter()
                .find(|s| s.get("kind").and_then(Value::as_str) == Some(kind))
        })
        .ok_or("next step")?;
    let args = next
        .get("args")
        .and_then(Value::as_array)
        .ok_or("step arguments")?
        .iter()
        .map(|a| a.as_str().ok_or("argument"))
        .collect::<Result<Vec<_>, _>>()?;
    let cli = call(env, replay, &args, 0, "get/follow/1")?;
    let args: Vec<String> = args.into_iter().map(str::to_owned).collect();
    let mcp = crate::runner::mcp::call_with_globals(
        env,
        &args,
        false,
        &env.globals(Some(replay), false),
    )?;
    if cli != mcp {
        return Err("emitted study step differs between CLI and MCP".into());
    }
    Ok(cli)
}
fn navigation(env: &Environment, replay: &Path) -> Check {
    let card = call(
        env,
        replay,
        &["get", "study", "synthetic_0"],
        0,
        "get/navigation/1",
    )?;
    let section = follow(env, replay, &card, "section")?;
    let next = follow(env, replay, &section, "page")?;
    if next.pointer("/page/offset") != Some(&json!(5))
        || next.pointer("/result/acquired") != Some(&json!(42))
    {
        return Err("emitted section or page lost offset or acquisition".into());
    }
    call(
        env,
        replay,
        &["get", "study", "synthetic_absent"],
        1,
        "get/navigation/2",
    )?;
    fs::remove_dir_all(replay.join("cbioportal")).map_err(|e| e.to_string())?;
    call(
        env,
        replay,
        &["get", "study", "../escape"],
        2,
        "get/navigation/3",
    )?;
    Ok(())
}

fn call(
    env: &Environment,
    replay: &Path,
    args: &[&str],
    exit: i32,
    phase: &str,
) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).to_owned()));
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &command)
        .output()
        .map_err(|e| e.to_string())?;
    if output.status.code() != Some(exit) || !output.stderr.is_empty() {
        return Err(format!(
            "study command {args:?}: expected exit {exit}, actual {:?}; stdout {}; stderr {}",
            output.status.code(),
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let value = serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())?;
    super::budget(&value, phase, "cli")?;
    let words = args.iter().map(|arg| (*arg).to_owned()).collect::<Vec<_>>();
    let mcp = crate::runner::mcp::call_with_globals(
        env,
        &words,
        exit > 1,
        &env.globals(Some(replay), false),
    )?;
    super::budget(&mcp, phase, "mcp")?;
    Ok(value)
}
