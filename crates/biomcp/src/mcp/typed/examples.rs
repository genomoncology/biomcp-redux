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
pub(in crate::mcp) fn lines(context: crate::command::Context<'_>, typed: bool) -> io::Result<String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    use std::path::Path;

    fn recorded_runs() -> Vec<Value> {
        let mut runs = Vec::new();
        let mut pending = vec![Path::new(env!("CARGO_MANIFEST_DIR")).join("../../tests/cases")];
        while let Some(directory) = pending.pop() {
            for entry in std::fs::read_dir(directory).expect("case directory") {
                let path = entry.expect("case entry").path();
                if path.is_dir() {
                    pending.push(path);
                } else if path.extension().is_some_and(|extension| extension == "yaml") {
                    let text = std::fs::read_to_string(&path).expect("case text");
                    let Ok(case) = serde_json::from_str::<Value>(&text) else {
                        continue;
                    };
                    for step in case["steps"].as_array().into_iter().flatten() {
                        runs.push(step["run"].clone());
                    }
                }
            }
        }
        runs
    }

    /// Each example is a recorded case's command, parses, and is the call its use case documents.
    #[test]
    fn every_use_case_example_is_recorded_parsed_and_fits_its_tool() {
        let root = crate::grammar::root().expect("grammar");
        let globals = crate::grammar::Globals::default();
        let settings = Ok(None);
        let fetch = Err(crate::config::StartupError::Local);
        let context = crate::command::Context::new(&globals, &fetch, &root, &settings);
        let catalog = super::super::catalog(context, true).expect("catalog");
        let runs = recorded_runs();
        let examples = all().expect("embedded examples");
        assert_eq!(examples.len(), 17, "one example per use case");
        let mut wrong = Vec::new();
        for example in &examples {
            assert!(
                runs.contains(&example["args"]),
                "example is no recorded case step: {}",
                example["command"]
            );
            let made = call(&root, example, &catalog);
            let documented = json!({"name":example["tool"],"arguments":example["call"]});
            if made.as_ref() != Some(&documented) {
                wrong.push(format!("{}: {:?}", example["label"], made.map(|call| format!(
                    "Typed tool: `{}` `{}`.",
                    call["name"].as_str().unwrap_or_default(),
                    call["arguments"]
                ))));
            }
        }
        assert!(wrong.is_empty(), "documented typed calls differ:\n{}", wrong.join("\n"));
        let typed = lines(context, true).expect("typed lines");
        assert_eq!(typed.lines().count(), 18, "every example fits the typed catalog");
    }
}
