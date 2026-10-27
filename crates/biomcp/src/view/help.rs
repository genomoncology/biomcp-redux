//! Command help uses headings and copyable shell examples.
use super::{escape, table};
use serde_json::Value;

pub(super) fn render(value: &Value, out: &mut String) {
    for command in value
        .pointer("/result/commands")
        .and_then(Value::as_array)
        .into_iter()
        .flatten()
    {
        let path = command["command"]
            .as_array()
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .collect::<Vec<_>>()
            .join(" ");
        out.push_str(&format!(
            "# biomcp {}\n\n{}\n\n",
            escape(&path),
            escape(command["description"].as_str().unwrap_or_default())
        ));
        if let Some(arguments) = command["arguments"].as_array().filter(|a| !a.is_empty()) {
            out.push_str("## Arguments\n\n");
            table(arguments, &["name", "description"], false, out);
        }
        if let Some(rule) = command["section_rule"].as_str() {
            out.push_str(&format!("{}\n\n", escape(rule)));
        }
        if let Some(sections) = command["sections"].as_array().filter(|a| !a.is_empty()) {
            out.push_str("## Sections\n\n");
            table(sections, &["word", "description"], false, out);
        }
        for example in command["examples"].as_array().into_iter().flatten() {
            let args = example
                .as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str);
            if let Ok(line) = shlex::try_join(args) {
                out.push_str(&format!("```sh\nbiomcp {line}\n```\n\n"));
            }
        }
    }
}
