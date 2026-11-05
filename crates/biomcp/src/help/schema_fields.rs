//! Resolve schema references and nullable argument types.
use serde_json::Value;
pub(super) fn value_type(node: &Value) -> String {
    if let Some(kind) = node.get("type") {
        return kind.as_str().map(str::to_owned).unwrap_or_else(|| {
            kind.as_array()
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .filter(|v| *v != "null")
                .collect::<Vec<_>>()
                .join("|")
        });
    }
    if node.get("enum").is_some() {
        "string"
    } else {
        "object"
    }
    .into()
}
pub(super) fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    if let Some(reference) = node
        .get("$ref")
        .and_then(Value::as_str)
        .and_then(|s| root.pointer(s.trim_start_matches('#')))
    {
        return resolve(reference, root);
    }
    if let Some(branch) = ["anyOf", "oneOf", "allOf"]
        .iter()
        .find_map(|key| node.get(key).and_then(Value::as_array))
        .and_then(|branches| {
            branches
                .iter()
                .find(|v| v.get("type") != Some(&Value::from("null")))
        })
    {
        return resolve(branch, root);
    }
    node
}
