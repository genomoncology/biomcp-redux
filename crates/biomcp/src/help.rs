//! Schema-fed command help shared with MCP discovery.

mod source_settings;

use biomcp_types::{Help, HelpArgument, HelpCommand, HelpWord};
use serde_json::Value;
use std::{collections::BTreeMap, io};

pub(crate) fn dispatch(path: &[String]) -> io::Result<Help> {
    if let [topic] = path
        && matches!(topic.as_str(), "sources" | "config")
    {
        return Ok(source_settings::help(topic));
    }
    if let [category, setting] = path
        && category == "network"
    {
        return Ok(network(setting));
    }
    let commands = commands()?
        .into_iter()
        .filter(|c| {
            path.is_empty()
                || c.command.starts_with(path)
                || (path.len() == 1
                    && path.first().is_some_and(|word| word == "skill")
                    && c.command.first() == path.first())
                || (path.len() == 1 && c.command.get(1) == path.first())
                || (path.len() == 1
                    && path.first().is_some_and(|word| word == "author")
                    && c.command.first() == path.first())
        })
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
            description: command_description(verb, entity, &schema),
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
    commands.extend(crate::housekeeping::help());
    extra_commands(&mut commands, &examples);
    Ok(commands)
}
const AUTHOR_BOUNDARY: &str = " Author IDs identify provider records, not resolved people. Counts cover that provider record. \
    Missing metadata is unavailable. Topic-wide author ranking and cross-provider merging are \
    unavailable. Semantic Scholar and ORCID are noncommercial; commercial users disable both with \
    --disable-sources noncommercial or config disable_sources. Optional keys: biomcp key set \
    semantic-scholar or S2_API_KEY; biomcp key set orcid or ORCID_ACCESS_TOKEN. IDs use \
    semantic-scholar:ID (alias semanticscholar:ID) or orcid:ID. Search supports --source \
    semantic-scholar (alias semanticscholar), --query/-q and --limit/-l; papers supports --limit/-l \
    and Semantic Scholar details/--full.";
const CELL_LINE_BOUNDARY: &str = "An assay count is not an activity or sensitivity count. No treatment response is inferred; no bioinformatics tool runs.";

fn command_description(verb: &str, entity: &str, schema: &Value) -> String {
    let mut description = text(schema, "description");
    if entity == "author" || verb == "author" {
        description.push_str(AUTHOR_BOUNDARY);
    }
    if entity == "cell-line" || verb == "cell-line" {
        description.push(' ');
        description.push_str(CELL_LINE_BOUNDARY);
    }
    description
}

fn extra_commands(commands: &mut Vec<HelpCommand>, examples: &BTreeMap<String, Vec<Vec<String>>>) {
    for (command, description) in [
        (
            vec!["store", "clean"],
            "Remove expired replies, pools and snapshots and older incompatible cache files. Return removal \
                counts in one bounded reply. Fresh state and bundles remain.",
        ),
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
            "Serve MCP over stdio or --http [ADDR]. Non-loopback binds require --allow-host HOST or \
                http.allowed_hosts; neither authenticates callers. HTTP refuses sync, store clean and --fresh. \
                Stdio saves refuse --replace.",
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
    for &bundle in crate::grammar::BUNDLE_HELP {
        commands.push(HelpCommand {
            command: vec!["sync".into(), bundle.into()],
            description: if bundle == "wikipathways" {
                "Replace WikiPathways search/get metadata catalogs atomically; reads never sync. Membership names a \
                    source and release; it implies no therapy, response or actionability. No enrichment is calculated."
            } else {
                "Replace the complete validated local bundle atomically. Reads use stored data and never \
                    synchronize. Failed replacement preserves the last usable bundle."
            }
            .into(),
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
    let mut lines = tree_intro();
    for command in commands {
        if command.command.first().is_none_or(|verb| {
            !matches!(
                verb.as_str(),
                "search"
                    | "get"
                    | "article"
                    | "trial"
                    | "variant"
                    | "sync"
                    | "pathway"
                    | "cell-line"
                    | "author"
            )
        }) {
            continue;
        }
        if command.command == ["sync", "[hgnc|clingen]"] {
            continue;
        }
        let mut line = command.command.join(" ");
        // Detailed options belong to help COMMAND; retain search facets
        // required for discovering checked gene and disease searches.
        if command.command == ["search", "gene"] || command.command == ["search", "disease"] {
            for argument in command
                .arguments
                .iter()
                .filter(|arg| discovery_argument(arg))
            {
                line.push_str(&format!(" {}", argument.name));
            }
        }
        if command.command.iter().any(|word| word == "adverse-event") {
            for argument in command.arguments.iter().filter(|arg| {
                arg.position.is_none() && arg.name != "--snapshot" && discovery_argument(arg)
            }) {
                line.push_str(&format!(" {}", argument.name));
                if !argument.choices.is_empty() {
                    line.push_str(&format!(" {}", argument.choices.join("|")));
                }
            }
        }
        if command.command == ["search", "protein"] {
            line.push_str(" --gene --reviewed");
        }
        author_selectors(&command, &mut line);
        cell_line_selectors(&command, &mut line);
        pgx_selectors(&command, &mut line);
        gwas_selectors(&command, &mut line);
        pathway_selectors(&command, &mut line);
        diagnostic_selectors(&command, &mut line);
        study_selectors(&command, &mut line);
        phenotype_selectors(&command, &mut line);
        if !command.sections.is_empty() {
            line.push_str(" :");
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
    lines.join("\n")
}

fn tree_intro() -> Vec<String> {
    vec![
        "help CMD;schema ID".into(),
        CELL_LINE_BOUNDARY.into(),
        "Author IDs name provider records;no ranking/merge. Both sources noncommercial;--disable-sources \
            noncommercial. Optional keys: biomcp key set semantic-scholar (S2_API_KEY);biomcp key set orcid \
            (ORCID_ACCESS_TOKEN)"
            .into(),
        "protein features --position --type;domains --type;isoforms;complexes;complex-members --complex;structures;sequence".into(),
        "Human source facts;no variant effect/structure/interaction/clinical inference;no bioinformatics tool runs".into(),
        "Pages --limit --offset --pool; sections --section-limit --section-offset".into(),
        "Pathway membership names source/release; no therapy, response, actionability or enrichment".into(),
        "GWAS source associations; no personal risk, causal variant/gene or clinical statistic \
            interpretation; --p-value requires default p-value-ascending"
            .into(),
    ]
}

fn author_selectors(command: &HelpCommand, line: &mut String) {
    if command.command == ["get", "author"] {
        line.push_str(" semantic-scholar:ID|orcid:ID (semanticscholar:ID alias)");
    }
    if command.command == ["author", "papers"] {
        line.push_str(" details|--full --limit/-l --offset --pool");
    }
    if command.command == ["search", "author"] {
        line.push_str(
            " --source semantic-scholar|semanticscholar --query/-q --limit/-l --offset --pool",
        );
    }
}

fn cell_line_selectors(command: &HelpCommand, line: &mut String) {
    if command.command == ["search", "cell-line"] {
        line.push_str(" query --query/-q --limit/-l --offset");
    }
    if command.command == ["get", "cell-line"] {
        line.push_str(" CVCL_ID|SOURCE:ID --section-limit --section-offset");
    }
}

fn phenotype_selectors(command: &HelpCommand, line: &mut String) {
    if command.command.get(1).is_some_and(|v| v == "phenotype") {
        for argument in command.arguments.iter().filter(|a| a.position.is_none()) {
            line.push_str(&format!(" {}", argument.name));
            if argument.name == "--source" {
                line.push(' ');
                line.push_str(&argument.choices.join("|"));
            }
        }
        if command.command.first().is_some_and(|verb| verb == "get") {
            line.push_str(
                "; terms are terminology, associations publisher assertions; neither diagnoses",
            );
        }
    }
}

fn pathway_selectors(command: &HelpCommand, line: &mut String) {
    if command.command.iter().any(|word| word == "pathway") {
        for argument in command
            .arguments
            .iter()
            .filter(|arg| discovery_argument(arg))
        {
            line.push_str(&format!(" {}", argument.name));
            if argument.name == "--source" {
                line.push_str(&format!(" {}", argument.choices.join("|")));
            }
        }
        match command.command.as_slice() {
            [verb, _] if verb == "get" => line.push_str("; R-HSA|WP"),
            [verb, relation] if verb == "pathway" && relation == "genes" => {
                line.push_str("; Reactome|WikiPathways GPML")
            }
            [verb, relation] if verb == "pathway" && relation == "events" => {
                line.push_str("; Reactome")
            }
            _ => {}
        }
    }
}

fn pgx_selectors(command: &HelpCommand, line: &mut String) {
    if command.command.iter().any(|word| word == "pgx") {
        for argument in command.arguments.iter().filter(|arg| {
            discovery_argument(arg)
                && matches!(
                    arg.name.as_str(),
                    "--source"
                        | "--diplotype"
                        | "--kind"
                        | "--cpic-level"
                        | "--limit"
                        | "--offset"
                        | "--section-limit"
                        | "--section-offset"
                )
        }) {
            line.push_str(&format!(" {}", argument.name));
            if argument.name == "--source" {
                line.push_str(&format!(" {}", argument.choices.join("|")));
            }
        }
    }
}

fn discovery_argument(argument: &HelpArgument) -> bool {
    !matches!(
        argument.name.as_str(),
        "--limit" | "--offset" | "--section-limit" | "--section-offset"
    )
}

fn gwas_selectors(command: &HelpCommand, line: &mut String) {
    if command.command.iter().any(|word| word == "gwas") {
        for argument in command
            .arguments
            .iter()
            .filter(|arg| discovery_argument(arg))
        {
            line.push_str(&format!(" {}", argument.name));
        }
    }
}

fn study_selectors(command: &HelpCommand, line: &mut String) {
    if command.command.iter().any(|word| word == "study") {
        for argument in command
            .arguments
            .iter()
            .filter(|arg| arg.position.is_none() && discovery_argument(arg))
        {
            line.push_str(&format!(" {}", argument.name));
        }
    }
}

fn diagnostic_selectors(command: &HelpCommand, line: &mut String) {
    if command.command.iter().any(|word| word == "diagnostic")
        && let Some(source) = command.arguments.iter().find(|a| a.name == "--source")
    {
        line.push_str(&format!(" --source {}", source.choices.join("|")));
        for argument in command.arguments.iter().filter(|argument| {
            matches!(
                argument.name.as_str(),
                "--gene"
                    | "--gene-id"
                    | "--condition-id"
                    | "--type"
                    | "--device-id"
                    | "--drug-application"
                    | "--biomarker"
                    | "--marker"
                    | "--manufacturer"
                    | "--supplement"
            )
        }) {
            line.push_str(&format!(" {}", argument.name));
        }
    }
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
            "Set ca_bundle in config.json to a PEM certificate file. These roots add to platform trust for every source. \
        Keep certificate verification enabled."
        }
        _ => {
            "Set proxy in config.json to an HTTP or HTTPS proxy URL. Otherwise HTTPS_PROXY and HTTP_PROXY \
                select routes for their schemes; NO_PROXY bypasses exact hosts/IPs, .domain, *.domain, optional \
                ports, or *. An explicit proxy overrides proxy environment settings."
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
