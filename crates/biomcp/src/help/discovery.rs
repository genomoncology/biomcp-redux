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
    commands.push(super::topic("relations", format!("Follow an entity to these named source sections with get ENTITY ID SECTION. Sections can lead to related source records.\n{relations}")));
    for command in commands {
        command.description = plain(&command.description);
        for argument in &mut command.arguments {
            argument.description = plain(&argument.description);
        }
        for section in &mut command.sections {
            section.description = plain(&section.description);
        }

        if command.examples.is_empty() {
            let mut example = vec!["help".to_owned()];
            if command.command.first().is_some_and(|v| v == "help") {
                example.extend(command.command.iter().skip(1).cloned());
            } else {
                example.extend(command.command.iter().cloned());
            }
            command.examples.push(example);
        }
    }
}

fn plain(text: &str) -> String {
    text.replace("native", "source")
        .replace("Native", "Source")
        .replace("provider", "source")
        .replace("Provider", "Source")
        .replace("literal", "exact")
        .replace("source source", "source")
}
