//! Feature pathway-reactome: retained source membership and event walks through CLI and MCP.
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
    mcp,
};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{collections::BTreeSet, io::Write};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("pathway-sections/native-reference-walk", || {
            walk(true).map_err(Into::into)
        }),
        Trial::test("pathway-sections/native-event-walk", || {
            walk(false).map_err(Into::into)
        }),
        Trial::test("pathway-sections/pool-binding-and-fixed-lifetime", || {
            bindings().map_err(Into::into)
        }),
        Trial::test("pathway-sections/warm-hgnc-uniprot-index-migration", || {
            migration().map_err(Into::into)
        }),
        Trial::test("pathway-sections/native-disclosure-measurements", || {
            measurements().map_err(Into::into)
        }),
    ]
}
fn globals(env: &Environment, offline: bool) -> Vec<String> {
    env.globals(Some(&fixtures().join("replies/pathway/sections")), offline)
}
fn call(env: &Environment, args: &[String], offline: bool, exit: i32) -> Result<Value, String> {
    let mut full = args.to_vec();
    full.extend(globals(env, offline));
    invoke(env, full, exit)
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
        let value = call(&env, &args, offset > 0, 0)?;
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
            let forwarded = mcp::call_with_globals(&env, &next, false, &globals(&env, true))?;
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
                .pointer("/reference/value")
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
    let rows = value.pointer("/result/items").and_then(Value::as_array).ok_or("rows")?;
    let steps = value.get("next").and_then(Value::as_array).ok_or("steps")?;
    if genes { check_gene_steps(value, rows, steps) } else { check_event_steps(value, rows, steps) }
}
fn unresolved(value: &Value, row: &Value) -> Check {
    let namespace = row.pointer("/reference/namespace").and_then(Value::as_str).ok_or("namespace")?;
    let id = row.pointer("/reference/value").and_then(Value::as_str).ok_or("reference")?;
    let reference = format!("{namespace}:{id}");
    let problems = value.get("problems").and_then(Value::as_array).ok_or("problems")?;
    if !problems.iter().any(|p| p.get("record").and_then(Value::as_str) == Some(reference.as_str())) {
        return Err("unresolved reference omitted its problem".into());
    }
    Ok(())
}
fn check_gene_steps(value: &Value, rows: &[Value], steps: &[Value]) -> Check {
    let mut expected = Vec::new();
    for row in rows {
        if let Some(id) = row.get("gene").and_then(Value::as_str) {
            if !expected.contains(&id) { expected.push(id); }
        } else { unresolved(value, row)?; }
        if row.get("schema_class") == Some(&json!("ReferenceTherapeutic"))
            && (row.get("source_url").is_none() || row.get("gene").is_some()) {
            return Err("therapeutic reference lost URL or became a gene".into());
        }
    }
    let actual: Vec<_> = steps.iter().filter(|s| s.get("kind") == Some(&json!("pivot")))
        .map(|s| s.pointer("/args/2").and_then(Value::as_str).ok_or("gene pivot"))
        .collect::<Result<_,_>>()?;
    if actual != expected { return Err("gene pivots changed source order or multiplicity".into()); }
    if continuation(value)?.is_some() && steps.get(expected.len()).and_then(|s| s.get("kind")) != Some(&json!("page")) {
        return Err("gene page step precedes or interrupts pivots".into());
    }
    Ok(())
}
fn check_event_steps(value: &Value, rows: &[Value], steps: &[Value]) -> Check {
    if continuation(value)?.is_some() && steps.first().and_then(|s| s.get("kind")) != Some(&json!("page")) {
        return Err("event continuation does not precede selections".into());
    }
    for step in steps.iter().filter(|s| s.get("kind") == Some(&json!("disambiguate"))) {
        let id = step.pointer("/args/2").ok_or("event get identity")?;
        if !rows.iter().any(|r| r.get("id") == Some(id) && r.get("schema_class").and_then(Value::as_str)
            .is_some_and(|class| matches!(class, "Pathway" | "TopLevelPathway"))) {
            return Err("non-pathway event generated a pathway get".into());
        }
    }
    if steps.iter().filter(|s| s.get("kind") != Some(&json!("page"))).count() > 3 {
        return Err("events exposed too many selections".into());
    }
    Ok(())
}
fn bindings() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["pathway", "events", "R-HSA-5673001"]
        .map(str::to_owned)
        .to_vec();
    let first = call(&env, &args, false, 0)?;
    let next = continuation(&first)?.ok_or("missing continuation")?;
    let continued = call(&env, &next, true, 0)?;
    if first.pointer("/result/page/pool/created") != continued.pointer("/result/page/pool/created")
        || first.pointer("/result/page/pool/expires")
            != continued.pointer("/result/page/pool/expires")
    {
        return Err("continuation renewed pool lifetime".into());
    }
    for (position, replacement) in [(1, "genes"), (2, "R-HSA-177929")] {
        let mut changed = next.clone();
        *changed.get_mut(position).ok_or("selector")? = replacement.into();
        let value = call(&env, &changed, true, 3)?;
        if value.pointer("/error/code") != Some(&json!("pool.expired")) {
            return Err("foreign selector reused pool".into());
        }
    }
    let mut disabled = next.clone();
    disabled.extend(["--disable-sources".into(), "reactome".into()]);
    let value = call(&env, &disabled, true, 4)?;
    if value.pointer("/sources/0/state") != Some(&json!("skipped")) {
        return Err("disabled relation restored pool".into());
    }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite"))
        .map_err(|e| e.to_string())?;
    db.execute("UPDATE pool SET expires=0", [])
        .map_err(|e| e.to_string())?;
    let expired = call(&env, &next, true, 3)?;
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
    call(&env, &args, false, 0)?;
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
    let value = call(&env, &args, true, 0)?;
    if value.pointer("/result/items/0/gene") != Some(&json!("HGNC:1097")) {
        return Err("retained HGNC index lost native join".into());
    }
    call(&env, &args, true, 0)?;
    let db = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
    let (version, preserved): (u32, String) = db.query_row("SELECT version,(SELECT value FROM unrelated_owner) FROM meta WHERE owner='biomcp-fetch'", [], |r| Ok((r.get(0)?,r.get(1)?))).map_err(|e| e.to_string())?;
    if version != 4 || preserved != "preserve" {
        return Err("migration changed a foreign owner or version".into());
    }
    Ok(())
}
fn measurements() -> Check {
    for (args, max_bytes, max_tokens) in [
        (vec!["get", "pathway", "R-HSA-5673001", "genes"], 12469, 3118),
        (vec!["get", "pathway", "R-HSA-5673001", "events"], 2541, 636),
        (vec!["get", "pathway", "R-HSA-5673001", "all"], 19228, 4807),
        (vec!["pathway", "genes", "R-HSA-5673001"], 11962, 2992),
        (vec!["pathway", "events", "R-HSA-5673001"], 2034, 509),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let args: Vec<String> = args.into_iter().map(str::to_owned).collect();
        let cli = call(&env, &args, false, 0)?;
        let mcp = mcp::call_with_globals(&env, &args, false, &globals(&env, false))?;
        for (surface, value) in [("CLI", cli), ("MCP", mcp)] {
            super::schema::Registry::embedded()?.validate(&value, &args, true)?;
            let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
            let comparable = super::source_terms::size(&value, "")?;
            if comparable > max_bytes || comparable.div_ceil(4) > max_tokens {
                return Err(format!("{} exceeds its measured disclosure ceiling", args.join(" ")));
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
