//! Render typed Markdown strings once, using the same checked response schema as JSON.
use serde_json::Value;
use std::io;
pub(crate) fn markdown(value: &Value) -> io::Result<Vec<u8>> {
    let schema = value.get("schema").and_then(Value::as_str).and_then(crate::schema::find)
        .ok_or_else(|| io::Error::other("missing response schema"))?;
    let root: Value = serde_json::from_str(schema)?;
    let mut out = String::new();
    render(value, &root, &root, 1, &mut out);
    Ok(out.into_bytes())
}
fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    node.get("$ref").and_then(Value::as_str).and_then(|r| r.strip_prefix('#'))
        .and_then(|p| root.pointer(p)).map_or(node, |v| resolve(v, root))
}
fn property<'a>(node: &'a Value, name: &str, root: &'a Value) -> Option<&'a Value> {
    let node = resolve(node, root);
    node.get("properties").and_then(|p|p.get(name)).or_else(|| {
        ["anyOf", "oneOf"].into_iter().filter_map(|k|node.get(k).and_then(Value::as_array))
            .flatten().find_map(|n|property(n,name,root))
    })
}
fn render(value: &Value, node: &Value, root: &Value, depth: usize, out: &mut String) {
    let node = resolve(node, root);
    match value {
        Value::Object(fields) => {
            for (name, value) in fields {
                out.push_str(&format!("{} {}\n\n", "#".repeat(depth.min(6)), name.replace('_', " ")));
                render(value, property(node, name, root).unwrap_or(&Value::Null), root, depth+1, out);
            }
        }
        Value::Array(items) => {
            for value in items { render(value, node.get("items").unwrap_or(&Value::Null), root, depth, out); }
        }
        Value::String(text) => {
            let formatted = if marked(node, root) { text.clone() }
                else { text.replace('\\', "\\\\").replace('*', "\\*").replace('_', "\\_").replace('`', "\\`") };
            out.push_str(&formatted); out.push_str("\n\n");
        }
        _ => { out.push_str(&value.to_string()); out.push_str("\n\n"); }
    }
}

fn marked(node: &Value, root: &Value) -> bool {
    let node = resolve(node, root);
    node.get("format").and_then(Value::as_str) == Some("markdown") || ["anyOf", "oneOf"].into_iter()
        .filter_map(|k|node.get(k).and_then(Value::as_array)).flatten().any(|n|marked(n,root))
}
