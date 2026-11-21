//! Locate named sections and their row lists in reply schemas.
use super::rows::{
    Call, List, Step, WALK_DEPTH, items, key, list_items, objects, resolve, row_of, variants,
};
use serde_json::Value;

/// Every section under a node, by its path; a type already on the walk is not entered again.
struct Walk<'a> {
    root: &'a Value,
    found: Vec<(Vec<Step>, &'a Value)>,
    entered: Vec<&'a str>,
}

impl<'a> Walk<'a> {
    fn sections(&mut self, node: &'a Value, at: &[Step]) {
        let reference = node.get("$ref").and_then(Value::as_str);
        if reference.is_some_and(|reference| reference.starts_with("#/$defs/Reply_Section_")) {
            self.found.push((at.to_vec(), resolve(node, self.root)));
            return;
        }
        if self.entered.len() >= WALK_DEPTH
            || reference.is_some_and(|reference| self.entered.contains(&reference))
        {
            return;
        }
        if let Some(reference) = reference {
            self.entered.push(reference);
        }
        self.inside(resolve(node, self.root), at);
        if reference.is_some() {
            self.entered.pop();
        }
    }

    fn inside(&mut self, node: &'a Value, at: &[Step]) {
        for key in ["anyOf", "oneOf", "allOf"] {
            for branch in node
                .get(key)
                .and_then(Value::as_array)
                .into_iter()
                .flatten()
            {
                self.sections(branch, at);
            }
        }
        if let Some(items) = node.get("items").filter(|items| items.is_object()) {
            self.sections(items, &[at, &[Step::Each]].concat());
        }
        for (name, property) in node
            .get("properties")
            .and_then(Value::as_object)
            .into_iter()
            .flatten()
        {
            self.sections(property, &[at, &[Step::Key(name.clone())]].concat());
        }
    }
}

/// The row lists inside one section's data: the data itself when it is a list of objects,
/// or each of its own lists of objects.
fn section_lists(section: &Value, root: &Value, at: &[Step]) -> Vec<List> {
    let mut lists = Vec::new();
    let data = variants(section, root)
        .into_iter()
        .filter_map(|branch| branch.pointer("/properties/data"))
        .collect::<Vec<_>>();
    for data in data {
        let at = [at, &[Step::Key("data".into())]].concat();
        if let Some(item) = list_items(data, root).filter(|item| objects(item, root)) {
            lists.push(List {
                at: [at.as_slice(), &[Step::Each]].concat(),
                row: row_of(item, root),
            });
            continue;
        }
        for branch in variants(data, root) {
            for (name, property) in branch
                .get("properties")
                .and_then(Value::as_object)
                .into_iter()
                .flatten()
            {
                if let Some(item) = list_items(property, root).filter(|item| objects(item, root)) {
                    lists.push(List {
                        at: [at.as_slice(), &[Step::Key(name.clone()), Step::Each]].concat(),
                        row: row_of(item, root),
                    });
                }
            }
        }
    }
    lists
}

/// The schema's name without its namespace and version, such as `drug-approvals`.
fn stem(schema: &Value) -> &str {
    let id = schema
        .get("$id")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let id = id.split('/').next().unwrap_or_default();
    id.split_once('.').map_or(id, |(_, stem)| stem)
}

/// The row lists of one reply schema that a call reaches: a get's named sections, the
/// section a reply of its own serves, or the rows of a search or subcommand.
pub(super) fn lists(schema: &Value, call: &Call) -> Vec<List> {
    let mut extra = super::section_rows::standalone(schema, call, stem(schema));
    if call.verb != "get" {
        if extra.is_empty() {
            return items(schema);
        }
        return extra;
    }
    if call.words.is_empty() {
        return Vec::new();
    }
    let mut walk = Walk {
        root: schema,
        found: Vec::new(),
        entered: Vec::new(),
    };
    if let Some(result) = schema.pointer("/properties/result") {
        walk.sections(result, &[Step::Key("result".into())]);
    }
    let found = walk.found;
    extra.extend(named_lists(schema, call, &found));
    extra
}

fn named_lists(schema: &Value, call: &Call, found: &[(Vec<Step>, &Value)]) -> Vec<List> {
    let named = |at: &[Step]| {
        matches!(at.last(), Some(Step::Key(name))
            if call.words.iter().any(|word| *word == key(name) || word == "all"))
    };
    let chosen = found.iter().filter(|(at, _)| named(at)).collect::<Vec<_>>();
    let own = call
        .words
        .iter()
        .any(|word| stem(schema) == format!("{}_{word}", key(&call.entity)).replace('_', "-"));
    let chosen = if chosen.is_empty() && own {
        found.iter().collect()
    } else {
        chosen
    };
    if chosen.is_empty() {
        return if own && found.is_empty() {
            items(schema)
        } else {
            Vec::new()
        };
    }
    chosen
        .into_iter()
        .flat_map(|(at, section)| {
            let lists = section_lists(section, schema, at);
            if lists.is_empty() {
                super::section_rows::payload(schema, call, at, section)
            } else {
                lists
            }
        })
        .collect()
}
