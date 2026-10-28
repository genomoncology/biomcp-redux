//! Feature source-controls: callers see group consequences before disabling sources.
use super::{configuration::invoke, launch::Environment};
use crate::runner::transcript::trial as transcript_trial;
use biomcp_adapters::registry;
use libtest_mimic::Trial;
use serde_json::Value;

pub(super) fn trials() -> Vec<Trial> {
    vec![transcript_trial(
        "sources/group-consequences-help-and-health",
        || check().map_err(Into::into),
    )]
}
fn check() -> Result<(), String> {
    let env = Environment::new().map_err(|e| e.to_string())?;
    let health = invoke(&env, vec!["health".into(), "--offline".into()], 0)?;
    let groups = health
        .pointer("/result/disable_groups")
        .and_then(Value::as_array)
        .ok_or("health omitted disable groups")?;
    measure(&invoke(&env, vec!["help".into()], 0)?, "all")?;
    for topic in ["sources", "config"] {
        let help = invoke(&env, vec!["help".into(), topic.into()], 0)?;
        measure(&help, topic)?;
        let commands = help
            .pointer("/result/commands")
            .and_then(Value::as_array)
            .ok_or("help commands missing")?;
        let description = commands
            .first()
            .and_then(|c| c["description"].as_str())
            .ok_or("source settings help missing")?;
        if !description.contains("each publisher's terms need checking") {
            return Err("restricted explanation missing".into());
        }
        for name in ["open", "noncommercial", "restricted"] {
            let group = groups
                .iter()
                .find(|g| g["group"] == name)
                .ok_or("missing group")?;
            let usage = registry::group(name).ok_or("unknown group")?;
            let expected: std::collections::BTreeSet<_> = registry::SOURCES
                .iter()
                .filter(|s| {
                    s.terms.usage == usage && !registry::components().any(|c| c.source == s.source)
                })
                .map(|s| s.source)
                .collect();
            let sources: Vec<_> = expected.iter().map(ToString::to_string).collect();
            if group["sources"] != serde_json::json!(sources) {
                return Err(format!("health has wrong {name} sources"));
            }
            let labels = expected
                .into_iter()
                .map(|source| format!("{} ({source})", registry::display_name(source)))
                .collect::<Vec<_>>()
                .join(", ");
            let components: Vec<_> = registry::components()
                .filter(|c| c.terms.usage == usage)
                .map(|c| serde_json::json!({"source": c.source, "name": c.name}))
                .collect();
            if group["components"] != serde_json::json!(components) {
                return Err(format!("health has wrong {name} components"));
            }
            let fields = registry::components()
                .filter(|c| c.terms.usage == usage)
                .map(|c| format!("{}:{}", c.source, c.name))
                .collect::<Vec<_>>()
                .join(", ");
            if !description.contains(&format!(
                "{name} turns off sources: {labels}. Carried fields: {fields}."
            )) {
                return Err(format!("{topic} has wrong {name} consequences"));
            }
        }
    }
    Ok(())
}

fn measure(help: &Value, topic: &str) -> Result<(), String> {
    let bytes = serde_json::to_vec(help).map_err(|e| e.to_string())?.len() + 1;
    if bytes > super::CLI_HELP_BYTES {
        return Err(format!("{topic} help exceeded CLI_HELP_BYTES: {bytes}"));
    }
    use std::io::Write;
    writeln!(std::io::stderr(), "SOURCE_GROUP_HELP_BYTES {topic} {bytes}")
        .map_err(|e| e.to_string())
}
