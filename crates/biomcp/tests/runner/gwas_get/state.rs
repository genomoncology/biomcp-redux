//! Warm reads, source policy, stale results and lost continuations remain distinct.
use super::{
    Environment, Stub, Trial, Value, ancestry_body, associations, call, card, fs, gwas_budgets,
    json, seed, source, study,
};
use crate::runner::transcript::trial as transcript_trial;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("gwas-get/synthetic-successful-empty-collections", || {
            empty().map_err(Into::into)
        }),
        transcript_trial("gwas-get/warm-fresh-and-disabled-before-io", || {
            warm().map_err(Into::into)
        }),
        transcript_trial("gwas-get/stale-is-partial-with-fresh-action", || {
            stale().map_err(Into::into)
        }),
        transcript_trial(
            "gwas-get/expired-ancestry-restarts-original-selection",
            || expired().map_err(Into::into),
        ),
    ]
}
fn empty() -> Result<(), String> {
    for (section, body) in [
        ("ancestries", json!({"_embedded":{"ancestries":[]}})),
        ("loci", json!({"_embedded":{"loci":[]}})),
        (
            "associations",
            json!({"_embedded":{"associations":[]},
            "page":{"size":3,"number":0,"totalElements":0,"totalPages":0},
            "_links":{"self":{"href":"https://www.ebi.ac.uk/gwas/rest/api/v2/associations?accession_id=GCST000035&direction=asc&page=0&size=3&sort=p_value"}}}),
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let id = if section == "loci" {
            "association:6114"
        } else {
            "GCST000035"
        };
        let args = ["get", "gwas", id, section];
        let replay = seed(&env, &args, body.to_string(), 200, 0)?;
        let value = call("gwas_get/state/empty/1", &env, &replay, &args, 0)?;
        if value.get("status") != Some(&json!("complete"))
            || value.pointer("/page/total") != Some(&json!(0))
            || value.pointer("/result/items").is_some()
        {
            return Err(format!("successful source zero lost: {value}"));
        }
        bounded(&value)?;
    }
    Ok(())
}
fn warm() -> Result<(), String> {
    let stub = Stub::for_source("www.ebi.ac.uk", "application/json", source)?;
    let env = super::super::variant::source_environment(&stub)?;
    for tail in [
        vec!["association:6114"],
        vec!["GCST000035"],
        vec!["GCST000035", "associations"],
        vec!["GCST000035", "ancestries"],
    ] {
        let args = [vec!["get", "gwas"], tail].concat();
        let args = args.iter().map(|a| (*a).to_owned()).collect::<Vec<_>>();
        let first = gwas_budgets::invoke("gwas_get/state/warm/1", &env, args.clone(), 0)?;
        let second = gwas_budgets::invoke("gwas_get/state/warm/2", &env, args.clone(), 0)?;
        if first != second {
            return Err("warm result changed".into());
        }
        bounded(&first)?;
        let mut fresh = args.clone();
        fresh.push("--fresh".into());
        gwas_budgets::invoke("gwas_get/state/warm/3", &env, fresh, 0)?;
        for selector in ["gwas", "restricted"] {
            let mut disabled = args.clone();
            disabled.extend(["--disable-sources".into(), selector.into()]);
            let value = gwas_budgets::invoke("gwas_get/state/warm/4", &env, disabled, 4)?;
            if value.get("result").is_some()
                || value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            {
                return Err(format!("warm policy bypassed: {value}"));
            }
            bounded(&value)?;
        }
    }
    if stub.finish()?.len() != 8 {
        return Err("warm/fresh/policy request count differs from 8".into());
    }
    Ok(())
}
fn stale() -> Result<(), String> {
    for args in [
        vec!["get", "gwas", "association:6114"],
        vec!["get", "gwas", "GCST000035"],
        vec!["get", "gwas", "GCST000035", "associations"],
        vec!["get", "gwas", "GCST000035", "ancestries"],
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let body = if args.len() == 3 {
            if args.get(2).is_some_and(|id| id.starts_with("association:")) {
                card()
            } else {
                study()
            }
        } else if args.get(3) == Some(&"ancestries") {
            ancestry_body()
        } else {
            associations(
                "/gwas/rest/api/v2/associations?accession_id=GCST000035&direction=asc&page=0&size=3&sort=p_value",
            )
        };
        let replay = seed(&env, &args, body.to_string(), 200, 0)?;
        call("gwas_get/state/stale/1", &env, &replay, &args, 0)?;
        let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
            .map_err(|e| e.to_string())?;
        db.execute("UPDATE reply SET fresh_until=0", [])
            .map_err(|e| e.to_string())?;
        drop(db);
        let mut offline = env.globals(None, true);
        offline.extend(args.iter().map(|a| (*a).to_owned()));
        let value = gwas_budgets::invoke("gwas_get/state/stale/2", &env, offline, 0)?;
        if value.get("status") != Some(&json!("partial"))
            || value.get("result").is_none()
            || value.pointer("/sources/0/state") != Some(&json!("stale"))
        {
            return Err(format!("stale result presented as complete: {value}"));
        }
        if super::super::action(&value, "fresh")?.is_empty() {
            return Err("missing fresh action".into());
        }
        bounded(&value)?;
    }
    Ok(())
}
fn expired() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = [
        "get",
        "gwas",
        "GCST000035",
        "ancestries",
        "--section-limit",
        "1",
    ];
    let replay = seed(&env, &args, ancestry_body().to_string(), 200, 0)?;
    let value = call("gwas_get/state/expired/1", &env, &replay, &args, 0)?;
    let page = super::super::action(&value, "page")?;
    fs::remove_dir_all(replay.join("gwas")).map_err(|e| e.to_string())?;
    for source in ["gwas", "restricted"] {
        let mut command = env.globals(Some(&replay), false);
        command.extend(page.clone());
        command.extend(["--disable-sources".into(), source.into()]);
        let value = gwas_budgets::invoke("gwas_get/state/expired/2", &env, command, 4)?;
        if value.pointer("/sources/0/reason") != Some(&json!("disabled"))
            || value.get("result").is_some()
        {
            return Err(format!(
                "disabled source restored ancestry snapshot: {value}"
            ));
        }
        bounded(&value)?;
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    let changed = db
        .execute("UPDATE trial_snapshot SET expires=0", [])
        .map_err(|e| e.to_string())?;
    if changed != 1 {
        return Err("expected one retained ancestry snapshot".into());
    }
    drop(db);
    let mut command = env.globals(Some(&replay), false);
    command.extend(page);
    let value = gwas_budgets::invoke("gwas_get/state/expired/3", &env, command, 3)?;
    let restart = super::super::action(&value, "restart")?;
    if value.pointer("/error/code") != Some(&json!("pool.expired"))
        || !args.iter().all(|arg| restart.contains(&(*arg).to_owned()))
        || restart.contains(&"--snapshot".to_owned())
    {
        return Err(format!("expired continuation lost selectors: {value}"));
    }
    bounded(&value)
}
fn bounded(value: &Value) -> Result<(), String> {
    let bytes = serde_json::to_vec(value).map_err(|e| e.to_string())?.len();
    if bytes > 7000 || bytes.div_ceil(4) > 1750 {
        return Err("synthetic state exceeds ceiling".into());
    }
    Ok(())
}
