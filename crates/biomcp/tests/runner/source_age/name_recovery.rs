//! Drug name lookups preserve the distinction between unread, absent, and stale matches.
use super::names::{unreachable, unread};
use crate::runner::{
    launch::{Environment, fixtures},
    response_cases::{CACHE_ACQUIRED, cache_acquisition, copy_capture},
    trial_get::command,
};
use serde_json::{Value, json};

type Check = Result<(), String>;
const RECOVERY: &str = "drug/name-lookup-recovery";

fn environment() -> Result<Environment, String> {
    Environment::new().map_err(|e| e.to_string())
}

fn approvals_args(name: &str) -> [String; 6] {
    ["get", "drug", name, "approvals", "--source", "drugsfda"].map(str::to_owned)
}

pub(super) fn capped_alias() -> Check {
    let env = environment()?;
    let replay = env.root().join("replay");
    copy_capture(RECOVERY, &replay, |_, _| {}).map_err(|e| e.to_string())?;
    copy_capture("drug/name-get-pages", &replay, |_, _| {}).map_err(|e| e.to_string())?;
    let args = approvals_args("Buried therapy");
    let (reply, _) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    unread(&reply, "mychem", "fresh")
}

pub(super) fn capped_product() -> Check {
    let env = environment()?;
    let replay = env.root().join("replay");
    copy_capture(RECOVERY, &replay, |_, _| {}).map_err(|e| e.to_string())?;
    copy_capture("drug/name-get-pages", &replay, |_, _| {}).map_err(|e| e.to_string())?;
    let args = approvals_args("Capped product");
    let (reply, _) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    unread(&reply, "mychem", "fresh")
}

pub(super) fn short_chembl_copy() -> Check {
    let env = environment()?;
    let replay = env.root().join("replay");
    copy_capture(RECOVERY, &replay, |_, _| {}).map_err(|e| e.to_string())?;
    let mut changed = false;
    copy_capture("drug/source-aliases", &replay, |source, record| {
        let url = record
            .get("url")
            .and_then(Value::as_str)
            .unwrap_or_default();
        let direct = url.contains("q=Orphan%20code&");
        let copy = url.contains("q=chembl.pref_name%3A%22Orphan%20code");
        if source == "mychem"
            && (direct || copy)
            && let Some(body) = record.pointer_mut("/exchanges/0/body/value")
        {
            if direct {
                *body = json!({"total": 0, "hits": []});
                return;
            }
            let hits: Vec<_> = (0..50)
                .map(|i| {
                    let (first, second) = (char::from(b'A' + i / 26), char::from(b'A' + i % 26));
                    json!({"_id": format!("SYNTHETIC{first}{second}AAA-UHFFFAOYSA-N")})
                })
                .collect();
            *body = json!({"total": 51, "hits": hits});
            changed = true;
        }
    })
    .map_err(|e| e.to_string())?;
    if !changed {
        return Err("ChEMBL-copy fixture was not changed".into());
    }
    let args = approvals_args("Orphan code");
    let (reply, _) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    unread(&reply, "mychem", "fresh")
}

pub(super) fn deadline() -> Check {
    let env = environment()?;
    env.config(&json!({"command_timeout_seconds":1}))
        .map_err(|e| e.to_string())?;
    let replay = env.root().join("replay");
    copy_capture(RECOVERY, &replay, |_, _| {}).map_err(|e| e.to_string())?;
    copy_capture("drug/source-aliases", &replay, |source, record| {
        if source == "mychem"
            && record
                .get("url")
                .and_then(Value::as_str)
                .is_some_and(|u| u.contains("q=Orphan%20code&"))
            && let Some(delay) = record.pointer_mut("/exchanges/0/delay_ms")
        {
            *delay = json!(2_000);
        }
    })
    .map_err(|e| e.to_string())?;
    let args = approvals_args("Orphan code");
    let (reply, _) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    unread(&reply, "openfda-label", "fresh")
}

pub(super) fn label_headings() -> Check {
    let env = environment()?;
    let replay = env.root().join("replay");
    copy_capture("drug/several-sections-name", &replay, |_, record| {
        unreachable(record)
    })
    .map_err(|e| e.to_string())?;
    let args = [
        "get",
        "drug",
        "EXAMPLE LABEL",
        "label",
        "--label-section",
        "boxed_warning,contraindications",
        "--section-limit",
        "2",
        "--section-offset",
        "1",
        "--source",
        "openfda-label",
    ]
    .map(str::to_owned);
    let (reply, exit) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    let retries: Vec<_> = reply
        .pointer("/result/parts")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|part| {
            part.get("reply")
                .and_then(|reply| super::super::super::action(reply, "fresh").ok())
        })
        .collect();
    if exit != Some(4)
        || reply.pointer("/query/section_limit") != Some(&json!(2))
        || reply.pointer("/query/section_offset") != Some(&json!(1))
        || !["boxed_warning", "contraindications"]
            .iter()
            .all(|heading| {
                retries.iter().any(|args| {
                    args.iter().any(|arg| arg == heading)
                        && args.windows(2).any(|pair| pair == ["--limit", "2"])
                        && args.windows(2).any(|pair| pair == ["--offset", "1"])
                })
            })
    {
        return Err(format!(
            "failed name discovery lost label headings or paging: {reply}"
        ));
    }
    Ok(())
}

pub(super) fn chembl_match() -> Check {
    let env = environment()?;
    let args = ["get", "drug", "Canonical therapy", "--source", "chembl"].map(str::to_owned);
    let replay = fixtures().join("replies/drug/source-aliases");
    let current = command(&env, &args, &env.globals(Some(&replay), false), 0)?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    for selector in ["pref_name__iexact", "molecule_synonym__iexact"] {
        db.execute(
            "UPDATE reply SET fresh_until=CASE WHEN instr(url, ?1)>0 THEN 0 ELSE 4102444800 END WHERE source='chembl'",
            [selector],
        )
        .map_err(|e| e.to_string())?;
        let (reply, exit) = invoke(&env, &args, &env.globals(None, true))?;
        if exit != Some(0)
            || reply.get("status") != Some(&json!("partial"))
            || reply.pointer("/result/header/id") != current.pointer("/result/header/id")
        {
            return Err(format!(
                "stale ChEMBL {selector} lookup discarded its record: {reply}"
            ));
        }
        unread(&reply, "chembl", "fresh")?;
    }
    Ok(())
}

pub(super) fn chembl_sections() -> Check {
    let env = environment()?;
    let replay = env.root().join("replay");
    for capture in [RECOVERY, "drug/source-aliases", "drug/native-chembl"] {
        copy_capture(capture, &replay, |source, record| {
            cache_acquisition(record, CACHE_ACQUIRED);
            if source == "chembl"
                && record
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|u| u.ends_with("/molecule/CHEMBL941.json"))
                && let Some(exchanges) = record.get_mut("exchanges").and_then(Value::as_array_mut)
                && let Some(exchange) = exchanges.first().cloned()
            {
                exchanges.push(exchange);
            }
        })
        .map_err(|e| e.to_string())?;
    }
    let args = [
        "get",
        "drug",
        "Canonical therapy",
        "targets",
        "indications",
        "--source",
        "chembl",
        "--section-limit",
        "1",
    ]
    .map(str::to_owned);
    let (current, exit) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    if exit != Some(0) || current.get("status") != Some(&json!("complete")) {
        return Err(format!(
            "ChEMBL sections did not both resolve from replay: {current}"
        ));
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute(
        "UPDATE reply SET fresh_until=CASE WHEN instr(url, 'pref_name__iexact')>0 THEN 0 ELSE 4102444800 END WHERE source='chembl'",
        [],
    )
    .map_err(|e| e.to_string())?;
    let (reply, exit) = invoke(&env, &args, &env.globals(None, true))?;
    if exit != Some(0) || reply.get("status") != Some(&json!("partial")) {
        return Err(format!(
            "stale ChEMBL lookup did not keep its sections: {reply}"
        ));
    }
    for (index, section) in ["targets", "indications"].into_iter().enumerate() {
        let part = reply
            .pointer(&format!("/result/parts/{index}/reply"))
            .ok_or_else(|| format!("missing {section} part: {reply}"))?;
        unread(part, "chembl", "fresh")?;
        let retry = super::super::super::action(part, "fresh")?;
        if !retry
            .windows(2)
            .any(|pair| pair == ["Canonical therapy", section])
        {
            return Err(format!(
                "fresh retry for {section} changes its request: {retry:?}"
            ));
        }
    }
    Ok(())
}

pub(super) fn market_retry() -> Check {
    let env = environment()?;
    let replay = env.root().join("replay");
    copy_capture(RECOVERY, &replay, |_, _| {}).map_err(|e| e.to_string())?;
    copy_capture("drug/source-aliases", &replay, |source, record| {
        if source == "mychem" {
            unreachable(record);
        }
    })
    .map_err(|e| e.to_string())?;
    let args = [
        "get",
        "drug",
        "Orphan code",
        "markets",
        "--source",
        "drugsfda",
    ]
    .map(str::to_owned);
    let (reply, _) = invoke(&env, &args, &env.globals(Some(&replay), false))?;
    unread(&reply, "mychem", "fresh")?;
    let retry = super::super::super::action(&reply, "fresh")?;
    if !args.iter().all(|arg| retry.contains(arg)) || !retry.iter().any(|arg| arg == "--fresh") {
        return Err(format!("fresh retry changed the markets request: {reply}"));
    }
    Ok(())
}

pub(super) fn invoke(
    env: &Environment,
    args: &[String],
    globals: &[String],
) -> Result<(Value, Option<i32>), String> {
    let mut all = args.to_vec();
    all.extend_from_slice(globals);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &all)
        .output()
        .map_err(|e| e.to_string())?;
    let reply = serde_json::from_slice(&output.stdout)
        .map_err(|e| format!("{e}: {}", String::from_utf8_lossy(&output.stderr)))?;
    Ok((reply, output.status.code()))
}
