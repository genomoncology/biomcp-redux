//! Feature source-controls: synthetic delayed exchanges explain bounded source latency.
use super::{configuration::invoke, launch::Environment};
use libtest_mimic::Trial;
mod quota;
use serde_json::Value;
use std::{fs, io::Write, path::Path};
pub(super) fn trials() -> Vec<Trial> {
    [
        (
            "gwas",
            "gwas/deadline",
            vec!["search", "gwas", "--gene", "BRAF", "--limit", "3"],
            vec!["help", "search", "gwas"],
            "GWAS Catalog",
        ),
        (
            "vaers",
            "vaers/live-mmr",
            vec!["search", "adverse-event", "MMR", "--source", "vaers"],
            vec!["help", "search", "adverse-event"],
            "interactive form",
        ),
        (
            "interpro",
            "interpro/deadline",
            vec!["health"],
            vec!["help", "health"],
            "InterPro",
        ),
    ]
    .into_iter()
    .map(|(source, fixture, words, help, expected)| {
        Trial::test(
            format!("source-deadlines/{source}/reply-and-help-guidance"),
            move || check(source, fixture, &words, &help, expected).map_err(Into::into),
        )
    })
    .chain(std::iter::once(Trial::test(
        "source-deadlines/wonder/shared-quota-reply",
        || quota::run().map_err(Into::into),
    )))
    .collect()
}
fn check(
    source: &str,
    fixture: &str,
    words: &[&str],
    help: &[&str],
    expected: &str,
) -> Result<(), String> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let env = Environment::new().map_err(|e| e.to_string())?;
    let evidence = tempfile::Builder::new()
        .prefix(&format!("synthetic-{source}-deadline-"))
        .tempdir_in(super::network_stub::evidence_directory().map_err(|e| e.to_string())?)
        .map_err(|e| e.to_string())?
        .keep();
    let recording = evidence.join("recording");
    super::adverse_event_failures::mutate(
        &root.join("tests/replies").join(fixture),
        &recording,
        "timeout",
    )?;
    let mut args: Vec<String> = words.iter().map(|s| s.to_string()).collect();
    args.extend(["--replay".into(), recording.to_string_lossy().into_owned()]);
    if source == "interpro" {
        args.extend([
            "--disable-sources".into(),
            biomcp_adapters::registry::SOURCES
                .iter()
                .filter(|s| s.source.to_string() != source)
                .map(|s| s.source.to_string())
                .collect::<Vec<_>>()
                .join(","),
        ]);
    }
    let reply = invoke(&env, args, if source == "interpro" { 0 } else { 4 })?;
    fs::write(evidence.join("reply.json"), reply.to_string()).map_err(|e| e.to_string())?;
    writeln!(
        std::io::stdout(),
        "Synthetic {source} deadline evidence: {}",
        evidence.display()
    )
    .map_err(|e| e.to_string())?;
    let rows = if source == "interpro" {
        &reply["result"]["sources"]
    } else {
        &reply["sources"]
    };
    let row = rows
        .as_array()
        .ok_or_else(|| format!("missing rows: {reply}"))?
        .iter()
        .find(|r| r["source"] == source)
        .ok_or("missing source")?;
    if row["reason"] != "deadline_exceeded" || row["state"] != "failed" {
        return Err(format!("synthetic latency lost source reason: {row}"));
    }
    let notices = row.get("notices").unwrap_or(&Value::Null).to_string();
    if !notices.contains(expected)
        || !notices.contains("BioMCP")
        || !notices.contains("retry") && source != "vaers"
    {
        return Err(format!("missing deadline guidance: {row}"));
    }
    let help_reply = invoke(&env, help.iter().map(|s| s.to_string()).collect(), 0)?.to_string();
    fs::write(evidence.join("help.json"), &help_reply).map_err(|e| e.to_string())?;
    if !help_reply.contains(expected) || !help_reply.contains("BioMCP") {
        return Err(format!("missing source guidance in help: {help_reply}"));
    }

    Ok(())
}
