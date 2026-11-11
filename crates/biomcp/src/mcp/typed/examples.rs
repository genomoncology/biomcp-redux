//! Worked examples from the skill's use cases, written in the form this connection calls.
use serde_json::Value;
use std::io;

const EXAMPLES: &str = include_str!(concat!(env!("OUT_DIR"), "/first-read-examples.json"));

fn all() -> io::Result<Vec<Value>> {
    serde_json::from_str(EXAMPLES).map_err(io::Error::other)
}

/// The typed call this server would make of a use case's command, if it serves one.
fn call(root: &crate::grammar::Root, example: &Value, catalog: &super::Catalog) -> Option<Value> {
    let args = example.get("args")?.as_array()?;
    super::summary::translate(root, args, catalog)
}

/// One line per use case this connection can run: the typed call in typed mode, else CLI words.
pub(in crate::mcp) fn lines(
    context: crate::command::Context<'_>,
    typed: bool,
) -> io::Result<String> {
    let catalog = super::catalog(context, true)?;
    let mut lines = vec![if typed {
        "Worked examples (use case: tool call):".to_owned()
    } else {
        "Worked examples (use case: biomcp args):".to_owned()
    }];
    for example in all()? {
        let (Some(call), Some(label), Some(command)) = (
            call(context.root, &example, &catalog),
            example.get("label").and_then(Value::as_str),
            example.get("command").and_then(Value::as_str),
        ) else {
            continue;
        };
        lines.push(match (typed, call.get("name"), call.get("arguments")) {
            (true, Some(Value::String(tool)), Some(arguments)) => {
                format!("{label}: {tool} {arguments}")
            }
            _ => format!("{label}: {command}"),
        });
    }
    Ok(lines.join("\n"))
}
