//! Project only protocol-owned Steps; upstream text and query values remain data.
use serde_json::Value;

pub(super) fn project(root: &crate::grammar::Root, value: &mut Value) {
    match value {
        Value::Object(fields) => {
            if let Some(next) = fields.get_mut("next").and_then(Value::as_array_mut) {
                *next = next
                    .iter()
                    .filter_map(|step| {
                        let args = step.get("args")?.as_array()?;
                        let args = without_globals(root, args)?;
                        let mut call = super::summary::translate(root, &args)?;
                        if let Some(kind) = step.get("kind") {
                            call.as_object_mut()?.insert("kind".into(), kind.clone());
                        }
                        Some(call)
                    })
                    .collect();
            }
            for (key, node) in fields {
                if matches!(key.as_str(), "error" | "problems") {
                    project(root, node);
                }
            }
        }
        Value::Array(nodes) => {
            for node in nodes {
                project(root, node);
            }
        }
        _ => {}
    }
}
fn without_globals(root: &crate::grammar::Root, args: &[Value]) -> Option<Vec<Value>> {
    let mut result = Vec::new();
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let text = word.as_str()?;
        if text == "--" {
            result.push(word.clone());
            result.extend(words.cloned());
            break;
        }
        let flag = text
            .strip_prefix("--")
            .unwrap_or_default()
            .split('=')
            .next()?;
        if let Some(global) = root
            .get_arguments()
            .find(|arg| arg.get_long() == Some(flag))
        {
            if global.get_action().takes_values() && !text.contains('=') {
                words.next()?;
            }
        } else {
            result.push(word.clone());
        }
    }
    Some(result)
}
