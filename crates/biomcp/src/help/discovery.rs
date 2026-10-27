//! Build navigation from the same commands and sections used for execution.
use biomcp_types::HelpCommand;

pub(super) fn complete(commands: &mut Vec<HelpCommand>) {
    let relations = commands
        .iter()
        .filter(|command| {
            command.command.first().is_some_and(|v| v == "get") && !command.sections.is_empty()
        })
        .map(|command| {
            format!(
                "{} -> {}",
                command.command[1],
                command
                    .sections
                    .iter()
                    .map(|s| s.word.as_str())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    commands.push(super::topic(
        "relations",
        format!(
            "Follow an entity to these named source sections with get ENTITY ID SECTION. \
        Sections can lead to related source records.\n{relations}"
        ),
    ));
    let examples: Vec<_> = commands
        .iter()
        .filter(|command| !command.examples.is_empty())
        .map(|command| (command.command.clone(), command.examples[0].clone()))
        .collect();
    for command in commands.iter_mut() {
        command.description = plain(&command.description);
        for argument in &mut command.arguments {
            argument.description = plain(&argument.description);
        }
        if let Some(rule) = &mut command.section_rule {
            *rule = plain(rule);
        }
        for section in &mut command.sections {
            section.description = plain(&section.description);
        }
        if command.examples.is_empty() {
            let example = if command.command.is_empty() {
                vec!["help".into()]
            } else if let Some((_, example)) = examples
                .iter()
                .find(|(path, _)| path.starts_with(&command.command))
            {
                example.clone()
            } else if command.command == ["serve"] {
                vec!["help".into(), "serve".into()]
            } else {
                command.command.clone()
            };
            command.examples.push(example);
        }
    }
    if let Some(index) = commands.iter().position(|c| c.command.is_empty()) {
        let index = commands.remove(index);
        commands.insert(0, index);
    }
}

fn plain(text: &str) -> String {
    let mut out = String::new();
    for token in text.split_inclusive(|c: char| !c.is_alphanumeric() && c != '_') {
        let word = token.trim_end_matches(|c: char| !c.is_alphanumeric() && c != '_');
        out.push_str(match word {
            "native" | "provider" => "source",
            "Native" | "Provider" => "Source",
            "literal" => "exact",
            "Literal" => "Exact",
            _ => word,
        });
        out.push_str(&token[word.len()..]);
    }
    out.replace("source source", "source")
        .replace("source-source", "source")
}
