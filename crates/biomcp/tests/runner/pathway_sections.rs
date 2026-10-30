//! Feature pathway-reactome: retained source membership and event walks through CLI and MCP.
use super::launch::{Environment, fixtures};
use crate::runner::pathway_budgets as budgets;
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{collections::BTreeSet, io::Write};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        transcript_trial("pathway-reactome/native-reference-walk", || {
            walk(true).map_err(Into::into)
        }),
        transcript_trial("pathway-reactome/native-event-walk", || {
            walk(false).map_err(Into::into)
        }),
        transcript_trial("pathway-reactome/pool-binding-and-fixed-lifetime", || {
            bindings().map_err(Into::into)
        }),
        transcript_trial("pathway-reactome/warm-hgnc-uniprot-index-migration", || {
            migration().map_err(Into::into)
        }),
        transcript_trial(
            "pathway-reactome/compact-and-complete-context-modes",
            || disclosure_modes().map_err(Into::into),
        ),
        transcript_trial("pathway-reactome/native-disclosure-measurements", || {
            measurements().map_err(Into::into)
        }),
    ]
}
fn globals(env: &Environment, offline: bool) -> Vec<String> {
    env.globals(Some(&fixtures().join("replies/pathway/sections")), offline)
}
fn call(
    phase: &str,
    env: &Environment,
    args: &[String],
    offline: bool,
    exit: i32,
) -> Result<Value, String> {
    let mut full = args.to_vec();
    full.extend(globals(env, offline));
    budgets::invoke(phase, env, full, exit)
}
fn continuation(value: &Value) -> Result<Option<Vec<String>>, String> {
    value
        .get("next")
        .and_then(Value::as_array)
        .ok_or("missing next array")?
        .iter()
        .find(|s| s.get("kind") == Some(&json!("page")))
        .map(|s| {
            serde_json::from_value(s.get("args").ok_or("missing page args")?.clone())
                .map_err(|e| e.to_string())
        })
        .transpose()
}
fn walk(genes: bool) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let kind = if genes { "genes" } else { "events" };
    let mut args = ["pathway", kind, "R-HSA-5673001", "--limit", "50"]
        .map(str::to_owned)
        .to_vec();
    let mut seen = BTreeSet::new();
    let mut offset = 0;
    let mut objects = 0;
    let mut integers = 0;
    let mut joins = BTreeSet::new();
    loop {
        let value = call("pathway_sections/walk/1", &env, &args, offset > 0, 0)?;
        check_steps(&value, genes)?;
        let rows = value
            .pointer("/result/items")
            .and_then(Value::as_array)
            .ok_or("missing rows")?;
        if value.pointer("/result/page/offset") != Some(&json!(offset)) {
            return Err("page gap".into());
        }
        let observed = observe(rows, genes, &mut seen, &mut joins)?;
        objects += observed.0;
        integers += observed.1;
        let Some(next) = continuation(&value)? else {
            break;
        };
        if offset == 0 {
            let forwarded = budgets::mcp(
                "runner/pathway_sections/walk/1",
                &env,
                &next,
                false,
                &globals(&env, true),
            )?;
            if forwarded.pointer("/result/page/offset") != Some(&json!(50)) {
                return Err("MCP continuation changed offset".into());
            }
        }
        args = next;
        offset += 50;
    }
    if genes {
        if seen.len() != 332 {
            return Err(format!("reference total changed: {}", seen.len()));
        }
        for (protein, gene) in [
            ("P15056", "HGNC:1097"),
            ("P01116-1", "HGNC:6407"),
            ("P01116-2", "HGNC:6407"),
            ("Q02750", "HGNC:6840"),
            ("P36507", "HGNC:6842"),
        ] {
            if !joins.contains(&(protein.into(), gene.into())) {
                return Err(format!("missing exact join {protein} {gene}"));
            }
        }
    } else if (seen.len(), objects, integers) != (81, 69, 12) {
        return Err(format!(
            "native event union changed: {} / {objects} / {integers}",
            seen.len()
        ));
    }
    Ok(())
}
fn observe(
    rows: &[Value],
    genes: bool,
    seen: &mut BTreeSet<String>,
    joins: &mut BTreeSet<(String, String)>,
) -> Result<(usize, usize), String> {
    let mut objects = 0;
    let mut integers = 0;
    for row in rows {
        let identity = if genes {
            let namespace = row
                .pointer("/reference/namespace")
                .and_then(Value::as_str)
                .ok_or("namespace")?;
            let id = row
                .pointer("/reference/id")
                .and_then(Value::as_str)
                .ok_or("reference")?;
            if let Some(gene) = row.get("gene").and_then(Value::as_str) {
                joins.insert((id.to_owned(), gene.to_owned()));
            }
            format!("{namespace}:{id}")
        } else {
            if row.get("kind") == Some(&json!("object")) {
                objects += 1;
            } else {
                integers += 1;
            }
            row.get("db_id").ok_or("database reference")?.to_string()
        };
        if !seen.insert(identity) {
            return Err("duplicate source member during walk".into());
        }
    }
    Ok((objects, integers))
}
fn check_steps(value: &Value, genes: bool) -> Check {
    let rows = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("rows")?;
    let steps = value.get("next").and_then(Value::as_array).ok_or("steps")?;
    if genes {
        check_gene_steps(value, rows, steps)
    } else {
        check_event_steps(value, rows, steps)
    }
}
fn unresolved(value: &Value, row: &Value) -> Check {
    let namespace = row
        .pointer("/reference/namespace")
        .and_then(Value::as_str)
        .ok_or("namespace")?;
    let id = row
        .pointer("/reference/id")
        .and_then(Value::as_str)
        .ok_or("reference")?;
    let reference = format!("{namespace}:{id}");
    let problems = value
        .get("problems")
        .and_then(Value::as_array)
        .ok_or("problems")?;
    if !problems
        .iter()
        .any(|p| p.get("record").and_then(Value::as_str) == Some(reference.as_str()))
    {
        return Err("unresolved reference omitted its problem".into());
    }
    Ok(())
}
fn check_gene_steps(value: &Value, rows: &[Value], steps: &[Value]) -> Check {
    let mut expected = Vec::new();
    for row in rows {
        if let Some(id) = row.get("gene").and_then(Value::as_str) {
            if !expected.contains(&id) {
                expected.push(id);
            }
        } else {
            unresolved(value, row)?;
        }
        if row.get("schema_class") == Some(&json!("ReferenceTherapeutic"))
            && (row.get("source_url").is_none() || row.get("gene").is_some())
        {
            return Err("therapeutic reference lost URL or became a gene".into());
        }
    }
    let actual: Vec<_> = steps
        .iter()
        .filter(|s| s.get("kind") == Some(&json!("pivot")))
        .map(|s| {
            s.pointer("/args/2")
                .and_then(Value::as_str)
                .ok_or("gene pivot")
        })
        .collect::<Result<_, _>>()?;
    if actual != expected {
        return Err("gene pivots changed source order or multiplicity".into());
    }
    if continuation(value)?.is_some()
        && steps.get(expected.len()).and_then(|s| s.get("kind")) != Some(&json!("page"))
    {
        return Err("gene page step precedes or interrupts pivots".into());
    }
    Ok(())
}
fn check_event_steps(value: &Value, rows: &[Value], steps: &[Value]) -> Check {
    if continuation(value)?.is_some()
        && steps.first().and_then(|s| s.get("kind")) != Some(&json!("page"))
    {
        return Err("event continuation does not precede selections".into());
    }
    for step in steps
        .iter()
        .filter(|s| s.get("kind") == Some(&json!("disambiguate")))
    {
        let id = step.pointer("/args/2").ok_or("event get identity")?;
        if !rows.iter().any(|r| {
            r.get("id") == Some(id)
                && r.get("schema_class")
                    .and_then(Value::as_str)
                    .is_some_and(|class| matches!(class, "Pathway" | "TopLevelPathway"))
        }) {
            return Err("non-pathway event generated a pathway get".into());
        }
    }
    if steps
        .iter()
        .filter(|s| s.get("kind") != Some(&json!("page")))
        .count()
        > 3
    {
        return Err("events exposed too many selections".into());
    }
    Ok(())
}
fn bindings() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["pathway", "events", "R-HSA-5673001"]
        .map(str::to_owned)
        .to_vec();
    let first = call("pathway_sections/bindings/1", &env, &args, false, 0)?;
    let next = continuation(&first)?.ok_or("missing continuation")?;
    let continued = call("pathway_sections/bindings/2", &env, &next, true, 0)?;
    if first.pointer("/result/page/pool/created") != continued.pointer("/result/page/pool/created")
        || first.pointer("/result/page/pool/expires")
            != continued.pointer("/result/page/pool/expires")
    {
        return Err("continuation renewed pool lifetime".into());
    }
    for (position, replacement) in [(1, "genes"), (2, "R-HSA-177929")] {
        let mut changed = next.clone();
        *changed.get_mut(position).ok_or("selector")? = replacement.into();
        let value = call("pathway_sections/bindings/3", &env, &changed, true, 3)?;
        if value.pointer("/error/code") != Some(&json!("pool.expired")) {
            return Err("foreign selector reused pool".into());
        }
    }
    let mut disabled = next.clone();
    disabled.extend(["--disable-sources".into(), "reactome".into()]);
    let value = call("pathway_sections/bindings/4", &env, &disabled, true, 4)?;
    if value.pointer("/sources/0/state") != Some(&json!("skipped")) {
        return Err("disabled relation restored pool".into());
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE pool SET expires=0", [])
        .map_err(|e| e.to_string())?;
    let expired = call("pathway_sections/bindings/5", &env, &next, true, 3)?;
    if expired.pointer("/error/code") != Some(&json!("pool.expired")) {
        return Err("expired relation refetched".into());
    }
    Ok(())
}
fn migration() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["pathway", "genes", "R-HSA-5673001"]
        .map(str::to_owned)
        .to_vec();
    call("pathway_sections/migration/1", &env, &args, false, 0)?;
    let path = env.root().join("case-cache/replay.sqlite");
    {
        let db = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        db.execute(
            "DELETE FROM bundle_key WHERE bundle='hgnc' AND kind=4 AND key LIKE 'uniprot:%'",
            [],
        )
        .map_err(|e| e.to_string())?;
        db.execute("UPDATE meta SET version=3 WHERE owner='biomcp-fetch'", [])
            .map_err(|e| e.to_string())?;
        db.execute("CREATE TABLE unrelated_owner(value TEXT)", [])
            .map_err(|e| e.to_string())?;
        db.execute("INSERT INTO unrelated_owner VALUES ('preserve')", [])
            .map_err(|e| e.to_string())?;
    }
    let value = call("pathway_sections/migration/2", &env, &args, true, 0)?;
    if value.pointer("/result/items/0/gene") != Some(&json!("HGNC:1097")) {
        return Err("retained HGNC index lost native join".into());
    }
    call("pathway_sections/migration/3", &env, &args, true, 0)?;
    let db = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
    let (version, preserved): (u32, String) = db
        .query_row(
            "SELECT version,(SELECT value FROM unrelated_owner) FROM meta WHERE owner='biomcp-fetch'",
            [],
            |r| Ok((r.get(0)?, r.get(1)?)),
        )
        .map_err(|e| e.to_string())?;
    if version != biomcp_fetch::VERSION {
        return Err("migration did not update the transport owner version".into());
    }
    if preserved != "preserve" {
        return Err("migration changed a foreign owner's data".into());
    }
    Ok(())
}
fn disclosure_modes() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["pathway", "genes", "R-HSA-5673001"]
        .map(str::to_owned)
        .to_vec();
    let compact = call("pathway_sections/disclosure_modes/1", &env, &args, false, 0)?;
    if compact.pointer("/result/items/0/physical_context_count") != Some(&json!(29))
        || compact.pointer("/result/items/0/participants").is_some()
    {
        return Err("default membership disclosed full contexts or omitted their count".into());
    }
    let steps = compact
        .get("next")
        .and_then(Value::as_array)
        .ok_or("steps")?;
    let details = steps
        .iter()
        .filter_map(|step| step.get("args"))
        .find(|args| {
            args.as_array()
                .is_some_and(|args| args.contains(&json!("details")))
        })
        .ok_or("missing explicit details step")?;
    let args: Vec<String> = serde_json::from_value(details.clone()).map_err(|e| e.to_string())?;
    let full = call("pathway_sections/disclosure_modes/2", &env, &args, true, 0)?;
    if full.pointer("/result/page/pool") != compact.pointer("/result/page/pool") {
        return Err("disclosure mode replaced or renewed the retained relation".into());
    }
    check_context_counts(&full)?;
    let next = continuation(&full)?.ok_or("missing details continuation")?;
    if !next.iter().any(|s| s == "details") {
        return Err("details mode disappeared from continuation".into());
    }
    let page = budgets::mcp(
        "runner/pathway_sections/disclosure_modes/1",
        &env,
        &next,
        false,
        &globals(&env, true),
    )?;
    check_context_counts(&page)?;
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE pool SET expires=0", [])
        .map_err(|e| e.to_string())?;
    let expired = call("pathway_sections/disclosure_modes/3", &env, &next, true, 3)?;
    let restart = expired
        .pointer("/next/0/args")
        .and_then(Value::as_array)
        .ok_or("details restart")?;
    if !restart.contains(&json!("details")) {
        return Err("restart lost details mode".into());
    }
    Ok(())
}
fn check_context_counts(value: &Value) -> Check {
    let rows = value
        .pointer("/result/items")
        .and_then(Value::as_array)
        .ok_or("details rows")?;
    for row in rows {
        let contexts = row
            .get("participants")
            .and_then(Value::as_array)
            .ok_or("full contexts")?;
        if row.get("physical_context_count").and_then(Value::as_u64) != Some(contexts.len() as u64)
        {
            return Err("details clipped or changed physical contexts".into());
        }
    }
    Ok(())
}
fn measurements() -> Check {
    for (args, max_bytes, max_tokens) in [
        (vec!["get", "pathway", "R-HSA-5673001", "genes"], 3061, 766),
        (vec!["get", "pathway", "R-HSA-5673001", "events"], 2541, 636),
        (vec!["get", "pathway", "R-HSA-5673001", "all"], 9820, 2456),
        (vec!["pathway", "genes", "R-HSA-5673001"], 2553, 639),
        (vec!["pathway", "events", "R-HSA-5673001"], 2034, 509),
        (
            vec!["pathway", "genes", "R-HSA-5673001", "details"],
            12155,
            3040,
        ),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let args: Vec<String> = args.into_iter().map(str::to_owned).collect();
        let cli = call("pathway_sections/measurements/1", &env, &args, false, 0)?;
        let mcp = budgets::mcp(
            "runner/pathway_sections/measurements/1",
            &env,
            &args,
            false,
            &globals(&env, false),
        )?;
        for (surface, value) in [("CLI", cli), ("MCP", mcp)] {
            super::schema::Registry::embedded()?.validate(&value, &args, true)?;
            let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
            let comparable = super::source_terms::size(&value, "")?;
            if comparable > max_bytes || comparable.div_ceil(4) > max_tokens {
                return Err(format!(
                    "{} exceeds its measured disclosure ceiling",
                    args.join(" ")
                ));
            }
            writeln!(
                std::io::stdout(),
                "pathway measure {} {surface}: full={full} comparable={comparable} tokens={}",
                args.join(" "),
                comparable.div_ceil(4)
            )
            .map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
