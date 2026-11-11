//! Generate the on-request question map and worked examples from the declarations and use cases.
use super::front_map::typed_front_map::{ENTITY_TOOLS, command_name};
use serde_json::{Value, json};
use std::{collections::BTreeSet, error::Error, fs, path::Path};

const TYPED: &str = "Typed tool: `";

/// One use case's first step, as a command line and as a typed tool call.
pub(crate) struct Example {
    name: String,
    title: String,
    command: String,
    tool: String,
    call: Value,
}

/// Read every use case's first command and its documented typed call.
pub(crate) fn examples(directory: &Path) -> Result<Vec<Example>, Box<dyn Error>> {
    let mut paths = fs::read_dir(directory)?
        .map(|entry| entry.map(|entry| entry.path()))
        .collect::<Result<Vec<_>, _>>()?;
    paths.sort();
    let mut examples = Vec::new();
    for path in paths {
        let Some(stem) = path.file_stem().and_then(|stem| stem.to_str()) else {
            continue;
        };
        if path.extension().is_none_or(|extension| extension != "md") || stem.starts_with('_') {
            continue;
        }
        examples.push(example(stem, &fs::read_to_string(&path)?)?);
    }
    Ok(examples)
}

fn example(stem: &str, text: &str) -> Result<Example, Box<dyn Error>> {
    let missing = |what: &str| format!("use case {stem} lacks {what}");
    let title = text
        .lines()
        .next()
        .and_then(|line| line.strip_prefix("# "))
        .ok_or_else(|| missing("a title"))?;
    let command = text
        .split("```sh\n")
        .nth(1)
        .and_then(|block| block.lines().next())
        .and_then(|line| line.strip_prefix("biomcp "))
        .ok_or_else(|| missing("a first biomcp command"))?;
    let typed = text
        .lines()
        .filter_map(|line| line.strip_prefix(TYPED))
        .collect::<Vec<_>>();
    let [typed] = typed.as_slice() else {
        return Err(missing("exactly one typed tool line").into());
    };
    let parts = typed.split('`').collect::<Vec<_>>();
    let [tool, " ", call, "."] = parts.as_slice() else {
        return Err(missing("a typed line of the form Typed tool: `tool` `{call}`.").into());
    };
    Ok(Example {
        name: stem.split_once('-').map_or(stem, |(_, name)| name).into(),
        title: title.into(),
        command: command.into(),
        tool: (*tool).into(),
        call: serde_json::from_str(call)?,
    })
}

/// Split a documented command line into words; use cases quote with single quotes only.
pub(crate) fn words(command: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let mut words = Vec::new();
    let mut word = String::new();
    let (mut quoted, mut started) = (false, false);
    for character in command.chars() {
        match character {
            '\'' => (quoted, started) = (!quoted, true),
            ' ' if !quoted => {
                if started {
                    words.push(std::mem::take(&mut word));
                }
                started = false;
            }
            '"' | '\\' | '$' | '`' => return Err(format!("unsupported quoting: {command}").into()),
            _ => {
                word.push(character);
                started = true;
            }
        }
    }
    if quoted {
        return Err(format!("unclosed quote: {command}").into());
    }
    if started {
        words.push(word);
    }
    Ok(words)
}

/// The examples the server reads to write its instructions for each tool mode.
pub(crate) fn embedded(examples: &[Example]) -> Result<Value, Box<dyn Error>> {
    examples
        .iter()
        .map(|example| {
            Ok(
                json!({"name":example.name,"title":example.title,"command":example.command,
                "args":words(&example.command)?,"tool":example.tool,"call":example.call}),
            )
        })
        .collect::<Result<Vec<_>, Box<dyn Error>>>()
        .map(Value::Array)
}

/// Every registry command under the tool that serves it, with the question it answers.
pub(crate) fn question_map(commands: &[(&str, &str, &str)]) -> Result<String, Box<dyn Error>> {
    let cli = |name: &str| {
        commands
            .iter()
            .find(|(verb, entity, _)| command_name(verb, entity) == name)
            .map(|(verb, entity, _)| format!("{verb} {entity}"))
    };
    let mut listed = BTreeSet::new();
    let mut lines = Vec::new();
    for (tool, asks, actions, _, _) in ENTITY_TOOLS {
        let mut own = Vec::new();
        let mut others = Vec::new();
        for (_, name, does) in *actions {
            let Some(command) = cli(name) else { continue };
            if !listed.insert(command.clone()) {
                continue;
            }
            if does.is_empty() {
                own.push(command);
            } else {
                others.push(format!("{command} ({does})"));
            }
        }
        let mut line = format!("- {tool} ({}): {asks}.", own.join(", "));
        if !others.is_empty() {
            line.push_str(&format!(" Also {}.", others.join("; ")));
        }
        lines.push(line);
    }
    for (verb, entity, _) in commands {
        if !listed.contains(&format!("{verb} {entity}")) {
            return Err(
                format!("registry command missing from the question map: {verb} {entity}").into(),
            );
        }
    }
    Ok(lines.join("\n"))
}
