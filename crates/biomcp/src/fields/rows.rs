//! Row lists of a reply schema: a search's or subcommand's `result.items`, or the lists
//! inside the data of the sections a get names (`Section_*`), with the names each row offers.
use serde_json::Value;
use std::collections::BTreeSet;

/// How deep a dotted path may reach inside a row, and how far a schema walk may go.
const PATH_DEPTH: usize = 4;
pub(super) const WALK_DEPTH: usize = 24;

/// One step from the reply root towards a row list.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Step {
    Key(String),
    Each,
}

/// The names a row offers and those it always keeps.
#[derive(Clone, Debug, Default)]
pub(super) struct Row {
    /// Top-level names.
    pub(super) names: BTreeSet<String>,
    /// Dotted paths inside the row's objects and application documents.
    pub(super) paths: BTreeSet<String>,
    /// Names and dotted paths kept whenever their object is: required and identifier keys.
    pub(super) kept: BTreeSet<String>,
}

impl Row {
    /// Whether the row offers this name or dotted path.
    pub(super) fn knows(&self, name: &str) -> bool {
        self.names.contains(name) || self.paths.contains(name)
    }
    /// Top-level names kept whatever is chosen.
    pub(super) fn required(&self) -> impl Iterator<Item = &String> {
        self.kept.iter().filter(|name| !name.contains('.'))
    }
    /// The first dotted path that ends in this name.
    pub(super) fn inner(&self, name: &str) -> Option<&String> {
        self.paths
            .iter()
            .find(|path| path.rsplit('.').next() == Some(name))
    }
    pub(super) fn extend(&mut self, other: Self) {
        self.names.extend(other.names);
        self.paths.extend(other.paths);
        self.kept.extend(other.kept);
    }
}

/// One list of rows in a reply.
#[derive(Clone, Debug)]
pub(super) struct List {
    pub(super) at: Vec<Step>,
    pub(super) row: Row,
}

/// What a call asks for: its command path and its section words, normalized.
#[derive(Clone, Debug, Default)]
pub(super) struct Call {
    pub(super) verb: String,
    pub(super) entity: String,
    pub(super) words: Vec<String>,
}

impl Call {
    pub(super) fn of(verb: &str, entity: &str, words: &[String]) -> Self {
        Self {
            verb: verb.into(),
            entity: entity.into(),
            words: words.iter().map(|word| key(word)).collect(),
        }
    }

    pub(super) fn new(request: &biomcp_types::Request) -> Self {
        let (verb, entity) = request.path();
        let words = serde_json::to_value(request)
            .ok()
            .and_then(|value| value.get("section").cloned())
            .map(|section| match section {
                Value::String(word) => vec![word],
                Value::Array(words) => words
                    .into_iter()
                    .filter_map(|word| word.as_str().map(str::to_owned))
                    .collect(),
                _ => Vec::new(),
            })
            .unwrap_or_default();
        Self::of(verb, entity, &words)
    }
}

pub(super) fn key(word: &str) -> String {
    word.to_ascii_lowercase().replace('-', "_")
}

pub(super) fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    node.get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| root.pointer(reference.trim_start_matches('#')))
        .map_or(node, |target| resolve(target, root))
}

/// A node and its variant branches, each resolved.
fn branches<'a>(node: &'a Value, root: &'a Value, out: &mut Vec<&'a Value>, depth: usize) {
    let node = resolve(node, root);
    out.push(node);
    if depth >= WALK_DEPTH {
        return;
    }
    for key in ["anyOf", "oneOf", "allOf"] {
        for branch in node
            .get(key)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            branches(branch, root, out, depth + 1);
        }
    }
}

pub(super) fn variants<'a>(node: &'a Value, root: &'a Value) -> Vec<&'a Value> {
    let mut out = Vec::new();
    branches(node, root, &mut out, 0);
    out
}

/// The item schema of a list property, through nullable or variant wrappers.
pub(super) fn list_items<'a>(node: &'a Value, root: &'a Value) -> Option<&'a Value> {
    variants(node, root)
        .into_iter()
        .find_map(|branch| branch.get("items").filter(|items| items.is_object()))
}

/// Whether an item schema describes objects.
pub(super) fn objects(node: &Value, root: &Value) -> bool {
    variants(node, root)
        .iter()
        .any(|branch| branch.get("properties").is_some_and(Value::is_object))
}

/// Whether a node is an identifier object, directly or as a nullable variant.
/// Identifier keys, and flags marked `x-kept` that change what the row means (1065).
fn identifier(node: &Value, root: &Value) -> bool {
    variants(node, root).iter().any(|branch| {
        ["x-identifier", "x-kept"]
            .iter()
            .any(|key| branch.get(*key) == Some(&Value::Bool(true)))
    })
}

fn join(prefix: &str, name: &str) -> String {
    if prefix.is_empty() {
        name.to_owned()
    } else {
        format!("{prefix}.{name}")
    }
}

/// Collect a row's names, inner paths and kept keys across its variants.
fn collect(node: &Value, root: &Value, row: &mut Row, prefix: &str, level: usize) {
    for branch in variants(node, root) {
        if let Some(fields) = branch.get("properties").and_then(Value::as_object) {
            for (name, property) in fields {
                let path = join(prefix, name);
                if prefix.is_empty() {
                    row.names.insert(name.clone());
                } else {
                    row.paths.insert(path.clone());
                }
                if identifier(property, root) {
                    row.kept.insert(path.clone());
                }
                if level < PATH_DEPTH {
                    match list_items(property, root) {
                        // Approval rows can list document IDs without the rest of each link.
                        Some(items) if path == "application_docs" => {
                            collect(items, root, row, &path, level + 1);
                        }
                        Some(_) => {}
                        None => collect(property, root, row, &path, level + 1),
                    }
                }
            }
        }
        row.kept.extend(
            branch
                .get("required")
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
                .filter_map(Value::as_str)
                .map(|name| join(prefix, name)),
        );
    }
}

pub(super) fn row_of(items: &Value, root: &Value) -> Row {
    let mut row = Row::default();
    collect(items, root, &mut row, "", 1);
    row
}

/// The `result.items` rows of a reply schema, across its result variants.
pub(super) fn items(schema: &Value) -> Vec<List> {
    let Some(result) = schema.pointer("/properties/result") else {
        return Vec::new();
    };
    let mut row = Row::default();
    let mut found = false;
    for body in variants(result, schema) {
        if let Some(item) = body
            .pointer("/properties/items")
            .and_then(|list| list_items(list, schema))
            .filter(|item| objects(item, schema))
        {
            row.extend(row_of(item, schema));
            found = true;
        }
    }
    let at = vec![
        Step::Key("result".into()),
        Step::Key("items".into()),
        Step::Each,
    ];
    if found {
        vec![List { at, row }]
    } else {
        Vec::new()
    }
}
