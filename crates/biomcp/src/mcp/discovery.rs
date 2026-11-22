//! Opt-in tool discovery over the same filtered catalog as ordinary typed listing.
use super::{protocol, typed};
use crate::command::Context;
use serde_json::{Value, json};
use std::collections::BTreeSet;

pub(super) const COMPACT_KEY: &str = "biomcp/compactCatalog";
const NAME: &str = "search_tools";

pub(super) fn requested(params: &Value) -> bool {
    params["_meta"][COMPACT_KEY].as_bool() == Some(true)
}

pub(super) fn called(params: &Value) -> bool {
    params["name"].as_str() == Some(NAME)
}

pub(super) fn tool() -> Value {
    json!({
        "name":NAME,
        "description":"Find typed tools by task (query), or load one full definition (name). Call discovered tools by name.",
        "inputSchema":{
            "type":"object",
            "properties":{"query":{"type":"string","minLength":1},"name":{"type":"string","minLength":1}},
            "oneOf":[{"required":["query"]},{"required":["name"]}],
            "additionalProperties":false
        },
        "outputSchema":{"type":"object","required":["tools"],"properties":{
            "tools":{"type":"array","items":{"type":"object","required":["name","description"],
                "properties":{"name":{"type":"string"},"description":{"type":"string"}}}}
        }},
        "annotations":{"readOnlyHint":true,"destructiveHint":false,"openWorldHint":false}
    })
}

pub(super) fn call(
    id: Value,
    params: &Value,
    context: Context<'_>,
    read_only: bool,
    modern: bool,
) -> Value {
    let Some(args) = params["arguments"]
        .as_object()
        .filter(|args| args.len() == 1)
    else {
        return protocol::error(id, -32602, "Invalid params");
    };
    let result = match (
        args.get("query").and_then(Value::as_str),
        args.get("name").and_then(Value::as_str),
    ) {
        (Some(query), _) if !query.trim().is_empty() => {
            typed::discovery_tools(context, read_only, false).map(|tools| search(&tools, query))
        }
        (_, Some(name)) if !name.trim().is_empty() => {
            typed::discovery_tools(context, read_only, true).map(|tools| {
                tools
                    .into_iter()
                    .filter(|tool| tool["name"] == name)
                    .collect()
            })
        }
        _ => return protocol::error(id, -32602, "Invalid params"),
    };
    let Ok(tools) = result else {
        return protocol::error(id, -32603, "Output schema unavailable");
    };
    protocol::discovery_result(id, json!({"tools":tools}), modern, context)
}

fn search(tools: &[Value], query: &str) -> Vec<Value> {
    let terms = words(query).collect::<BTreeSet<_>>();
    let mut matches = tools
        .iter()
        .filter_map(|tool| {
            let name = tool["name"].as_str()?;
            let description = tool["description"].as_str()?.split(". ").next()?;
            let name_terms = words(name).collect::<BTreeSet<_>>();
            let description_terms = words(description).collect::<BTreeSet<_>>();
            let fields = tool
                .pointer("/inputSchema/properties")
                .and_then(Value::as_object);
            let field_terms = fields
                .into_iter()
                .flat_map(|actions| actions.values())
                .filter_map(|action| action.get("properties").and_then(Value::as_object))
                .flat_map(|fields| fields.keys())
                .flat_map(|field| words(field))
                .collect::<BTreeSet<_>>();
            let score = 3 * terms.intersection(&name_terms).count()
                + 2 * terms.intersection(&description_terms).count()
                + terms.intersection(&field_terms).count();
            (score > 0).then_some((score, name, description))
        })
        .collect::<Vec<_>>();
    // Stable sort keeps the catalog order for equally relevant tools.
    matches.sort_by_key(|(score, _, _)| std::cmp::Reverse(*score));
    matches
        .into_iter()
        .take(5)
        .map(|(_, name, description)| {
            json!({"name":name,"description":description.chars().take(200).collect::<String>()})
        })
        .collect()
}

fn words(text: &str) -> impl Iterator<Item = String> {
    text.split(|c: char| !c.is_alphanumeric())
        .filter(|word| word.chars().count() >= 3)
        .map(str::to_lowercase)
        .filter(|word| !matches!(word.as_str(), "the" | "and" | "for" | "with" | "near"))
        .map(|word| word.strip_suffix('s').unwrap_or(&word).to_owned())
}
