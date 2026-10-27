use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

type Check<T> = Result<T, String>;

#[derive(Debug, Default)]
struct Shape {
    types: BTreeSet<String>,
    values: Option<BTreeSet<String>>,
    required: BTreeSet<String>,
    selectors: Option<Vec<BTreeSet<String>>>,
    minimum_length: u64,
    minimum_items: u64,
    unique_items: bool,
    maximum_items: Option<u64>,
    pattern: Option<String>,
    minimum: Option<Bound>,
    maximum: Option<Bound>,
    closed: bool,
    markdown: bool,
    impossible: bool,
    map_values: bool,
    map_keys: bool,
    map_patterns: BTreeSet<String>,
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
        if old.map_patterns != new.map_patterns
            || old.impossible != new.impossible
            || new.map_values && !old.map_values
            || new.map_keys && !old.map_keys
            || old.markdown && !new.markdown
            || old.pattern != new.pattern
            || new
                .minimum
                .is_some_and(|bound| old.minimum.is_none_or(|old| bound > old))
            || new
                .maximum
                .is_some_and(|bound| old.maximum.is_none_or(|old| bound < old))
            || new.selectors.as_ref().is_some_and(|new| {
                old.selectors.as_ref().is_none_or(|old| {
                    old.iter()
                        .any(|old| !new.iter().any(|new| new.is_subset(old)))
                })
            })
            || old.types != new.types
            || !new.required.is_subset(&old.required)
            || new.minimum_length > old.minimum_length
            || new.minimum_items > old.minimum_items
            || new.unique_items && !old.unique_items
            || new
                .maximum_items
                .is_some_and(|bound| old.maximum_items.is_none_or(|old| bound < old))
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
    if let Some(boolean) = node.as_bool() {
        shapes.entry(path.into()).or_default().impossible = !boolean;
        return Ok(());
    }
    if let Some(negated) = node.get("not") {
        let object = node.as_object().ok_or("invalid negation")?;
        if negated.as_object().is_none_or(|value| !value.is_empty())
            || object
                .keys()
                .any(|key| !["not", "title", "description"].contains(&key.as_str()))
        {
            return Err("unsupported negation".into());
        }
        shapes.entry(path.into()).or_default().impossible = true;
        return Ok(());
    }
    let object = node
        .as_object()
        .ok_or_else(|| format!("unsupported schema at {path}"))?;
    validate_keywords(object, path)?;
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
    if let Some(branches) = node.get("anyOf")
        && selectors(node)?.is_none()
    {
        if let Ok(branches) = optional_branches(branches) {
            for branch in branches {
                visit(root, branch, path, shapes, refs)?;
            }
        } else {
            for (kind, branch) in scalar_branches(root, branches)? {
                visit(root, branch, &format!("{path}/@type={kind}"), shapes, refs)?;
            }
        }
    }
    if let Some(branches) = node.get("oneOf") {
        for (tag, branch) in tagged_branches(branches)? {
            visit(root, branch, &format!("{path}/@{tag}"), shapes, refs)?;
        }
    }
    collect(node, path, shapes)?;
    visit_children(root, node, path, shapes, refs)
}

fn visit_children(
    root: &Value,
    node: &Value,
    path: &str,
    shapes: &mut BTreeMap<String, Shape>,
    refs: &mut BTreeSet<String>,
) -> Check<()> {
    if let Some(patterns) = node.get("patternProperties") {
        for (pattern, child) in patterns.as_object().ok_or("invalid patternProperties")? {
            let key = pattern.replace('~', "~0").replace('/', "~1");
            visit(
                root,
                child,
                &format!("{path}/{{pattern={key}}}"),
                shapes,
                refs,
            )?;
        }
    }
    for (field, suffix) in [
        ("additionalProperties", "{value}"),
        ("propertyNames", "{key}"),
    ] {
        if let Some(child) = node
            .get(field)
            .filter(|value| field == "propertyNames" || !value.is_boolean())
        {
            visit(root, child, &format!("{path}/{suffix}"), shapes, refs)?;
        }
    }
    if let Some(items) = node.get("items") {
        visit(root, items, &format!("{path}/[]"), shapes, refs)?;
    }
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
    if let Some(selectors) = selectors(node)? {
        shape.selectors = Some(selectors);
    }
    if let Some(kind) = node.get("type") {
        let kinds = kind
            .as_array()
            .cloned()
            .unwrap_or_else(|| vec![kind.clone()]);
        for kind in kinds {
            let kind = kind.as_str().ok_or("invalid type")?;
            if ![
                "string", "object", "null", "integer", "array", "boolean", "number",
            ]
            .contains(&kind)
            {
                return Err(format!("unsupported type {kind} at {path}"));
            }
            shape.types.insert(kind.into());
        }
    }
    if let Some(unique) = node.get("uniqueItems") {
        shape.unique_items |= unique.as_bool().ok_or("invalid uniqueItems")?;
    }
    if let Some(pattern) = node.get("pattern") {
        shape.pattern = Some(pattern.as_str().ok_or("invalid pattern")?.into());
    }
    if let Some(minimum) = node.get("minimum") {
        shape.minimum = Some(bound(node, minimum)?);
    }
    if let Some(maximum) = node.get("maximum") {
        shape.maximum = Some(bound(node, maximum)?);
    }
    if node.get("format").is_some_and(|format| {
        format != "uint32"
            && format != "uint64"
            && format != "int64"
            && format != "double"
            && format != "markdown"
    }) {
        return Err("unsupported format".into());
    }
    shape.markdown |= node.get("format").is_some_and(|v| v == "markdown");
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
    if let Some(value) = node.get("const") {
        if !value.is_string() {
            return Err("unsupported non-string const".into());
        }
        shape.values = Some(BTreeSet::from([value.to_string()]));
    }
    if let Some(required) = node.get("required") {
        for name in required.as_array().ok_or("invalid required")? {
            shape
                .required
                .insert(name.as_str().ok_or("invalid required name")?.into());
        }
    }
    if let Some(bound) = node.get("minItems") {
        shape.minimum_items = bound.as_u64().ok_or("invalid minItems")?;
    }
    if let Some(bound) = node.get("maxItems") {
        shape.maximum_items = Some(bound.as_u64().ok_or("invalid maxItems")?);
    }
    if let Some(minimum) = node.get("minLength") {
        shape.minimum_length = minimum.as_u64().ok_or("invalid minLength")?;
    }
    collect_map(node, shape)
}

fn collect_map(node: &Value, shape: &mut Shape) -> Check<()> {
    if let Some(additional) = node.get("additionalProperties") {
        shape.closed = additional == &Value::Bool(false);
        shape.map_values = additional.is_object();
    }
    shape.map_keys = node.get("propertyNames").is_some();
    if let Some(patterns) = node.get("patternProperties") {
        shape.map_patterns = patterns
            .as_object()
            .ok_or("invalid patternProperties")?
            .keys()
            .cloned()
            .collect();
    }
    Ok(())
}

// The only object-level union permits at least one named selector.
pub(super) fn selectors(node: &Value) -> Check<Option<Vec<BTreeSet<String>>>> {
    if node["type"] != "object" || node.get("anyOf").is_none() {
        return Ok(None);
    }
    let properties = node["properties"]
        .as_object()
        .ok_or("missing selector properties")?;
    let branches = node["anyOf"].as_array().ok_or("invalid selector anyOf")?;
    if branches.is_empty() {
        return Err("empty selector anyOf".into());
    }
    let mut result = Vec::new();
    for branch in branches {
        let object = branch.as_object().ok_or("invalid selector branch")?;
        if object.len() != 1 {
            return Err("selector branches contain only required".into());
        }
        let names = branch["required"]
            .as_array()
            .ok_or("missing selector required")?;
        let mut required = BTreeSet::new();
        for name in names {
            let name = name.as_str().ok_or("invalid selector name")?;
            if !properties.contains_key(name) || !required.insert(name.to_owned()) {
                return Err("unknown or repeated selector".into());
            }
        }
        if required.is_empty() || result.contains(&required) {
            return Err("empty or repeated selector branch".into());
        }
        result.push(required);
    }
    Ok(Some(result))
}

// Nullable generated references and strings remain distinct from tagged unions.
fn optional_branches(branches: &Value) -> Check<&[Value]> {
    let branches = branches.as_array().ok_or("invalid anyOf")?;
    if branches.len() != 2
        || branches
            .iter()
            .filter(|branch| **branch == serde_json::json!({"type":"null"}))
            .count()
            != 1
    {
        return Err("unsupported anyOf: expected one value and null".into());
    }
    let value = branches
        .iter()
        .find(|branch| branch["type"] != "null")
        .ok_or("missing optional value")?;
    if value.get("$ref").is_none() && value["type"] != "string" {
        return Err("unsupported anyOf value".into());
    }
    Ok(branches)
}

// The scalar representation permits exactly two disjoint JSON types. Keep
// their paths separate so constraints in one branch cannot hide the other.
fn scalar_branches<'a>(root: &Value, branches: &'a Value) -> Check<Vec<(String, &'a Value)>> {
    let branches = branches.as_array().ok_or("invalid scalar anyOf")?;
    if branches.len() != 2 {
        return Err("unsupported scalar union".into());
    }
    let mut kinds = BTreeSet::new();
    let mut result = Vec::new();
    for branch in branches {
        let resolved = if let Some(reference) = branch.get("$ref") {
            root.pointer(
                reference
                    .as_str()
                    .and_then(|r| r.strip_prefix('#'))
                    .filter(|r| r.starts_with("/$defs/"))
                    .ok_or("unsupported scalar reference")?,
            )
            .ok_or("unresolved scalar reference")?
        } else {
            branch
        };
        let kind = resolved["type"].as_str().ok_or("missing scalar type")?;
        if !["number", "object"].contains(&kind) || !kinds.insert(kind) {
            return Err("unsupported scalar union".into());
        }
        result.push((kind.to_owned(), branch));
    }
    Ok(result)
}

// These unions are internally tagged objects. Branch identity includes both
// discriminator field and value, so another branch cannot hide a removal.
fn tagged_branches(branches: &Value) -> Check<Vec<(String, &Value)>> {
    let mut result = Vec::new();
    let mut tags = BTreeSet::new();
    let mut discriminator = None;
    for branch in branches.as_array().ok_or("invalid oneOf")? {
        if branch["type"] != "object" {
            return Err("unsupported oneOf branch".into());
        }
        let properties = branch["properties"]
            .as_object()
            .ok_or("missing tagged properties")?;
        let candidates: Vec<_> = properties
            .iter()
            .filter_map(|(key, value)| {
                value
                    .get("const")
                    .and_then(Value::as_str)
                    .map(|value| (key, value))
            })
            .collect();
        let [(field, value)] = candidates.as_slice() else {
            return Err("expected one discriminator".into());
        };
        if !branch["required"]
            .as_array()
            .is_some_and(|required| required.contains(&Value::String((*field).clone())))
        {
            return Err("optional discriminator".into());
        }
        if discriminator.is_some_and(|old| old != field.as_str()) {
            return Err("inconsistent discriminator".into());
        }
        discriminator = Some(field.as_str());
        let tag = format!("{field}={value}");
        if !tags.insert(tag.clone()) {
            return Err("duplicate discriminator".into());
        }
        result.push((tag, branch));
    }
    if result.is_empty() {
        return Err("empty tagged union".into());
    }
    Ok(result)
}

fn validate_keywords(object: &serde_json::Map<String, Value>, path: &str) -> Check<()> {
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
            "propertyNames",
            "patternProperties",
            "anyOf",
            "oneOf",
            "const",
            "items",
            "minLength",
            "pattern",
            "minimum",
            "maximum",
            "format",
            "default",
            "minItems",
            "maxItems",
            "uniqueItems",
            "x-enum-descriptions",
            "x-positional",
            "x-trailing-section",
            "x-comma-separated",
            "x-cli-flags",
            "x-reply-query-titles",
        ]
        .contains(&key.as_str())
        {
            return Err(format!("unsupported keyword {key} at {path}"));
        }
    }
    validate_annotations(object)?;
    // These generated forms carry annotations beside a reference or union,
    // never assertions. Assertions would intersect, while Shape only records
    // one constraint at each path; refuse them instead of merging incorrectly.
    for composition in ["$ref", "anyOf", "oneOf"] {
        if object.contains_key(composition)
            && !(composition == "anyOf" && selectors(&Value::Object(object.clone()))?.is_some())
            && object.keys().any(|key| {
                key != composition
                    && ![
                        "$schema",
                        "$id",
                        "$defs",
                        "title",
                        "description",
                        "x-enum-descriptions",
                        "x-positional",
                        "x-trailing-section",
                        "x-comma-separated",
                        "x-cli-flags",
                        "x-reply-query-titles",
                        "default",
                    ]
                    .contains(&key.as_str())
            })
        {
            return Err(format!(
                "unsupported assertion sibling of {composition} at {path}"
            ));
        }
    }
    Ok(())
}

fn validate_annotations(object: &serde_json::Map<String, Value>) -> Check<()> {
    validate_cli_flags(object.get("x-cli-flags"))?;
    if object.get("x-reply-query-titles").is_some_and(|value| {
        !value.as_array().is_some_and(|titles| {
            !titles.is_empty()
                && titles
                    .iter()
                    .all(|title| title.as_str().is_some_and(|title| !title.is_empty()))
        })
    }) {
        return Err("invalid reply query title annotation".into());
    }
    if object
        .get("x-trailing-section")
        .is_some_and(|value| value != &Value::Bool(true))
    {
        return Err("unsupported trailing-section annotation".into());
    }
    if object
        .get("x-comma-separated")
        .is_some_and(|v| v != &Value::Bool(true))
    {
        return Err("unsupported comma-separated annotation".into());
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum Bound {
    Integer(i128),
    Decimal(f64),
}
fn bound(node: &Value, value: &Value) -> Check<Bound> {
    if node["type"] == "integer" {
        return value
            .as_i64()
            .map(i128::from)
            .or_else(|| value.as_u64().map(i128::from))
            .map(Bound::Integer)
            .ok_or_else(|| "unsupported integer bound".into());
    }
    value
        .as_f64()
        .filter(|value| value.is_finite())
        .map(Bound::Decimal)
        .ok_or_else(|| "unsupported decimal bound".into())
}

fn validate_cli_flags(flags: Option<&Value>) -> Check<()> {
    if let Some(flags) = flags {
        let flags = flags.as_object().ok_or("unsupported CLI flag annotation")?;
        if flags.is_empty()
            || flags.iter().any(|(name, value)| {
                !name.as_bytes().first().is_some_and(u8::is_ascii_lowercase)
                    || !name.bytes().all(|b| b.is_ascii_lowercase() || b == b'-')
                    || value.as_str().is_none_or(str::is_empty)
            })
        {
            return Err("unsupported CLI flag annotation".into());
        }
    }
    Ok(())
}
