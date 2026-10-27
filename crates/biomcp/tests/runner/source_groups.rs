//! Feature source-controls: callers see group consequences before disabling sources.
use super::{configuration::invoke, launch::Environment};
use biomcp_adapters::registry;
use libtest_mimic::Trial;
use serde_json::Value;

pub(super) fn trials() -> Vec<Trial> {
    vec![Trial::test(
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
    for topic in ["sources", "config"] {
        let help = invoke(&env, vec!["help".into(), topic.into()], 0)?;
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
            let expected = registry::disabled(&[name.into()]).map_err(|e| e.to_string())?;
            let sources: Vec<_> = expected.iter().map(ToString::to_string).collect();
            if group["sources"] != serde_json::json!(sources) {
                return Err(format!("health has wrong {name} sources"));
            }
            for source in expected {
                let label = format!("{} ({source})", registry::display_name(source));
                if !description.contains(&label) {
                    return Err(format!("{topic} omits {name}: {label}"));
                }
            }
            let components: Vec<_> = registry::components()
                .filter(|c| c.terms.usage == usage)
                .map(|c| serde_json::json!({"source": c.source, "name": c.name}))
                .collect();
            if group["components"] != serde_json::json!(components) {
                return Err(format!("health has wrong {name} components"));
            }
            for component in registry::components().filter(|c| c.terms.usage == usage) {
                let label = format!("{}:{}", component.source, component.name);
                if !description.contains(&label) {
                    return Err(format!("{topic} omits {name} component {label}"));
                }
            }
        }
    }
    Ok(())
}
