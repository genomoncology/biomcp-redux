//! Schema-fed command help shared with MCP discovery.

use biomcp_types::{Help, HelpArgument, HelpCommand, HelpWord};
use serde_json::Value;
use std::{collections::BTreeMap, io};

pub(crate) fn dispatch(path: &[String]) -> io::Result<Help> {
    if let [category, setting] = path
        && category == "network"
    {
        return Ok(network(setting));
    }
    let commands = commands()?
        .into_iter()
        .filter(|c| path.is_empty() || c.command == path)
        .collect();
    Ok(Help { commands })
}
fn commands() -> io::Result<Vec<HelpCommand>> {
    let examples: BTreeMap<String, Vec<Vec<String>>> =
        serde_json::from_str(include_str!(concat!(env!("OUT_DIR"), "/examples.json")))?;
    let mut commands = Vec::new();
    for &(verb, entity, id) in crate::grammar::QUERIES {
        let schema: Value = serde_json::from_str(
            crate::schema::find(id).ok_or_else(|| io::Error::other("missing query schema"))?,
        )?;
        let properties = field(&schema, "properties")
            .as_object()
            .ok_or_else(|| io::Error::other("missing properties"))?;
        let arguments = properties
            .iter()
            .flat_map(|(name, node)| arguments(name, node, &schema))
            .collect();
        let sections = section_words(&schema);
        commands.push(HelpCommand {
            command: vec![verb.into(), entity.into()],
            description: text(&schema, "description"),
            arguments,
            section_rule: (!sections.is_empty()).then(|| {
                text(
                    field(field(&schema, "properties"), "section"),
                    "description",
                )
            }),
            sections,
            examples: examples
                .get(&format!("{verb} {entity}"))
                .cloned()
                .unwrap_or_default(),
        });
    }
    extra_commands(&mut commands, &examples);
    Ok(commands)
}
fn extra_commands(commands: &mut Vec<HelpCommand>, examples: &BTreeMap<String, Vec<Vec<String>>>) {
    for (command, description) in [
        (
            vec!["health"],
            "Report publisher terms and bounded source reachability; at most one page for each implemented source.",
        ),
        (
            vec!["sync", "[hgnc|clingen]"],
            "Synchronize a local source bundle.",
        ),
        (
            vec!["schema", "<artifact id>"],
            "Print a checked-in schema artifact.",
        ),
        (
            vec!["help", "[COMMAND...]"],
            "Print generated command help.",
        ),
        (
            vec!["key", "list"],
            "List supported credential names and their origin without values.",
        ),
        (
            vec!["key", "set", "<source>", "[--replace]"],
            "Register one credential from a no-echo prompt or one standard-input line. Use --config PATH to choose the configuration file.",
        ),
        (
            vec!["key", "remove", "<source>"],
            "Remove a stored credential while preserving other settings.",
        ),
        (
            vec!["serve"],
            "Serve MCP over stdio, or use --http [ADDR] for native HTTP clients. Non-loopback addresses require --allow-host HOST or http.allowed_hosts. Responses contain one JSON object; HTTP frames and active connections have fixed limits.",
        ),
    ] {
        commands.push(HelpCommand {
            command: command.into_iter().map(String::from).collect(),
            description: description.into(),
            arguments: Vec::new(),
            sections: Vec::new(),
            section_rule: None,
            examples: Vec::new(),
        });
    }
    bundle_commands(commands, examples);
    commands.extend(
        biomcp_fetch::SecretName::ALL
            .iter()
            .map(|name| HelpCommand {
                command: vec!["help".into(), "key".into(), name.source().into()],
                description: "Show credential registration and environment fallback names.".into(),
                arguments: Vec::new(),
                sections: Vec::new(),
                section_rule: None,
                examples: Vec::new(),
            }),
    );
}
fn bundle_commands(commands: &mut Vec<HelpCommand>, examples: &BTreeMap<String, Vec<Vec<String>>>) {
    for bundle in ["ema", "ddinter", "gtr"] {
        commands.push(HelpCommand {
            command: vec!["sync".into(), bundle.into()],
            description: "Replace the complete validated local bundle atomically. Reads use stored data and never synchronize. Failed replacement preserves the last usable bundle.".into(),
            arguments: Vec::new(),
            sections: Vec::new(),
            section_rule: None,
            examples: examples.get(&format!("sync {bundle}")).cloned().unwrap_or_default(),
        });
    }
}
fn arguments(name: &str, node: &Value, root: &Value) -> Vec<HelpArgument> {
    if let Some(flags) = node.get("x-cli-flags").and_then(Value::as_object) {
        return flags
            .keys()
            .map(|flag| {
                let mut value = argument(flag, node, root);
                value.value_type = "boolean".into();
                value.choices.clear();
                value
            })
            .collect();
    }
    vec![argument(name, node, root)]
}
fn argument(name: &str, node: &Value, root: &Value) -> HelpArgument {
    let position = field(node, "x-positional").as_u64();
    let resolved = resolve(node, root);
    let choices = if text(resolved, "type") == "array" {
        resolve(field(resolved, "items"), root)
    } else {
        resolved
    };
    HelpArgument {
        name: if position.is_some() {
            name.into()
        } else {
            format!("--{}", name.replace('_', "-"))
        },
        description: text(node, "description"),
        value_type: text(resolved, "type"),
        required: field(root, "required")
            .as_array()
            .is_some_and(|v| v.iter().any(|v| v == name)),
        position,
        minimum: field(node, "minimum")
            .as_u64()
            .or(field(node, "minItems").as_u64()),
        maximum: field(node, "maximum")
            .as_u64()
            .or(field(node, "maxItems").as_u64()),
        choices: field(choices, "enum")
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(|v| v.as_str().map(String::from))
            .collect(),
    }
}
fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    field(node, "$ref")
        .as_str()
        .and_then(|s| root.pointer(s.trim_start_matches('#')))
        .unwrap_or(node)
}
fn text(node: &Value, key: &str) -> String {
    node[key].as_str().unwrap_or_default().into()
}
pub(crate) fn tree() -> String {
    let Ok(commands) = commands() else {
        return "BioMCP command help unavailable".into();
    };
    let mut lines = vec!["BioMCP: one card or page. help COMMAND lists options and examples.".into()];
    for command in commands {
        let mut line = command.command.join(" ");
        // Detailed options belong to help COMMAND; retain search facets
        // required for discovering checked gene and disease searches.
        if command.command == ["search", "gene"] || command.command == ["search", "disease"] {
            for argument in &command.arguments {
                line.push_str(&format!(" [{}]", argument.name));
            }
        }
        if command.command.iter().any(|word| word == "adverse-event") {
            for argument in command.arguments.iter().filter(|arg| arg.position.is_none()) {
                line.push_str(&format!(" [{}", argument.name));
                if !argument.choices.is_empty() {
                    line.push_str(&format!(" {}", argument.choices.join("|")));
                }
                line.push(']');
            }
        }
        if !command.sections.is_empty() {
            line.push_str(" sections: ");
            line.push_str(
                &command
                    .sections
                    .iter()
                    .map(|s| s.word.as_str())
                    .collect::<Vec<_>>()
                    .join("|"),
            );
        }
        lines.push(line);
    }
    lines.push(format!(
        "schema discovery: {} artifacts",
        crate::schema::names().count()
    ));
    lines.join("\n")
}

fn field<'a>(node: &'a Value, key: &str) -> &'a Value {
    node.get(key).unwrap_or(&Value::Null)
}

fn section_words(schema: &Value) -> Vec<HelpWord> {
    let property = field(field(schema, "properties"), "section");
    let section = property
        .get("anyOf")
        .and_then(Value::as_array)
        .and_then(|variants| variants.iter().find(|v| v.get("$ref").is_some()))
        .or_else(|| property.get("items"))
        .unwrap_or(property);
    let section = resolve(section, schema);
    field(section, "enum")
        .as_array()
        .into_iter()
        .flatten()
        .enumerate()
        .filter_map(|(i, v)| {
            Some(HelpWord {
                word: v.as_str()?.into(),
                description: field(section, "x-enum-descriptions")
                    .get(i)
                    .unwrap_or(&Value::Null)
                    .as_str()?
                    .trim()
                    .into(),
            })
        })
        .collect::<Vec<_>>()
}

fn network(setting: &str) -> Help {
    let description = match setting {
        "ca_bundle" => {
            "Set ca_bundle in config.json to a PEM certificate file. These roots add to platform trust for every source. Keep certificate verification enabled."
        }
        _ => {
            "Set proxy in config.json to an HTTP or HTTPS proxy URL. Otherwise HTTPS_PROXY and HTTP_PROXY select routes for their schemes; NO_PROXY bypasses exact hosts/IPs, .domain, *.domain, optional ports, or *. An explicit proxy overrides proxy environment settings."
        }
    };
    Help {
        commands: vec![HelpCommand {
            command: vec!["help".into(), "network".into(), setting.into()],
            description: description.into(),
            arguments: Vec::new(),
            sections: Vec::new(),
            section_rule: None,
            examples: Vec::new(),
        }],
    }
}
