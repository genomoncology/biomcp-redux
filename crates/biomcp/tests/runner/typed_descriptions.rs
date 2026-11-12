//! Installed discovery uses command-owned summaries, primary operands and section publishers.
use crate::runner::{
    launch::Environment,
    mcp::{exchange_with_globals, request},
};
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(super) fn check(env: &Environment) -> Result<(), String> {
    let entities = catalog(env, false)?;
    let tools = crate::runner::protocol::typed_catalog::commands(&entities);
    standalone(&entities)?;
    standalone(&tools)?;
    examples(&entities, true)?;
    let hosted = catalog(env, true)?;
    standalone(&hosted)?;
    standalone(&crate::runner::protocol::typed_catalog::commands(&hosted))?;
    examples(&hosted, false)?;
    faers_drug(&tools)?;
    for (verb, entity, id) in biomcp_types::command_paths() {
        let name = format!("{verb}_{entity}").replace('-', "_");
        let tool = tools
            .iter()
            .find(|tool| tool["name"] == name)
            .ok_or_else(|| format!("{name}: no advertised action"))?;
        let description = tool
            .get("description")
            .and_then(Value::as_str)
            .ok_or("description")?;
        // The tool map's ruling: citations says it lists citing papers, not counts.
        let description = if name == "article_citations" {
            description
                .strip_suffix(" Lists citing papers, not counts.")
                .ok_or_else(|| format!("{name}: citations note missing: {description}"))?
        } else {
            description
        };
        let summary = biomcp_types::COMMANDS
            .iter()
            .find(|row| row.verb == verb && row.entity == entity)
            .ok_or("registry summary")?
            .summary;
        if summary != summary.trim()
            || summary.is_empty()
            || summary.lines().count() != 1
            || !description.starts_with(summary)
            || description.chars().count() > 300
        {
            return Err(format!(
                "{name}: command summary or budget differs: {description}"
            ));
        }
        let schema = cli(env, &["schema".into(), id.into()])?;
        complete_words(&schema, summary, description, &name)?;
        for (key, property) in tool
            .pointer("/inputSchema/properties")
            .and_then(Value::as_object)
            .ok_or("tool properties")?
        {
            let description = property["description"]
                .as_str()
                .ok_or("parameter description")?;
            complete_words(&schema, summary, description, &format!("{name}.{key}"))?;
        }
        publishers(verb, entity, &schema, description)?;
    }
    spot_words(&tools)?;
    let help = cli(env, &["help".into(), "get".into(), "drug".into()])?;
    let description = help
        .pointer("/result/commands/0/description")
        .and_then(Value::as_str)
        .ok_or("drug help")?;
    let summary = biomcp_types::COMMANDS
        .iter()
        .find(|row| row.verb == "get" && row.entity == "drug")
        .ok_or("drug registry")?
        .summary
        .trim();
    if !description.starts_with(summary) {
        return Err("CLI help lost the command summary".into());
    }
    Ok(())
}
fn spot_words(tools: &[Value]) -> Result<(), String> {
    for (name, words) in [
        ("get_drug", ["drug", "medicine"]),
        ("search_adverse_event", ["adverse", "events"]),
        ("search_variant", ["genetic", "variants"]),
    ] {
        let description = tools
            .iter()
            .find(|t| t.get("name") == Some(&json!(name)))
            .and_then(|t| t.get("description"))
            .and_then(Value::as_str)
            .ok_or("spot description")?;
        if !words.into_iter().all(|word| description.contains(word)) {
            return Err(format!("{name}: missing search words"));
        }
    }
    Ok(())
}
fn faers_drug(tools: &[Value]) -> Result<(), String> {
    let description = tools
        .iter()
        .find(|tool| tool.get("name") == Some(&json!("search_adverse_event")))
        .and_then(|tool| tool.pointer("/inputSchema/properties/drug/description"))
        .and_then(Value::as_str)
        .ok_or("typed FAERS drug description")?;
    if description != "FAERS drug names; OR." {
        return Err(format!(
            "typed FAERS drug lost its owning description: {description}"
        ));
    }
    Ok(())
}
fn cli(env: &Environment, args: &[String]) -> Result<Value, String> {
    let output = env
        .command(env!("CARGO_BIN_EXE_biomcp"), args)
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err("discovery command failed".into());
    }
    serde_json::from_slice(&output.stdout).map_err(|e| e.to_string())
}
fn publishers(verb: &str, entity: &str, schema: &Value, description: &str) -> Result<(), String> {
    let mut sections = BTreeSet::new();
    for key in ["section", "sections"] {
        if let Some(node) = schema.get("properties").and_then(|p| p.get(key)) {
            words(node, schema, &mut sections);
        }
    }
    for section in sections.iter().filter(|section| section.as_str() != "all") {
        let sources =
            biomcp_types::command_section_sources(verb, entity, section, biomcp_types::Source::ALL)
                .unwrap_or_default();
        if !description.contains(section)
            || sources
                .iter()
                .any(|source| !description.contains(&source.to_string()))
        {
            return Err(format!(
                "{verb} {entity}: {section} publisher missing: {description}"
            ));
        }
    }
    Ok(())
}
fn words(node: &Value, root: &Value, out: &mut BTreeSet<String>) {
    if let Some(reference) = node.get("$ref").and_then(Value::as_str)
        && let Some(target) = root.pointer(reference.trim_start_matches('#'))
    {
        words(target, root, out);
    }
    out.extend(
        node.get("enum")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned),
    );
    if let Some(items) = node.get("items") {
        words(items, root, out);
    }
    for key in ["anyOf", "oneOf"] {
        for node in node
            .get(key)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            words(node, root, out);
        }
    }
}

fn standalone(tools: &[Value]) -> Result<(), String> {
    let mut failures = Vec::new();
    for tool in tools {
        let name = tool["name"].as_str().ok_or("tool name")?;
        let descriptions = std::iter::once((name.to_owned(), &tool["description"])).chain(
            tool.pointer("/inputSchema/properties")
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
                // The chosen-fields list is bare by design; search help names the fields.
                .filter(|(key, _)| *key != "fields")
                .map(|(key, property)| (format!("{name}.{key}"), &property["description"])),
        );
        for (path, value) in descriptions {
            let text = value.as_str().unwrap_or_default();
            if text.is_empty() || text.contains("; help ") {
                failures.push(format!("{path}: incomplete description: {text}"));
            }
        }
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures.join("\n"))
    }
}

// A model copies a description's example call, so each passes its tool's own schema, and
// each local one is a step of a case in tests/cases/mcp/typed, which sends exactly that
// call and reads its reply. Every step of those cases passes the advertised schema too,
// as a client that checks arguments before sending would require.
fn examples(tools: &[Value], local: bool) -> Result<(), String> {
    let mut paths = Vec::new();
    let directory = crate::runner::launch::fixtures().join("cases/mcp/typed");
    crate::runner::case_data::collect(&directory, &mut paths).map_err(|e| e.to_string())?;
    let mut sent = Vec::new();
    for path in &paths {
        let case = crate::runner::case_data::read_case(path)?;
        for (index, step) in case.steps.iter().enumerate() {
            let Some(call) = step.mcp_call.as_ref() else {
                continue;
            };
            let tool = tools.iter().find(|tool| tool["name"] == call.name.as_str());
            if let Some(tool) = tool.filter(|_| local) {
                jsonschema::validator_for(&tool["inputSchema"])
                    .map_err(|e| e.to_string())?
                    .validate(&call.arguments)
                    .map_err(|e| format!("{} step {}: {e}", path.display(), index + 1))?;
            }
            sent.push((call.name.clone(), call.arguments.clone()));
        }
    }
    let mut found = 0;
    for tool in tools {
        let text = tool["description"].as_str().unwrap_or_default();
        let name = tool["name"].as_str().unwrap_or_default();
        for (start, _) in text.match_indices("{\"action\":\"") {
            let example = serde_json::Deserializer::from_str(&text[start..])
                .into_iter::<Value>()
                .next()
                .and_then(Result::ok)
                .ok_or_else(|| format!("{name}: example is not JSON"))?;
            jsonschema::validator_for(&tool["inputSchema"])
                .map_err(|e| e.to_string())?
                .validate(&example)
                .map_err(|e| format!("{name}: example {example} fails its schema: {e}"))?;
            if local && !sent.contains(&(name.to_owned(), example.clone())) {
                return Err(format!("{name}: no case sends the example {example}"));
            }
            found += 1;
        }
    }
    if found == 0 {
        return Err("typed catalog advertises no example call".into());
    }
    Ok(())
}

// A truncated operand creates a word that is absent from the owning schema and registry.
fn complete_words(
    schema: &Value,
    summary: &str,
    description: &str,
    name: &str,
) -> Result<(), String> {
    let sources = biomcp_types::Source::ALL
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(" ");
    let declared = format!("{schema} {summary} {sources} IDs Sections Required");
    let vocabulary = declared
        .split(|c: char| !c.is_alphanumeric())
        .map(str::to_lowercase)
        .collect::<BTreeSet<_>>();
    let unknown = description
        .split(|c: char| !c.is_alphanumeric())
        .filter(|word| !word.is_empty() && !vocabulary.contains(&word.to_lowercase()))
        .collect::<Vec<_>>();
    if unknown.is_empty() {
        Ok(())
    } else {
        Err(format!(
            "{name}: description contains words absent from its source: {unknown:?}: {description}"
        ))
    }
}

fn catalog(env: &Environment, hosted: bool) -> Result<Vec<Value>, String> {
    let mut globals = vec!["--offline".into()];
    if hosted {
        globals.push("--hosted".into());
    }
    let replies = exchange_with_globals(
        env,
        &[request(
            json!(if hosted { 2 } else { 1 }),
            "tools/list",
            json!({}),
        )],
        &globals,
    )?;
    replies
        .first()
        .and_then(|reply| reply.pointer("/result/tools"))
        .and_then(Value::as_array)
        .cloned()
        .ok_or_else(|| "tools".into())
}
