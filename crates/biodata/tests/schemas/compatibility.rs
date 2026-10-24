use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

type Check<T> = Result<T, String>;

#[derive(Debug, Default)]
struct Shape {
    types: BTreeSet<String>,
    values: Option<BTreeSet<String>>,
    required: BTreeSet<String>,
    minimum_length: u64,
    closed: bool,
}

pub(super) fn check(old: &Value, new: &Value) -> Check<()> {
    let old_shapes = shapes(old)?;
    let new_shapes = shapes(new)?;
    let (old_name, old_major) = identity(old)?;
    let (new_name, new_major) = identity(new)?;
    if old_name != new_name || new_major < old_major {
        return Err("schema identity changed or major version decreased".into());
    }
    if new_major > old_major {
        return Ok(());
    }
    for (path, old) in old_shapes {
        let new = new_shapes
            .get(&path)
            .ok_or_else(|| format!("removed {path}"))?;
        if old.types != new.types
            || !new.required.is_subset(&old.required)
            || new.minimum_length > old.minimum_length
            || (!old.closed && new.closed)
            || match (&old.values, &new.values) {
                (Some(old), Some(new)) => !old.is_subset(new),
                (None, Some(_)) => true,
                _ => false,
            }
        {
            return Err(format!("incompatible contract at {path}"));
        }
    }
    Ok(())
}

fn identity(schema: &Value) -> Check<(&str, u64)> {
    let id = schema["$id"].as_str().ok_or("missing schema $id")?;
    let (name, major) = id.rsplit_once('/').ok_or("missing major version")?;
    Ok((name, major.parse().map_err(|_| "invalid major version")?))
}

fn shapes(root: &Value) -> Check<BTreeMap<String, Shape>> {
    let mut shapes = BTreeMap::new();
    visit(root, root, "", &mut shapes, &mut BTreeSet::new())?;
    Ok(shapes)
}

fn visit(
    root: &Value,
    node: &Value,
    path: &str,
    shapes: &mut BTreeMap<String, Shape>,
    refs: &mut BTreeSet<String>,
) -> Check<()> {
    let object = node
        .as_object()
        .ok_or_else(|| format!("unsupported schema at {path}"))?;
    for key in object.keys() {
        if ![
            "$schema",
            "$id",
            "$defs",
            "$ref",
            "title",
            "description",
            "type",
            "enum",
            "properties",
            "required",
            "additionalProperties",
            "anyOf",
            "minLength",
        ]
        .contains(&key.as_str())
        {
            return Err(format!("unsupported keyword {key} at {path}"));
        }
    }
    if let Some(reference) = node.get("$ref") {
        let reference = reference.as_str().ok_or("invalid reference")?;
        let pointer = reference
            .strip_prefix("#/$defs/")
            .map(|_| &reference[1..])
            .ok_or_else(|| format!("unsupported reference {reference}"))?;
        if !refs.insert(reference.into()) {
            return Err(format!("recursive reference {reference}"));
        }
        visit(
            root,
            root.pointer(pointer)
                .ok_or_else(|| format!("unresolved {reference}"))?,
            path,
            shapes,
            refs,
        )?;
        refs.remove(reference);
    }
    if let Some(branches) = node.get("anyOf") {
        for branch in branches.as_array().ok_or("invalid anyOf")? {
            visit(root, branch, path, shapes, refs)?;
        }
    }
    collect(node, path, shapes)?;
    if let Some(properties) = node.get("properties") {
        for (name, child) in properties.as_object().ok_or("invalid properties")? {
            let name = name.replace('~', "~0").replace('/', "~1");
            visit(root, child, &format!("{path}/{name}"), shapes, refs)?;
        }
    }
    Ok(())
}

fn collect(node: &Value, path: &str, shapes: &mut BTreeMap<String, Shape>) -> Check<()> {
    let shape = shapes.entry(path.into()).or_default();
    if let Some(kind) = node.get("type") {
        let kinds = kind
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![kind.clone()]);
        for kind in kinds {
            let kind = kind.as_str().ok_or("invalid type")?;
            if !["string", "object", "null"].contains(&kind) {
                return Err(format!("unsupported type {kind} at {path}"));
            }
            shape.types.insert(kind.into());
        }
    }
    if let Some(values) = node.get("enum") {
        shape.values = Some(
            values
                .as_array()
                .ok_or("invalid enum")?
                .iter()
                .map(Value::to_string)
                .collect(),
        );
    }
    if let Some(required) = node.get("required") {
        for name in required.as_array().ok_or("invalid required")? {
            shape
                .required
                .insert(name.as_str().ok_or("invalid required name")?.into());
        }
    }
    if let Some(minimum) = node.get("minLength") {
        shape.minimum_length = minimum.as_u64().ok_or("invalid minLength")?;
    }
    if let Some(additional) = node.get("additionalProperties") {
        shape.closed = !additional
            .as_bool()
            .ok_or("unsupported additionalProperties")?;
    }
    Ok(())
}
