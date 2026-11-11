//! On-request guide kept out of the first read: commands by question and worked examples.
use biomcp_types::{Help, HelpCommand};
use serde_json::Value;
use std::io;

const QUESTIONS: &str = include_str!(concat!(env!("OUT_DIR"), "/question-map.txt"));

/// `help questions`: every command under the tool that serves it, with what it answers.
pub(super) fn questions() -> Help {
    Help {
        commands: vec![super::topic("questions", QUESTIONS.into())],
    }
}

/// `help examples`: each use case's first call as a CLI command and the derived typed call.
pub(super) fn examples(root: &crate::grammar::Root) -> io::Result<Help> {
    Ok(Help {
        commands: vec![super::topic("examples", lines(root, None)?)],
    })
}

/// The page that opens `help ENTITY` for an entity tool: its question line and its examples.
pub(super) fn tool_page(
    root: &crate::grammar::Root,
    entity: &str,
) -> io::Result<Option<HelpCommand>> {
    let tool = entity.replace('-', "_");
    let Some(question) = QUESTIONS
        .lines()
        .find(|line| line.starts_with(&format!("- {tool} (")))
    else {
        return Ok(None);
    };
    let examples = lines(root, Some(&tool))?;
    let mut text = question.trim_start_matches("- ").to_owned();
    if !examples.is_empty() {
        text.push_str("\nWorked examples:\n");
        text.push_str(&examples);
    }
    Ok(Some(super::topic(entity, text)))
}

fn lines(root: &crate::grammar::Root, tool: Option<&str>) -> io::Result<String> {
    let mut lines = Vec::new();
    for (example, call) in crate::mcp::example_calls(root)? {
        let field = |key: &str| example.get(key).and_then(Value::as_str).unwrap_or_default();
        let typed = call
            .as_ref()
            .and_then(|call| Some((call.get("name")?.as_str()?, call.get("arguments")?)));
        if tool.is_some_and(|tool| typed.is_none_or(|(name, _)| name != tool)) {
            continue;
        }
        let typed = typed.map_or_else(
            || "no typed call".to_owned(),
            |(name, arguments)| format!("{name} {arguments}"),
        );
        lines.push(format!(
            "- {} ({}): biomcp {} | {typed}",
            field("title"),
            field("name"),
            field("command")
        ));
    }
    Ok(lines.join("\n"))
}
