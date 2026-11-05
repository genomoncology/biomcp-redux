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

#[cfg(test)]
mod tests {
    use super::Value;

    #[test]
    fn every_section_rule_keeps_its_complete_source_text() -> Result<(), Box<dyn std::error::Error>>
    {
        let root = crate::grammar::root().map_err(|_| "grammar unavailable")?;
        let commands = crate::help::commands(&root)?;
        let paths = crate::grammar::command_paths();
        for command in &commands {
            let Some(actual) = &command.section_rule else {
                continue;
            };
            let (_, _, id) = paths
                .iter()
                .find(|(verb, entity, _)| command.command == [*verb, *entity])
                .ok_or("section rule has no schema owner")?;
            let schema: Value =
                serde_json::from_str(crate::schema::find(id).ok_or("missing schema")?)?;
            let section = schema
                .pointer("/properties/section")
                .ok_or("missing section")?;
            let original = section
                .get("x-section-rule")
                .and_then(Value::as_str)
                .ok_or_else(|| format!("{id}: missing canonical section rule"))?;
            assert_eq!(actual, &crate::help::discovery::plain(original), "{id}");
            assert!(
                !actual.contains("biomcp_help"),
                "{id}: compact guidance leaked into rule"
            );
        }
        Ok(())
    }
}
