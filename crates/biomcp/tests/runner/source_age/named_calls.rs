//! Replays of the drug commands that motivated the stale-alias change.
use super::{
    name_recovery::invoke,
    names::{expire, unreachable, unread},
};
use crate::runner::{launch::Environment, response_cases::copy_capture, trial_get::command};
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

fn recordings(env: &Environment, name: &str, down: bool) -> Result<PathBuf, String> {
    let replay = env.root().join(name);
    copy_capture("drug/markets", &replay, |source, record| {
        // This replay tests name resolution. Keep the original approval so the case
        // needs no unrecorded supplement letters.
        if source == "drugsfda"
            && record
                .get("url")
                .and_then(Value::as_str)
                .is_some_and(|url| url.contains("application_number%3A%22NDA021335"))
            && let Some(submissions) = record
                .pointer_mut("/exchanges/0/body/value/results/0/submissions")
                .and_then(Value::as_array_mut)
        {
            submissions.retain(|row| row.get("submission_type") == Some(&json!("ORIG")));
        }
    })
    .map_err(|e| e.to_string())?;
    copy_capture("drug/name-lookup-recovery", &replay, |source, record| {
        if source == "mychem" && down {
            unreachable(record);
        }
    })
    .map_err(|e| e.to_string())?;
    Ok(replay)
}

pub(super) fn sti() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let current = recordings(&env, "current", false)?;
    let down = recordings(&env, "down", true)?;
    for (section, source, expected_id) in [
        ("approvals", "drugsfda", "NDA021335"),
        (
            "label",
            "openfda-label",
            "11111111-1111-1111-1111-111111111111",
        ),
    ] {
        let mut args = ["get", "drug", "STI-571", section, "--source", source]
            .map(str::to_owned)
            .to_vec();
        if section == "label" {
            args.extend(["--label-section", "boxed_warning"].map(str::to_owned));
        }
        let (reply, exit) = invoke(&env, &args, &env.globals(Some(&current), false))?;
        if exit != Some(0) || reply.pointer("/result/identity/id") != Some(&json!(expected_id)) {
            return Err(format!(
                "current MyChem did not resolve STI-571 for {section}: {reply}"
            ));
        }
        expire(env.root(), "mychem")?;
        let (reply, _) = invoke(&env, &args, &env.globals(Some(&down), false))?;
        unread(&reply, "mychem", "fresh")?;
    }
    Ok(())
}

pub(super) fn markets() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = recordings(&env, "current", false)?;
    sync(&env, &replay, "ema")?;
    sync(&env, &replay, "who-pq")?;
    let args = ["get", "drug", "zzznotadrug", "markets"].map(str::to_owned);
    let globals = env.globals(Some(&replay), false);
    command(&env, &args, &globals, 1)?;
    expire_ema(&env)?;
    let (reply, _) = invoke(&env, &args, &globals)?;
    unread(&reply, "ema", "sync")
}

pub(super) fn blarcamesine() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = recordings(&env, "current", false)?;
    let down = recordings(&env, "down", true)?;
    let args = [
        "get",
        "drug",
        "blarcamesine",
        "approvals",
        "--source",
        "drugsfda",
    ]
    .map(str::to_owned);
    let globals = env.globals(Some(&replay), false);
    command(&env, &args, &globals, 1)?;
    expire(env.root(), "mychem")?;
    let (reply, _) = invoke(&env, &args, &env.globals(Some(&down), false))?;
    unread(&reply, "mychem", "fresh")
}

fn sync(env: &Environment, replay: &Path, source: &str) -> Result<Value, String> {
    command(
        env,
        &["sync".into(), source.into()],
        &env.globals(Some(replay), false),
        0,
    )
}

fn expire_ema(env: &Environment) -> Result<(), String> {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE bundle SET fresh_until=0 WHERE name='ema'", [])
        .map_err(|e| e.to_string())?;
    Ok(())
}
