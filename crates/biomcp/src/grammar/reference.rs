//! Render documentation from the parser, schema help and harvested case steps.
use biomcp_types::{Help, ResponseBody};
use clap::Command;
use std::{collections::BTreeSet, fmt::Write, fs, path::PathBuf};

type Check = Result<(), Box<dyn std::error::Error>>;

fn documents() -> Result<Vec<(&'static str, String)>, Box<dyn std::error::Error>> {
    let mut parser = super::root().map_err(|()| "command grammar unavailable")?;
    parser.build();
    let help = crate::help::dispatch(&[])?;
    let mut reference = String::from(
        "# Command reference\n\nGenerated from the command grammar and schema help. \
        Run `make docs-generate` to update.\n\n\
        Options shown on the root command apply to its subcommands. \
        Query sections and source rules follow each command.\n\n",
    );
    let mut printed = BTreeSet::new();
    command(&mut reference, &parser, &[], &help, &mut printed)?;
    for entry in &help.commands {
        if !printed.contains(&entry.command.join(" ")) {
            writeln!(reference, "## biomcp {}\n", entry.command.join(" "))?;
            details(&mut reference, entry)?;
        }
    }
    let mut recipes = String::from(
        "# Worked commands\n\nGenerated from harvested case run steps through the shipped help examples. \
        The documentation check executes these commands against recordings through CLI and MCP. \
        Live source replies can differ.\n\n\
        Choose a command below, inspect its source outcomes and follow its returned next commands. \
        Stored sources require the explicit synchronization shown by their command.\n\n",
    );
    for entry in help
        .commands
        .iter()
        .filter(|entry| !entry.examples.is_empty())
    {
        let name = std::iter::once("biomcp")
            .chain(entry.command.iter().map(String::as_str))
            .collect::<Vec<_>>()
            .join(" ");
        writeln!(recipes, "## {name}\n\n```sh")?;
        for args in &entry.examples {
            writeln!(
                recipes,
                "biomcp {}",
                shlex::try_join(args.iter().map(String::as_str))?
            )?;
        }
        writeln!(recipes, "```\n")?;
    }
    let skill = fs::read_to_string(root().join("skills/SKILL.md"))?;
    let (before, rest) = skill
        .split_once("<!-- exit-reference:start -->")
        .ok_or("missing exit reference start")?;
    let (_, after) = rest
        .split_once("<!-- exit-reference:end -->")
        .ok_or("missing exit reference end")?;
    let skill = format!(
        "{before}<!-- exit-reference:start -->\n{}\n<!-- exit-reference:end -->{after}",
        crate::exit::reference()
    );
    Ok(vec![
        ("skills/SKILL.md", skill),
        ("docs/reference/commands.md", reference),
        ("docs/recipes.md", recipes),
    ])
}

fn command(
    output: &mut String,
    parser: &Command,
    parents: &[&str],
    help: &biomcp_types::Help,
    printed: &mut BTreeSet<String>,
) -> Result<(), std::fmt::Error> {
    let mut path = parents.to_vec();
    path.push(parser.get_name());
    printed.insert(path.iter().skip(1).copied().collect::<Vec<_>>().join(" "));
    writeln!(output, "## {}\n\n```text", path.join(" "))?;
    let mut printable = parser.clone().bin_name(path.join(" "));
    writeln!(output, "{}\n```\n", printable.render_long_help())?;
    if let Some(entry) = help.commands.iter().find(|entry| {
        entry
            .command
            .iter()
            .map(String::as_str)
            .eq(path.iter().skip(1).copied())
    }) {
        details(output, entry)?;
    }
    for child in parser.get_subcommands() {
        command(output, child, &path, help, printed)?;
    }
    Ok(())
}

fn details(output: &mut String, entry: &biomcp_types::HelpCommand) -> std::fmt::Result {
    writeln!(output, "{}\n", entry.description)?;
    for arg in &entry.arguments {
        writeln!(output, "- `{}`: {}", arg.name, arg.description)?;
    }
    if let Some(rule) = &entry.section_rule {
        writeln!(output, "\n{rule}\n")?;
    }
    for section in &entry.sections {
        writeln!(output, "- `{}`: {}", section.word, section.description)?;
    }
    writeln!(output)
}

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
}

#[test]
fn reference_matches_grammar() -> Check {
    for (name, expected) in documents()? {
        assert!(
            fs::read_to_string(root().join(name))? == expected,
            "{name}: run make docs-generate"
        );
    }
    Ok(())
}

#[test]
#[ignore = "explicit documentation generation"]
fn write_reference() -> Check {
    refresh_help_cases(&root().join("tests/cases"))?;
    for (name, text) in documents()? {
        let path = root().join(name);
        fs::create_dir_all(path.parent().ok_or("documentation directory missing")?)?;
        fs::write(path, text)?;
    }
    Ok(())
}

// Regenerate descriptive help snapshots from their owner, preserving other case values.
fn refresh_help_cases(directory: &std::path::Path) -> Check {
    use std::io::Write as _;
    for entry in fs::read_dir(directory)? {
        let path = entry?.path();
        if path.is_dir() {
            refresh_help_cases(&path)?;
            continue;
        }
        if path.extension().is_none_or(|v| v != "yaml") {
            continue;
        }
        let text = fs::read_to_string(&path)?;
        // JSON-shaped fixtures must retain JSON's numeric and string types.
        let json_text = text
            .lines()
            .filter(|line| !line.trim_start().starts_with('#'))
            .collect::<Vec<_>>()
            .join("\n");
        let mut case: serde_json::Value = if json_text.trim_start().starts_with('{') {
            serde_json::from_str(&json_text)?
        } else {
            serde_saphyr::from_str(&text)?
        };
        let mut changed = false;
        for (index, step) in case["steps"]
            .as_array_mut()
            .into_iter()
            .flatten()
            .enumerate()
        {
            let Some(args) = step["run"]
                .as_array()
                .filter(|args| args.first().and_then(serde_json::Value::as_str) == Some("help"))
            else {
                continue;
            };
            let args: Vec<std::ffi::OsString> = args
                .iter()
                .filter_map(serde_json::Value::as_str)
                .map(Into::into)
                .collect();
            let reply = crate::command::dispatch(&args)?;
            if reply.value["schema"] != Help::SCHEMA {
                continue;
            }
            let bytes = reply.cli_bytes.len();
            let markdown_bytes = crate::view::markdown(&reply.value)?.len();
            writeln!(
                std::io::stdout(),
                "HELP_MARKDOWN {} step {index}: {markdown_bytes} bytes",
                path.display()
            )?;
            writeln!(
                std::io::stdout(),
                "HELP_MEASURE {} step {index}: {bytes} bytes; max_bytes {} max_tokens {}",
                path.display(),
                step["max_bytes"],
                step["max_tokens"]
            )?;
            changed |= refresh_expectations(step, &reply.value);
        }
        if changed {
            fs::write(path, format!("{}\n", serde_json::to_string_pretty(&case)?))?;
        }
    }
    Ok(())
}

fn refresh_expectations(step: &mut serde_json::Value, reply: &serde_json::Value) -> bool {
    let mut changed = false;
    if let Some(expect) = step["expect"].as_object_mut() {
        for (pointer, expected) in expect {
            if pointer.ends_with("/arguments/*/name")
                && expected.is_array()
                && let Some(arguments) = pointer
                    .strip_suffix("/*/name")
                    .and_then(|path| reply.pointer(path))
                    .and_then(serde_json::Value::as_array)
            {
                let actual: serde_json::Value = arguments
                    .iter()
                    .filter_map(|a| a.get("name").cloned())
                    .collect();
                if *expected != actual {
                    *expected = actual;
                    changed = true;
                }
            }
            if pointer.ends_with("/description")
                && expected.is_string()
                && let Some(actual) = reply.pointer(pointer)
                && actual != expected
            {
                *expected = actual.clone();
                changed = true;
            }
        }
    }
    changed
}

// Temporary parser diagnosis; remove after the outside-in format matrix proves the shared selector.
#[test]
#[ignore]
fn presentation_partial_match_probe() -> Check {
    use std::io::Write as _;
    let mut cases = Vec::new();
    for words in [
        vec!["health", "--offline"],
        vec!["get", "gene", "--offline"],
        vec!["get", "trial", "not-a-trial", "--offline"],
    ] {
        for format in [None, Some("json"), Some("md")] {
            for config in [false, true] {
                let mut args = words.clone();
                if config {
                    args.extend(["--config", "explicit-config.json"]);
                }
                if let Some(format) = format {
                    args.extend(["--format", format]);
                }
                cases.push((args, format, config));
            }
        }
    }
    let mut failures = Vec::new();
    for (args, expected_format, expected_config) in cases {
        let (format, config) = partial_presentation_probe(&args)?;
        writeln!(
            std::io::stdout(),
            "PRESENTATION_MATRIX {}",
            serde_json::json!({"args":args,"format":format,"config":config})
        )?;
        if format.as_deref() != expected_format || config.is_some() != expected_config {
            failures.push(args);
        }
    }
    assert!(
        failures.is_empty(),
        "canonical partial grammar lost globals: {failures:?}"
    );
    for args in [
        vec!["get", "gene", "--help", "--format", "md"],
        vec!["get", "gene", "--offline", "--", "--format", "md"],
        vec![
            "--format",
            "md",
            "get",
            "gene",
            "BRAF",
            "--offline",
            "--unknown",
        ],
        vec![
            "get",
            "gene",
            "BRAF",
            "--offline",
            "--unknown",
            "--format",
            "md",
        ],
        vec![
            "search",
            "article",
            "--limit",
            "not-a-number",
            "--format",
            "md",
            "--offline",
        ],
    ] {
        let (format, config) = partial_presentation_probe(&args)?;
        writeln!(
            std::io::stdout(),
            "PRESENTATION_BOUNDARY {}",
            serde_json::json!({"args":args,"format":format,"config":config})
        )?;
    }
    Ok(())
}

#[cfg(test)]
fn partial_presentation_probe(
    args: &[&str],
) -> Result<(Option<String>, Option<PathBuf>), Box<dyn std::error::Error>> {
    let args: Vec<std::ffi::OsString> = args.iter().map(Into::into).collect();
    let mut parser = super::root().map_err(|()| "grammar unavailable")?;
    parser.build();
    let args = super::entry::arguments(&args, &parser);
    let matches = parser
        .ignore_errors(true)
        .try_get_matches_from(std::iter::once(std::ffi::OsString::from("biomcp")).chain(args))?;
    let format = matches
        .get_one::<String>("format")
        .filter(|_| matches.value_source("format") == Some(clap::parser::ValueSource::CommandLine))
        .cloned();
    Ok((format, matches.get_one::<PathBuf>("config").cloned()))
}
