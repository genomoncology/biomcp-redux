//! Combine schema argument facts, recorded examples and registry source coverage.
use biomcp_types::{HelpArgument, HelpCommand};

pub(super) fn complete(commands: &mut [HelpCommand]) {
    for command in commands {
        let schema = command
            .command
            .get(..2)
            .and_then(|path| {
                crate::grammar::command_paths()
                    .iter()
                    .find(|(verb, entity, _)| path == [*verb, *entity])
                    .and_then(|(_, _, id)| crate::schema::find(id))
            })
            .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok());
        let sources = match (command.command.get(..2), &schema) {
            (Some([verb, entity]), Some(schema)) => {
                super::coverage::command_sources(verb, entity, schema)
            }
            _ => Default::default(),
        };
        let shape = usage_form(command);
        let selectable = command.arguments.iter().any(|a| a.name == "--source");
        let flags = command
            .arguments
            .iter()
            .filter(|a| a.name.starts_with("--"))
            .map(|a| a.name.clone())
            .collect::<Vec<_>>();
        for argument in &mut command.arguments {
            if !argument.choices.is_empty() && argument.value_type != "boolean" {
                argument.description.push('\n');
                argument
                    .description
                    .push_str(crate::grammar::ENUM_CASE_GUIDANCE);
            }
            if !argument.name.starts_with("--") {
                // Agents guess `--section`; say the word goes bare in its place.
                if argument.position.is_some() {
                    let flag = if argument.aliases.is_empty()
                        && !flags.contains(&format!("--{}", argument.name))
                    {
                        format!("; not --{}", argument.name)
                    } else {
                        String::new()
                    };
                    argument
                        .description
                        .push_str(&format!("\nForm: {shape}{flag}."));
                }
                continue;
            }
            let sample = example(argument, &command.examples);
            let only = schema
                .as_ref()
                .filter(|_| selectable)
                .map(|schema| only_sources(schema, &argument.name, &sources))
                .unwrap_or_default();
            let scope = if sources.is_empty() {
                "Local command option; independent of source selection.".into()
            } else if only.is_empty() {
                format!(
                    "Command sources: {}; source restrictions in the field description apply.",
                    sources.iter().cloned().collect::<Vec<_>>().join(", ")
                )
            } else {
                // The schema's source fact, so a filter names the sources that serve it.
                format!("Only with --source {}.", or_list(&only))
            };
            let sample = if argument.description.contains("Example:") {
                String::new()
            } else {
                format!(" Example: {sample}.")
            };
            let form = if argument.value_type == "array" {
                "repeatable values (repeat the flag)"
            } else {
                &argument.value_type
            };
            argument
                .description
                .push_str(&format!(" Value form: {form}.{sample} {scope}"));
        }
    }
}
/// The command words, then each positional argument in order, as one call shape.
fn usage_form(command: &HelpCommand) -> String {
    let mut positional = command
        .arguments
        .iter()
        .filter_map(|argument| Some((argument.position?, argument)))
        .collect::<Vec<_>>();
    positional.sort_by_key(|(position, _)| *position);
    command
        .command
        .iter()
        .cloned()
        .chain(positional.into_iter().map(|(_, argument)| {
            let word = argument.name.to_uppercase();
            if argument.value_type == "array" {
                format!("{word}...")
            } else {
                word
            }
        }))
        .collect::<Vec<_>>()
        .join(" ")
}
/// Command sources an option's schema field requires, when it names only some of them.
fn only_sources(
    schema: &serde_json::Value,
    name: &str,
    sources: &std::collections::BTreeSet<String>,
) -> Vec<String> {
    let required = crate::mcp::typed::catalog::argument_sources(schema, name)
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>();
    // A component that is no selectable source (a vocabulary) leaves the option general.
    if required.is_empty()
        || required.len() >= sources.len()
        || !required.iter().all(|source| sources.contains(source))
    {
        return Vec::new();
    }
    required
}
fn or_list(words: &[String]) -> String {
    match words {
        [] => String::new(),
        [one] => one.clone(),
        [rest @ .., last] => format!("{} or {last}", rest.join(", ")),
    }
}
fn example(argument: &HelpArgument, examples: &[Vec<String>]) -> String {
    for example in examples {
        for (index, word) in example.iter().enumerate() {
            if word == &argument.name {
                let value = example
                    .get(index + 1)
                    .filter(|value| !value.starts_with("--"));
                return value.map_or_else(
                    || word.clone(),
                    |value| {
                        format!(
                            "{word} {}",
                            shlex::try_quote(value)
                                .map_or_else(|_| "<value>".into(), |value| value.into_owned())
                        )
                    },
                );
            }
            if word.starts_with(&format!("{}=", argument.name)) {
                return word.clone();
            }
        }
    }
    if argument.value_type == "boolean" {
        return argument.name.clone();
    }
    let value = argument
        .choices
        .first()
        .cloned()
        .or_else(|| argument.default.clone())
        .or_else(|| {
            (argument.value_type == "integer" || argument.value_type == "number")
                .then(|| argument.minimum.unwrap_or(0).to_string())
        });
    let Some(value) = value else {
        return format!(
            "{} <{}> (syntax; replace the placeholder with your retained value)",
            argument.name,
            argument.name.trim_start_matches("--")
        );
    };
    format!(
        "{} {}",
        argument.name,
        shlex::try_quote(&value).map_or_else(|_| "<value>".into(), |value| value.into_owned())
    )
}
