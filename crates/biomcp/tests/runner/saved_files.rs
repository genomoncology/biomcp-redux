//! Every checked-in saved.sqlite opens with this build and exports its checked-in JSON
//! (sdlc/planning/saved-work.md, Compatibility 1). BIOMCP_WRITE_SAVED_FIXTURE=DIR first
//! writes this build's file and export under DIR/<version>/ from real commands.
use super::launch::{Environment, fixtures};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;
use std::{fs, path::Path, path::PathBuf};

pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("saved-work/checked-in-files-export", || {
            exports().map_err(Into::into)
        }),
        transcript_trial("saved-work/data-folder-shared-with-cache", || {
            shared_folder().map_err(Into::into)
        }),
        transcript_trial("saved-work/changed-two-recorded-days", || {
            two_days().map_err(Into::into)
        }),
    ]
}

/// Where the product resolves saved.sqlite inside an isolated test environment.
fn saved_file(env: &Environment) -> PathBuf {
    if cfg!(windows) {
        env.root().join("config/biomcp/saved.sqlite")
    } else {
        env.root().join("home/.local/share/biomcp/saved.sqlite")
    }
}

fn run(env: &Environment, args: &[&str]) -> Result<Vec<u8>, String> {
    let args: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    if !output.status.success() {
        return Err(format!(
            "{args:?} failed: {}",
            String::from_utf8_lossy(&output.stdout)
        ));
    }
    Ok(output.stdout)
}

#[expect(
    clippy::disallowed_methods,
    reason = "The fixture writer is selected only by an explicit job variable"
)]
fn requested_output() -> Option<PathBuf> {
    std::env::var_os("BIOMCP_WRITE_SAVED_FIXTURE").map(PathBuf::from)
}

/// Build this release's file through the commands a user runs.
fn write(out: &Path) -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let replay = fixtures().join("replies/trial/condition");
    let mut search = vec![
        "search",
        "trial",
        "--condition",
        "melanoma",
        "--save-as",
        "melanoma-trials",
    ];
    let globals = env.globals(Some(replay.as_path()), false);
    search.extend(globals.iter().map(String::as_str));
    run(&env, &search)?;
    let mut check = vec!["saved", "changed", "melanoma-trials"];
    check.extend(globals.iter().map(String::as_str));
    run(&env, &check)?;
    run(
        &env,
        &[
            "saved",
            "list",
            "add",
            "melanoma-picks",
            "--entity",
            "trial",
            "--note",
            "Recruiting in the US",
            "NCT04631731",
            "NCT05355701",
        ],
    )?;
    run(
        &env,
        &[
            "saved",
            "list",
            "add",
            "braf-genes",
            "--entity",
            "gene",
            "BRAF",
            "NRAS",
        ],
    )?;
    let folder = out.join(env!("CARGO_PKG_VERSION"));
    fs::create_dir_all(&folder).map_err(|error| error.to_string())?;
    fs::copy(saved_file(&env), folder.join("saved.sqlite")).map_err(|error| error.to_string())?;
    let export: Value = serde_json::from_slice(&run(&env, &["saved", "export"])?)
        .map_err(|error| error.to_string())?;
    let mut text = serde_json::to_string_pretty(&export).map_err(|error| error.to_string())?;
    text.push('\n');
    fs::write(folder.join("export.json"), text).map_err(|error| error.to_string())
}

fn exports() -> Result<(), String> {
    if let Some(out) = requested_output() {
        write(&out)?;
    }
    let root = fixtures().join("fixtures/saved");
    let mut folders: Vec<PathBuf> = fs::read_dir(&root)
        .map_err(|error| format!("{}: {error}", root.display()))?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<_, _>>()
        .map_err(|error| error.to_string())?;
    folders.sort();
    if folders.is_empty() {
        return Err("no checked-in saved files".into());
    }
    for folder in folders {
        let env = Environment::new().map_err(|error| error.to_string())?;
        let target = saved_file(&env);
        if let Some(parent) = target.parent() {
            fs::create_dir_all(parent).map_err(|error| error.to_string())?;
        }
        fs::copy(folder.join("saved.sqlite"), &target).map_err(|error| error.to_string())?;
        let actual: Value = serde_json::from_slice(&run(&env, &["saved", "export"])?)
            .map_err(|error| error.to_string())?;
        let expected: Value = serde_json::from_str(
            &fs::read_to_string(folder.join("export.json")).map_err(|error| error.to_string())?,
        )
        .map_err(|error| error.to_string())?;
        if actual != expected {
            return Err(format!(
                "{}: export differs\nactual: {actual}\nexpected: {expected}",
                folder.display()
            ));
        }
    }
    Ok(())
}

/// With the data folder set to the cache folder, store clean still keeps saved.sqlite.
fn shared_folder() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let cache = env.root().join("cache");
    let run_shared = |args: &[&str]| -> Result<Value, String> {
        let args: Vec<String> = args.iter().map(|arg| (*arg).to_owned()).collect();
        let output = env
            .command(env!("CARGO_BIN_EXE_biomcp"), &args)
            .env("XDG_DATA_HOME", &cache)
            .env("APPDATA", &cache)
            .output()
            .map_err(|error| error.to_string())?;
        if !output.status.success() {
            return Err(format!(
                "{args:?} failed: {}",
                String::from_utf8_lossy(&output.stdout)
            ));
        }
        serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())
    };
    let saved = cache.join("biomcp/saved.sqlite");
    run_shared(&["saved", "delete", "--all"])?;
    run_shared(&["saved", "show"])?;
    if saved.exists() {
        return Err("reads and an empty delete created saved.sqlite".into());
    }
    run_shared(&[
        "saved",
        "list",
        "add",
        "braf-genes",
        "--entity",
        "gene",
        "BRAF",
    ])?;
    if !saved.exists() {
        return Err("saved.sqlite is not in the shared cache folder".into());
    }
    run_shared(&["store", "clean"])?;
    let shown = run_shared(&["saved", "show", "braf-genes"])?;
    if !saved.exists()
        || shown.pointer("/result/lists/0/items/0/ref/id") != Some(&Value::from("BRAF"))
    {
        return Err(format!("store clean lost the list: {shown}"));
    }
    Ok(())
}

/// One saved-changed call against a replay folder: its exit code and reply.
fn changed(env: &Environment, name: &str, replay: &Path) -> Result<(i32, Value), String> {
    let mut args = vec!["saved".to_owned(), "changed".to_owned(), name.to_owned()];
    args.extend(env.globals(Some(replay), false));
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &args)
        .output()
        .map_err(|error| error.to_string())?;
    let reply = serde_json::from_slice(&output.stdout).map_err(|error| error.to_string())?;
    Ok((output.status.code().unwrap_or(-1), reply))
}

/// Move the last check back past the once-an-hour floor, as time passing would.
fn age(env: &Environment, sql: &str) -> Result<(), String> {
    let connection =
        rusqlite::Connection::open(saved_file(env)).map_err(|error| error.to_string())?;
    connection
        .execute_batch(&format!(
            "UPDATE saved_search SET checked=checked-{}; {sql}",
            3_601
        ))
        .map_err(|error| error.to_string())
}

fn ids(report: &Value, field: &str) -> Vec<String> {
    let mut ids: Vec<String> = report
        .get(field)
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|reference| reference.get("id")?.as_str().map(str::to_owned))
        .collect();
    ids.sort();
    ids
}

fn expect_state(reply: &Value, state: &str) -> Result<&Value, String> {
    let report = reply
        .pointer("/result/changes/0")
        .ok_or_else(|| format!("no change report: {reply}"))?;
    if report.get("state") != Some(&Value::from(state)) {
        return Err(format!("expected {state}: {reply}"));
    }
    Ok(report)
}

/// Two recorded days of one trial search: the report equals the hand count in
/// tests/replies/saved/changed/hand-count.json, inside the floor nothing runs, and a
/// snapshot under another row schema rebaselines instead of listing every row.
fn two_days() -> Result<(), String> {
    let env = Environment::new().map_err(|error| error.to_string())?;
    let folder = fixtures().join("replies/saved/changed");
    let count: Value = serde_json::from_str(
        &fs::read_to_string(folder.join("hand-count.json")).map_err(|error| error.to_string())?,
    )
    .map_err(|error| error.to_string())?;
    let (day1, day2) = (folder.join("day1"), folder.join("day2"));
    let mut save: Vec<String> = count
        .get("args")
        .and_then(Value::as_array)
        .ok_or("hand count names no search words")?
        .iter()
        .filter_map(|word| word.as_str().map(str::to_owned))
        .collect();
    save.extend(["--save-as".into(), "watched".into()]);
    save.extend(env.globals(Some(&day1), false));
    let save: Vec<&str> = save.iter().map(String::as_str).collect();
    run(&env, &save)?;
    let (_, first) = changed(&env, "watched", &day1)?;
    let report = expect_state(&first, "first")?;
    if report.pointer("/to/total") != count.get("day1_total") {
        return Err(format!("day one total differs from the hand count: {first}"));
    }
    // Nothing is due, so an empty replay folder, which answers no request, is never read.
    let empty = env.root().join("no-replies");
    fs::create_dir_all(&empty).map_err(|error| error.to_string())?;
    let (code, not_due) = changed(&env, "watched", &empty)?;
    expect_state(&not_due, "not_due")?;
    if code != 0 {
        return Err(format!("not_due exited {code}"));
    }
    age(&env, "")?;
    let (_, second) = changed(&env, "watched", &day2)?;
    let report = expect_state(&second, "changed")?;
    for field in ["new", "gone", "changed"] {
        let mut expected: Vec<String> = count
            .get(field)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(|id| id.as_str().map(str::to_owned))
            .collect();
        expected.sort();
        if ids(report, field) != expected {
            return Err(format!("{field} differs from the hand count: {second}"));
        }
    }
    // Snapshots an older build fingerprinted under another row schema.
    age(
        &env,
        "UPDATE saved_snapshot SET row_schema='biomcp.trial-page/16#0000000000000000';",
    )?;
    let (_, rebaselined) = changed(&env, "watched", &day2)?;
    let report = expect_state(&rebaselined, "rebaselined")?;
    if ["new", "gone", "changed"]
        .iter()
        .any(|field| !ids(report, field).is_empty())
    {
        return Err(format!("a rebaseline listed rows: {rebaselined}"));
    }
    age(&env, "")?;
    let (_, unchanged) = changed(&env, "watched", &day2)?;
    expect_state(&unchanged, "unchanged")?;
    let export: Value = serde_json::from_slice(&run(&env, &["saved", "export"])?)
        .map_err(|error| error.to_string())?;
    let kept = export
        .pointer("/result/snapshots")
        .and_then(Value::as_array)
        .map_or(0, Vec::len);
    if kept != 2 {
        return Err(format!("kept {kept} snapshots, not the two newest"));
    }
    retired(&env, &day2)
}

/// A saved search whose words no longer parse answers the usage error and runs nothing.
fn retired(env: &Environment, replay: &Path) -> Result<(), String> {
    age(
        env,
        "UPDATE saved_search SET args='[\"search\",\"trial\",\"--no-such-filter\",\"x\"]';",
    )?;
    let (code, reply) = changed(env, "watched", replay)?;
    let usage = reply
        .pointer("/error/code")
        .and_then(Value::as_str)
        .is_some_and(|code| code.starts_with("usage"));
    if code != 2 || !usage {
        return Err(format!("a saved search that no longer parses ran: {reply}"));
    }
    Ok(())
}
