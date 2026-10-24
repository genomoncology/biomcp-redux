//! Schema-fed command help shared with MCP discovery.
use biodata::{Help, HelpArgument, HelpCommand, HelpWord};
use serde_json::Value;
use std::{collections::BTreeMap, io};

pub(crate) fn dispatch(path: &[String]) -> io::Result<Help> {
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
    for &(verb, id) in crate::grammar::QUERIES {
        let schema: Value = serde_json::from_str(
            crate::schema::find(id).ok_or_else(|| io::Error::other("missing query schema"))?,
        )?;
        let properties = field(&schema, "properties")
            .as_object()
            .ok_or_else(|| io::Error::other("missing properties"))?;
        let arguments = properties
            .iter()
            .map(|(name, node)| argument(name, node, &schema))
            .collect();
        let sections = section_words(&schema);
        commands.push(HelpCommand {
            command: vec![verb.into(), "gene".into()],
            description: text(&schema, "description"),
            arguments,
            section_rule: (!sections.is_empty()).then(|| {
                text(
                    field(field(&schema, "properties"), "section"),
                    "description",
                )
            }),
            sections,
            examples: examples.get(verb).cloned().unwrap_or_default(),
        });
    }
    for (command, description) in [
        (
            vec!["sync", "[hgnc|clingen]"],
            "Synchronize a local source bundle.",
        ),
        (
            vec!["schema", "<artifact id>"],
            "Print a checked-in schema artifact.",
        ),
        (
            vec!["help", "[get gene|search gene]"],
            "Print generated command help.",
        ),
        (
            vec!["help", "key", "oncokb"],
            "Show OncoKB credential configuration names.",
        ),
        (
            vec!["serve"],
            "Serve the shared command dispatcher over MCP.",
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
    Ok(commands)
}
fn argument(name: &str, node: &Value, root: &Value) -> HelpArgument {
    let position = field(node, "x-positional").as_u64();
    let resolved = resolve(node, root);
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
        choices: field(resolved, "enum")
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
    let mut lines = vec!["Run BioMCP commands. Human gene queries return one card or page by default; use sections for details.".into()];
    for command in commands {
        let mut line = command.command.join(" ");
        for argument in command.arguments {
            line.push_str(&format!(" [{}]", argument.name));
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
    let section = resolve(
            field(field(field(schema, "properties"), "section"), "items"),
            schema,
        );
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
