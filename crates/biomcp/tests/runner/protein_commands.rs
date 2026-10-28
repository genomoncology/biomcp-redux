//! Feature protein-search: dynamic pool bindings and lifetimes through the shipped command.
use super::{configuration::invoke, launch::Environment};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::Value;
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("protein-search/continuation-count-bound", || {
            continuation_count().map_err(|e| e.to_string().into())
        }),
        transcript_trial(
            "protein-search/continuation-keeps-publication-lifetime",
            || continuation_lifetime().map_err(Into::into),
        ),
        transcript_trial("protein-search/pool-binding-and-lifetime", || {
            pools().map_err(Into::into)
        }),
        transcript_trial("protein-search/native-continuation-admission", || {
            links().map_err(|e| e.to_string().into())
        }),
        transcript_trial("protein-search/row-cache-admission", || {
            cache_admission().map_err(|e| e.to_string().into())
        }),
    ]
}
fn call(env: &Environment, args: &[&str], exit: i32) -> Result<Value, String> {
    let replay = super::launch::fixtures().join("replies/protein/synthetic");
    let mut words = env.globals(Some(&replay), false);
    words.extend(args.iter().map(|s| (*s).into()));
    invoke(env, words, exit)
}
fn id(value: &Value) -> Result<&str, String> {
    value
        .pointer("/page/pool/id")
        .and_then(Value::as_str)
        .ok_or_else(|| "missing protein pool".into())
}
fn pools() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["search", "protein", "--gene", "BRAF"];
    let first = call(&env, &args, 0)?;
    super::protein_budgets::check(&first, "search-pool/first", "cli")?;
    let pool = id(&first)?;
    let retained = call(
        &env,
        &["search", "protein", "--gene", "BRAF", "--pool", pool],
        0,
    )?;
    super::protein_budgets::check(&retained, "search-pool/retained", "cli")?;
    if first.pointer("/page/pool") != retained.pointer("/page/pool") {
        return Err("retained protein pool changed its publication metadata".into());
    }
    let fresh = call(&env, &["search", "protein", "--gene", "BRAF", "--fresh"], 0)?;
    super::protein_budgets::check(&fresh, "search-pool/fresh", "cli")?;
    if pool == id(&fresh)? {
        return Err("fresh protein search reused a published pool".into());
    }
    let mismatch = call(
        &env,
        &["search", "protein", "--gene", "EGFR", "--pool", pool],
        2,
    )?;
    super::protein_budgets::check(&mismatch, "search-pool/mismatch", "cli")?;
    if mismatch.pointer("/error/code").and_then(Value::as_str) != Some("usage.pool_mismatch") {
        return Err("changed protein selectors did not reject the old pool".into());
    }
    let terminal = call(
        &env,
        &[
            "search", "protein", "--gene", "BRAF", "--pool", pool, "--offset", "1",
        ],
        0,
    )?;
    super::protein_budgets::check(&terminal, "search-pool/terminal", "cli")?;
    if terminal.pointer("/page/returned").and_then(Value::as_u64) != Some(0) {
        return Err("terminal protein page returned a duplicate".into());
    }
    let invalid = call(
        &env,
        &[
            "search", "protein", "--gene", "BRAF", "--pool", pool, "--offset", "2",
        ],
        3,
    )?;
    super::protein_budgets::check(&invalid, "search-pool/invalid-offset", "cli")?;
    if invalid.pointer("/next/0/kind").and_then(Value::as_str) != Some("restart") {
        return Err("invalid retained protein offset omitted restart".into());
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE trial_snapshot SET expires=0 WHERE id=?1", [pool])
        .map_err(|e| e.to_string())?;
    let expired = call(
        &env,
        &["search", "protein", "--gene", "BRAF", "--pool", pool],
        3,
    )?;
    super::protein_budgets::check(&expired, "search-pool/expired", "cli")?;
    if expired.pointer("/error/code").and_then(Value::as_str) != Some("pool.expired") {
        return Err("expired protein pool refetched instead of restarting".into());
    }
    Ok(())
}

fn links() -> Result<(), Box<dyn std::error::Error>> {
    for (before, after) in [
        ("rest.uniprot.org", "example.invalid"),
        ("%28EGFR%29", "%28OTHER%29"),
        ("size=5", "size=6"),
        (
            "cursor=native-second",
            "cursor=native-second&unexpected=field",
        ),
        ("uniprotkb/search", "other/search"),
    ] {
        let value = super::response_cases::checked_case(
            "protein/pages",
            &["search", "protein", "EGFR"],
            |_, record| {
                if record
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|u| !u.contains("cursor="))
                    && let Some(next) = record.pointer_mut("/exchanges/0/pagination/next")
                    && let Some(text) = next.as_str()
                {
                    *next = serde_json::json!(text.replace(before, after));
                }
            },
            serde_json::json!({"/status":"failed", "/sources/0/reason":"bad_reply"}),
            4,
        )?;
        super::protein_budgets::check(
            &value,
            &format!("search-continuation-rejection/{before}/{after}"),
            "cli",
        )?;
    }
    Ok(())
}

fn cache_admission() -> Result<(), Box<dyn std::error::Error>> {
    for (pointer, value) in [
        ("/organism/taxonId", serde_json::json!(10090)),
        (
            "/entryType",
            serde_json::json!("UniProtKB unreviewed (TrEMBL)"),
        ),
    ] {
        super::protein_budgets::steps(
            &format!("search-cache-admission/{pointer}"),
            "protein/pages",
            |_, record| {
                if record
                    .get("url")
                    .and_then(Value::as_str)
                    .is_some_and(|u| !u.contains("cursor="))
                    && let Some(row) = record.pointer_mut("/exchanges/0/body/value/results/0")
                    && let Some(field) = row.pointer_mut(pointer)
                {
                    *field = value.clone();
                }
            },
            serde_json::json!([
                {"run":["search","protein","EGFR"], "exit":0,
                 "expect":{"/status":"partial", "/problems/0/code":"value.unknown"}},
                {"run":["search","protein","EGFR","--fresh"], "offline":true,"exit":4,
                 "expect":{"/status":"failed"}}
            ]),
        )?;
    }
    Ok(())
}

/// Measure the ordinary CLI and MCP responses against the accepted external replay.
pub(super) fn native(root: &std::path::Path) -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let globals = env.globals(Some(&root.join("replay")), false);
    for words in [
        vec!["get", "protein", "P00533"],
        vec!["get", "protein", "Q9UIF7"],
        vec!["get", "protein", "P38398-2"],
        vec!["get", "protein", "P42771", "Q8N726"],
        vec!["get", "protein", "BRAF"],
        vec!["get", "protein", "CDKN2A"],
        vec!["search", "protein", "--gene", "BRAF"],
        vec!["search", "protein", "--gene", "CDKN2A"],
        vec!["search", "protein", "EGFR"],
    ] {
        let args: Vec<String> = words.iter().map(|s| (*s).into()).collect();
        let cli = measure_native(&env, &globals, &args)?;
        if words == ["search", "protein", "EGFR"] {
            measure_native(&env, &globals, &super::action(&cli, "page")?)?;
        }
    }
    Ok(())
}
fn measure_native(env: &Environment, globals: &[String], args: &[String]) -> Result<Value, String> {
    use std::io::Write;
    let mut command = globals.to_vec();
    command.extend_from_slice(args);
    let cli = invoke(env, command.clone(), 0)?;
    let mcp = super::mcp::call_with_globals(env, args, false, globals)?;
    for key in ["status", "result", "query", "page"] {
        if cli.get(key) != mcp.get(key) {
            return Err(format!("protein CLI/MCP {key} differs for {args:?}"));
        }
    }
    for (kind, value) in [("CLI", &cli), ("MCP", &mcp)] {
        let comparable = super::source_terms::size(value, "")?;
        writeln!(
            std::io::stderr(),
            "protein {kind} {args:?}: full={} comparable={comparable} ceiling={}",
            serde_json::to_vec(value).map_err(|e| e.to_string())?.len(),
            (comparable * 115).div_ceil(100)
        )
        .map_err(|e| e.to_string())?;
    }
    command.extend(["--format".into(), "md".into()]);
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), &command)
        .output()
        .map_err(|e| e.to_string())?;
    let markdown = String::from_utf8(output.stdout).map_err(|e| e.to_string())?;
    let identities = cli
        .pointer("/result/items")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
        .filter_map(|item| {
            item.pointer("/header/protein/accession")
                .or_else(|| item.pointer("/protein/accession"))
                .and_then(Value::as_str)
        });
    if !output.status.success()
        || markdown.trim().is_empty()
        || identities
            .into_iter()
            .any(|id| !markdown.contains(&super::source_terms::text::escape(id)))
    {
        return Err(format!("protein Markdown failed for {args:?}"));
    }
    Ok(cli)
}

fn continuation_lifetime() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let replay = super::launch::fixtures().join("replies/protein/pages");
    let mut args = env.globals(Some(&replay), false);
    args.extend(["search", "protein", "EGFR"].map(str::to_owned));
    let first = invoke(&env, args.clone(), 0)?;
    super::protein_budgets::check(&first, "native-continuation/first", "cli")?;
    args.extend(["--pool", id(&first)?, "--offset", "5"].map(str::to_owned));
    let next = invoke(&env, args, 0)?;
    super::protein_budgets::check(&next, "native-continuation/next", "cli")?;
    for key in ["id", "created", "expires"] {
        let pointer = format!("/page/pool/{key}");
        if first.pointer(&pointer) != next.pointer(&pointer) {
            return Err(format!("native continuation changed pool {key}"));
        }
    }
    if next.pointer("/page/pool/size").and_then(Value::as_u64) != Some(7) {
        return Err("native continuation did not extend the retained window".into());
    }
    Ok(())
}

fn continuation_count() -> Result<(), Box<dyn std::error::Error>> {
    let value = super::response_cases::checked_case(
        "protein/pages",
        &["search", "protein", "EGFR", "--limit", "25"],
        |_, record| {
            if record
                .get("url")
                .and_then(Value::as_str)
                .is_some_and(|u| u.contains("cursor="))
                && let Some(rows) = record
                    .pointer_mut("/exchanges/0/body/value/results")
                    .and_then(Value::as_array_mut)
                && let Some(template) = rows.first().cloned()
            {
                for id in ["P04626", "P01133", "P22681"] {
                    let mut row = template.clone();
                    if let Some(accession) = row.get_mut("primaryAccession") {
                        *accession = serde_json::json!(id);
                    }
                    rows.push(row);
                }
            }
        },
        serde_json::json!({"/status":"partial", "/sources/0/reason":"bad_reply",
            "/page/returned":5, "/page/total":7}),
        0,
    )?;
    super::protein_budgets::check(&value, "search-continuation/declared-count-rejected", "cli")?;
    Ok(())
}
