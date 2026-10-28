//! Feature source-controls: synthetic delayed exchanges explain bounded source latency.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
use serde_json::Value;
use std::{fs, path::Path};
pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test("source-deadlines/reply-and-help-guidance", || run().map_err(Into::into))]
}
fn run() -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (source, fixture, words, help, expected) in [
        ("gwas", "gwas/synthetic", vec!["search", "gwas", "--variant", "rs2981582", "--limit", "3"], vec!["help", "search", "gwas"], "GWAS Catalog"),
        ("vaers", "vaers/live-mmr", vec!["search", "adverse-event", "MMR", "--source", "vaers"], vec!["help", "search", "adverse-event"], "interactive form"),
        ("interpro", "protein/examples", vec!["health"], vec!["help", "health"], "InterPro"),
    ] {
        let env = Environment::new().map_err(|e| e.to_string())?;
        let recording = env.root().join("synthetic-deadline");
        super::adverse_event_failures::mutate(&root.join("tests/replies").join(fixture), &recording, "timeout")?;
        let mut args: Vec<String> = words.iter().map(|s| s.to_string()).collect();
        args.extend(["--replay".into(), recording.to_string_lossy().into_owned()]);
        if source == "interpro" {
            args.extend(["--disable-sources".into(), biomcp_adapters::registry::SOURCES.iter().filter(|s| s.source.to_string() != source).map(|s|s.source.to_string()).collect::<Vec<_>>().join(",")]);
        }
        let reply = invoke(&env, args, if source == "interpro" {0} else {4})?;
        let rows = if source == "interpro" { &reply["result"]["sources"] } else { &reply["sources"] };
        let row = rows.as_array().ok_or_else(||format!("missing rows: {reply}"))?.iter().find(|r|r["source"] == source).ok_or("missing source")?;
        if row["reason"] != "deadline_exceeded" || row["state"] != "failed" {
            return Err(format!("synthetic latency lost source reason: {row}"));
        }
        let notices = row.get("notices").unwrap_or(&Value::Null).to_string();
        if !notices.contains(expected) || !notices.contains("BioMCP") || !notices.contains("retry") && source != "vaers" {
            return Err(format!("missing deadline guidance: {row}"));
        }
        let help_reply = invoke(&env, help.iter().map(|s|s.to_string()).collect(), 0)?.to_string();
        if !help_reply.contains(expected) || !help_reply.contains("BioMCP") { return Err(format!("missing source guidance in help: {help_reply}")); }
        let _ = fs::metadata(&recording).map_err(|e|e.to_string())?;
    }
    Ok(())
}
