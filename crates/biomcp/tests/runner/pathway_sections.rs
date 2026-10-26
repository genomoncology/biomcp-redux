//! Feature pathway-reactome: retained source membership and event walks through CLI and MCP.
use super::{configuration::invoke, launch::{Environment, fixtures}, mcp};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{collections::BTreeSet, io::Write};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    vec![
        Trial::test("pathway-sections/native-reference-walk", || walk(true).map_err(Into::into)),
        Trial::test("pathway-sections/native-event-walk", || walk(false).map_err(Into::into)),
        Trial::test("pathway-sections/pool-binding-and-fixed-lifetime", || bindings().map_err(Into::into)),
        Trial::test("pathway-sections/warm-hgnc-uniprot-index-migration", || migration().map_err(Into::into)),
        Trial::test("pathway-sections/native-disclosure-measurements", || measurements().map_err(Into::into)),
    ]
}
fn globals(env: &Environment, offline: bool) -> Vec<String> {
    env.globals(Some(&fixtures().join("replies/pathway/sections")), offline)
}
fn call(env: &Environment, args: &[String], offline: bool, exit: i32) -> Result<Value, String> {
    let mut full = args.to_vec(); full.extend(globals(env, offline));
    invoke(env, full, exit)
}
fn continuation(value: &Value) -> Result<Option<Vec<String>>, String> {
    value.get("next").and_then(Value::as_array).ok_or("missing next array")?.iter()
        .find(|s| s.get("kind") == Some(&json!("page")))
        .map(|s| serde_json::from_value(s.get("args").ok_or("missing page args")?.clone())
            .map_err(|e| e.to_string())).transpose()
}
fn walk(genes: bool) -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let kind = if genes { "genes" } else { "events" };
    let mut args = ["pathway", kind, "R-HSA-5673001", "--limit", "50"].map(str::to_owned).to_vec();
    let mut seen = BTreeSet::new(); let mut offset = 0; let mut objects = 0; let mut integers = 0;
    let mut joins = BTreeSet::new();
    loop {
        let value = call(&env, &args, offset > 0, 0)?;
        let rows = value.pointer("/result/items").and_then(Value::as_array).ok_or("missing rows")?;
        if value.pointer("/result/page/offset") != Some(&json!(offset)) { return Err("page gap".into()); }
        for row in rows {
            let identity = if genes {
                let namespace = row.pointer("/reference/namespace").and_then(Value::as_str).ok_or("namespace")?;
                let id = row.pointer("/reference/value").and_then(Value::as_str).ok_or("reference")?;
                if let Some(gene) = row.get("gene").and_then(Value::as_str) { joins.insert((id.to_owned(), gene.to_owned())); }
                format!("{namespace}:{id}")
            } else {
                if row.get("kind") == Some(&json!("object")) { objects += 1; } else { integers += 1; }
                row.get("db_id").ok_or("database reference")?.to_string()
            };
            if !seen.insert(identity) { return Err("duplicate source member during walk".into()); }
        }
        let Some(next) = continuation(&value)? else { break; };
        if offset == 0 {
            let forwarded = mcp::call_with_globals(&env, &next, false, &globals(&env, true))?;
            if forwarded.pointer("/result/page/offset") != Some(&json!(50)) { return Err("MCP continuation changed offset".into()); }
        }
        args = next; offset += 50;
    }
    if genes {
        if seen.len() != 332 { return Err(format!("reference total changed: {}", seen.len())); }
        for (protein, gene) in [("P15056", "HGNC:1097"), ("P01116-1", "HGNC:6407"),
            ("P01116-2", "HGNC:6407"), ("Q02750", "HGNC:6840"), ("P36507", "HGNC:6842")] {
            if !joins.contains(&(protein.into(), gene.into())) { return Err(format!("missing exact join {protein} {gene}")); }
        }
    } else if (seen.len(), objects, integers) != (81, 69, 12) {
        return Err(format!("native event union changed: {} / {objects} / {integers}", seen.len()));
    }
    Ok(())
}
fn bindings() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["pathway", "events", "R-HSA-5673001"].map(str::to_owned).to_vec();
    let first = call(&env, &args, false, 0)?;
    let next = continuation(&first)?.ok_or("missing continuation")?;
    let continued = call(&env, &next, true, 0)?;
    if first.pointer("/result/page/pool/created") != continued.pointer("/result/page/pool/created")
        || first.pointer("/result/page/pool/expires") != continued.pointer("/result/page/pool/expires") {
        return Err("continuation renewed pool lifetime".into());
    }
    for (position, replacement) in [(1, "genes"), (2, "R-HSA-177929")] {
        let mut changed = next.clone(); *changed.get_mut(position).ok_or("selector")? = replacement.into();
        let value = call(&env, &changed, true, 3)?;
        if value.pointer("/error/code") != Some(&json!("pool.expired")) { return Err("foreign selector reused pool".into()); }
    }
    let mut disabled = next.clone(); disabled.extend(["--disable-sources".into(), "reactome".into()]);
    let value = call(&env, &disabled, true, 4)?;
    if value.pointer("/sources/0/state") != Some(&json!("skipped")) { return Err("disabled relation restored pool".into()); }
    let db = rusqlite::Connection::open(env.root().join("case-cache/replay.sqlite")).map_err(|e| e.to_string())?;
    db.execute("UPDATE pool SET expires=0", []).map_err(|e| e.to_string())?;
    let expired = call(&env, &next, true, 3)?;
    if expired.pointer("/error/code") != Some(&json!("pool.expired")) { return Err("expired relation refetched".into()); }
    Ok(())
}
fn migration() -> Check {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let args = ["pathway", "genes", "R-HSA-5673001"].map(str::to_owned).to_vec();
    call(&env, &args, false, 0)?;
    let path = env.root().join("case-cache/replay.sqlite");
    {
        let db = rusqlite::Connection::open(&path).map_err(|e| e.to_string())?;
        db.execute("DELETE FROM bundle_key WHERE bundle='hgnc' AND kind=4 AND key LIKE 'uniprot:%'", []).map_err(|e| e.to_string())?;
        db.execute("UPDATE meta SET version=3 WHERE owner='biomcp-fetch'", []).map_err(|e| e.to_string())?;
        db.execute("CREATE TABLE unrelated_owner(value TEXT)", []).map_err(|e| e.to_string())?;
        db.execute("INSERT INTO unrelated_owner VALUES ('preserve')", []).map_err(|e| e.to_string())?;
    }
    let value = call(&env, &args, true, 0)?;
    if value.pointer("/result/items/0/gene") != Some(&json!("HGNC:1097")) { return Err("retained HGNC index lost native join".into()); }
    call(&env, &args, true, 0)?;
    let db = rusqlite::Connection::open(path).map_err(|e| e.to_string())?;
    let (version, preserved): (u32, String) = db.query_row("SELECT version,(SELECT value FROM unrelated_owner) FROM meta WHERE owner='biomcp-fetch'", [], |r| Ok((r.get(0)?,r.get(1)?))).map_err(|e| e.to_string())?;
    if version != 4 || preserved != "preserve" { return Err("migration changed a foreign owner or version".into()); }
    Ok(())
}
fn measurements() -> Check {
    for args in [vec!["get", "pathway", "R-HSA-5673001", "genes"],
        vec!["get", "pathway", "R-HSA-5673001", "events"],
        vec!["get", "pathway", "R-HSA-5673001", "all"],
        vec!["pathway", "genes", "R-HSA-5673001"],
        vec!["pathway", "events", "R-HSA-5673001"]] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let args: Vec<String> = args.into_iter().map(str::to_owned).collect();
        let cli = call(&env, &args, false, 0)?;
        let mcp = mcp::call_with_globals(&env, &args, false, &globals(&env, false))?;
        for (surface, value) in [("CLI", cli), ("MCP", mcp)] {
            super::schema::Registry::embedded()?.validate(&value, &args, true)?;
            let full = serde_json::to_vec(&value).map_err(|e| e.to_string())?.len();
            let comparable = super::source_terms::size(&value, "")?;
            writeln!(std::io::stdout(), "pathway measure {} {surface}: full={full} comparable={comparable} tokens={}", args.join(" "), comparable.div_ceil(4)).map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}
