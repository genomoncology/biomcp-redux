//! Render documentation from the parser, schema help and harvested case steps.
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
        writeln!(recipes, "## {}\n\n```sh", entry.command.join(" "))?;
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
        assert_eq!(
            fs::read_to_string(root().join(name))?,
            expected,
            "{name}: run make docs-generate"
        );
    }
    Ok(())
}

#[test]
#[ignore = "explicit documentation generation"]
fn write_reference() -> Check {
    for (name, text) in documents()? {
        let path = root().join(name);
        fs::create_dir_all(path.parent().ok_or("documentation directory missing")?)?;
        fs::write(path, text)?;
    }
    Ok(())
}
