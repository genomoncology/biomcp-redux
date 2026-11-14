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
        let own = command.command.join(" ");
        let rule = command.section_rule.clone();
        let mut cased = false;
        for argument in &mut command.arguments {
            drop_pointer(&mut argument.description, &own, rule.as_deref());
            cased |= !argument.choices.is_empty() && argument.value_type != "boolean";
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
            option_sentence(argument, &sample, &only);
        }
        // Lines that hold for every option are said once, on the command.
        if cased {
            command.description.push('\n');
            command
                .description
                .push_str(crate::grammar::ENUM_CASE_GUIDANCE);
        }
        if !flags.is_empty() {
            command.description.push('\n');
            command.description.push_str(if sources.is_empty() {
                LOCAL_OPTIONS
            } else {
                SOURCE_OPTIONS
            });
        }
    }
}
/// One sentence per option that names the option, so no option repeats another's.
fn option_sentence(argument: &mut HelpArgument, sample: &str, only: &[String]) {
    let form = if argument.value_type == "array" {
        "repeatable values (repeat the flag)"
    } else {
        &argument.value_type
    };
    // The option's own text ends its sentence before the form sentence starts.
    if !argument.description.trim_end().ends_with('.') {
        argument.description.push('.');
    }
    if argument.description.contains("Example: ") {
        // The field's own example leads; the form joins its sentence.
        argument.description = argument.description.replacen(
            "Example: ",
            &format!("Value form: {form}; example: "),
            1,
        );
        if !only.is_empty() {
            argument
                .description
                .push_str(&format!(" Only with --source {}.", or_list(only)));
        }
    } else {
        // The schema's source fact, so a filter names the sources that serve it.
        let scope = if only.is_empty() {
            String::new()
        } else {
            format!("; only with --source {}", or_list(only))
        };
        argument
            .description
            .push_str(&format!(" Value form: {form}; example: {sample}{scope}."));
    }
}
/// Said once per command page instead of after every option.
pub(super) const SOURCE_OPTIONS: &str =
    "Options apply to every listed source unless their description restricts them.";
pub(super) const LOCAL_OPTIONS: &str =
    "Options are local to the command and independent of source selection.";
/// A command description without the lines that speak for its options, for pages that omit them.
pub(super) fn without_option_lines(description: &str) -> String {
    description
        .lines()
        .filter(|line| {
            ![
                crate::grammar::ENUM_CASE_GUIDANCE,
                SOURCE_OPTIONS,
                LOCAL_OPTIONS,
            ]
            .contains(line)
        })
        .collect::<Vec<_>>()
        .join("\n")
}
/// A field's compact pointer to help says nothing when its full text follows on this page.
/// A section field's line has no full text after it; when the section rule goes on, it says so.
fn drop_pointer(description: &mut String, own: &str, rule: Option<&str>) {
    let mut lines = description.lines();
    let Some((short, target)) = lines.next().and_then(|first| first.rsplit_once("; help ")) else {
        return;
    };
    let rest = lines.collect::<Vec<_>>();
    let head = if rest
        .first()
        .is_some_and(|next| next.starts_with(short.trim_end_matches('.')))
    {
        None
    } else if target == own {
        let cut = rule
            .and_then(|rule| rule.strip_prefix(short))
            .is_some_and(|tail| !tail.is_empty());
        Some(if cut {
            format!("{short}… (full rule with the sections).")
        } else {
            format!("{}.", short.trim_end_matches('.'))
        })
    } else {
        return;
    };
    *description = head
        .into_iter()
        .chain(rest.into_iter().map(String::from))
        .collect::<Vec<_>>()
        .join("\n");
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
