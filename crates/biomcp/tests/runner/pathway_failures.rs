//! Feature pathway-failures: synthetic source faults cannot become empty or cached answers.
use crate::runner::pathway_budgets as budgets;
use crate::runner::transcript::trial as transcript_trial;
mod absence;
mod fixtures;
mod oversized;
mod wikipathways;
use super::launch::Environment;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::path::Path;
type Check = Result<(), String>;
const ROUTES: &[(&str, &[&str])] = &[
    ("/search/query", &["search", "pathway", "Synthetic"]),
    ("/data/query/", &["get", "pathway", fixtures::ID]),
    ("/participants/", &["pathway", "genes", fixtures::ID]),
    ("/containedEvents", &["pathway", "events", fixtures::ID]),
    (
        "/mapping/UniProt/",
        &["search", "pathway", "--gene", "HGNC:1097"],
    ),
];
pub(super) fn trials() -> Vec<Trial> {
    let mut trials: Vec<_> = ROUTES
        .iter()
        .map(|&(route, args)| {
            transcript_trial(format!("pathway-failures/synthetic-{route}"), move || {
                faults(route, args).map_err(Into::into)
            })
        })
        .collect();
    trials.push(transcript_trial(
        "pathway-failures/synthetic-foreign-reactome-identity",
        || identities().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "pathway-failures/unsupported-pairings-before-io",
        || pairings().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "pathway-failures/synthetic-exact-absence-versus-foreign404",
        || absence::run().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "pathway-failures/kept-limit-aliases",
        || limits().map_err(Into::into),
    ));
    trials.push(transcript_trial(
        "pathway-failures/synthetic-oversized-source-admission",
        || oversized::run().map_err(Into::into),
    ));
    trials.extend(wikipathways::trials());
    trials
}
fn call(
    phase: &str,
    env: &Environment,
    root: &Path,
    args: &[&str],
    exit: i32,
) -> Result<Value, String> {
    let mut words = env.globals(Some(root), false);
    words.extend(args.iter().map(|s| (*s).to_owned()));
    let value = budgets::invoke(phase, env, words, exit)?;
    if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() > 4000 {
        return Err("source outcome exceeds size ceiling".into());
    }
    Ok(value)
}
fn faults(route: &str, args: &[&str]) -> Check {
    let template = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&template)?;
    for (label, status, body, delay, reason) in [
        ("429", 429, "{}", 0, "rate-limited"),
        ("500", 500, "{}", 0, "unavailable"),
        ("timeout", 200, "{}", 9000, "deadline-exceeded"),
        ("invalid JSON", 200, "{", 0, "bad-reply"),
        (
            "HTML success",
            200,
            "<html>source failed</html>",
            0,
            "bad-reply",
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        fixtures::replace(&root, route, body, status, delay)?;
        let value = call("pathway_failures/faults/1", &env, &root, args, 4)
            .map_err(|e| format!("{route} {label}: {e}"))?;
        failed(&value, reason).map_err(|e| format!("{route} {label}: {e}"))?;
        uncached(&env, route)?;
    }
    Ok(())
}
fn failed(value: &Value, reason: &str) -> Check {
    if serde_json::to_vec(value).map_err(|e| e.to_string())?.len() > 4000 {
        return Err("source failure exceeds size ceiling".into());
    }
    let source = value
        .get("sources")
        .and_then(Value::as_array)
        .and_then(|v| {
            v.iter()
                .find(|s| s.get("source") == Some(&json!("reactome")))
        })
        .ok_or("missing failed source")?;
    if value.get("status") != Some(&json!("failed"))
        || source.get("state") != Some(&json!("failed"))
        || source.get("reason") != Some(&json!(reason))
        || source.get("terms") != Some(&json!("CC0 1.0 database data"))
        || source.get("terms_url") != Some(&json!("https://reactome.org/license"))
    {
        return Err(format!("fault became empty or lost source/terms: {value}"));
    }
    Ok(())
}
fn uncached(env: &Environment, route: &str) -> Check {
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    let count: i64 = db
        .query_row(
            "SELECT count(*) FROM reply WHERE source='reactome' AND instr(url,?1)>0",
            [route],
            |r| r.get(0),
        )
        .map_err(|e| e.to_string())?;
    if count != 0 {
        return Err(format!("rejected {route} entered reply cache"));
    }
    Ok(())
}
fn identities() -> Check {
    let template = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&template)?;
    for (field, foreign) in [
        ("stId", "R-HSA-123"),
        ("speciesName", "Mus musculus"),
        ("schemaClass", "Reaction"),
    ] {
        let mut card = fixtures::card();
        *card.get_mut(field).ok_or("field")? = json!(foreign);
        fixtures::replace(&root, "/data/query/", &card.to_string(), 200, 0)?;
        let env = Environment::new().map_err(|e| e.to_string())?;
        let value = call(
            "pathway_failures/identities/1",
            &env,
            &root,
            &["get", "pathway", fixtures::ID],
            0,
        )?;
        if value.get("result").is_some_and(|v| !v.is_null())
            || !value
                .get("problems")
                .and_then(Value::as_array)
                .is_some_and(|v| {
                    v.iter().any(|p| {
                        p.get("source") == Some(&json!("reactome"))
                            && p.get("field") == Some(&json!("identity"))
                    })
                })
        {
            return Err(format!(
                "foreign {field} became a card or lost problem: {value}"
            ));
        }
        uncached(&env, "/data/query/")?;
    }
    Ok(())
}
fn pairings() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let socket = std::net::TcpListener::bind("127.0.0.1:0").map_err(|e| e.to_string())?;
    socket.set_nonblocking(true).map_err(|e| e.to_string())?;
    env.config(
        &json!({"proxy":format!("http://{}",socket.local_addr().map_err(|e|e.to_string())?)}),
    )
    .map_err(|e| e.to_string())?;
    for words in [
        "get pathway R-MMU-5673001",
        "get pathway WP382 events",
        "pathway events WP382",
        "search pathway MAPK --source kegg",
        "search pathway --gene BRAF --source wikipathways",
        "search pathway --top-level --source wikipathways",
    ] {
        let value = budgets::invoke(
            "runner/pathway_failures/pairings/1",
            &env,
            words.split_whitespace().map(str::to_owned).collect(),
            2,
        )?;
        if value.get("status") != Some(&json!("usage"))
            || value.pointer("/error/message")
                != Some(&json!(
                    "expected a supported command with checked arguments"
                ))
            || value.get("sources").is_some()
        {
            return Err(format!("unsupported pairing reached source: {value}"));
        }
        if serde_json::to_vec(&value).map_err(|e| e.to_string())?.len() > 1000 {
            return Err("pairing usage exceeds ceiling".into());
        }
    }
    if socket.accept().is_ok() {
        return Err("unsupported pairing opened provider connection".into());
    }
    Ok(())
}

fn limits() -> Check {
    let template = Environment::new().map_err(|e| e.to_string())?;
    let root = fixtures::seed(&template)?;
    for args in [
        &["search", "pathway", "Synthetic"][..],
        &["pathway", "events", fixtures::ID][..],
    ] {
        for option in ["--limit", "-l"] {
            let env = Environment::new().map_err(|e| e.to_string())?;
            let mut words = args.to_vec();
            words.extend([option, "5"]);
            let value = call("pathway_failures/limits/1", &env, &root, &words, 0)?;
            if value.pointer("/result/page/returned") != Some(&json!(1)) {
                return Err(format!("kept limit spelling changed page: {value}"));
            }
        }
    }
    Ok(())
}
