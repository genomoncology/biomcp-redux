//! Chosen fields: one generic filter keeps named top-level keys of each search row.
//!
//! The row is `result.items[]` of the reply schema. Its required fields and the whole
//! envelope always stay, and so do identifier properties (`x-identifier`), so identifiers
//! and source attribution survive narrowing.
use biomcp_query::Action;
use biomcp_types::{ErrorMessage, Request, Source, Step, StepKind, UsageArguments, UsageError};
use serde_json::Value;
use std::{collections::BTreeSet, io};

/// Names a search row offers, and those it always keeps (required or identifier).
#[derive(Debug, Default)]
struct Row {
    names: BTreeSet<String>,
    required: BTreeSet<String>,
}

fn resolve<'a>(node: &'a Value, root: &'a Value) -> &'a Value {
    node.get("$ref")
        .and_then(Value::as_str)
        .and_then(|reference| root.pointer(reference.trim_start_matches('#')))
        .map_or(node, |target| resolve(target, root))
}

/// Whether a property is an identifier object, directly or as a nullable variant.
fn identifier(node: &Value, root: &Value) -> bool {
    let node = resolve(node, root);
    node.get("x-identifier") == Some(&Value::Bool(true))
        || node
            .get("anyOf")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .any(|branch| identifier(branch, root))
}

/// Collect a row's properties across its variants; a nullable wrapper adds none.
/// Identifier properties are always kept, like required ones, so an optional row
/// identity (such as an article's key) survives narrowing.
fn collect(node: &Value, root: &Value, row: &mut Row) {
    let node = resolve(node, root);
    if let Some(fields) = node.get("properties").and_then(Value::as_object) {
        row.names.extend(fields.keys().cloned());
        row.required.extend(
            fields
                .iter()
                .filter(|(_, property)| identifier(property, root))
                .map(|(name, _)| name.clone()),
        );
    }
    row.required.extend(
        node.get("required")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .filter_map(Value::as_str)
            .map(str::to_owned),
    );
    for key in ["anyOf", "oneOf", "allOf"] {
        for branch in node
            .get(key)
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
        {
            collect(branch, root, row);
        }
    }
}

/// The row of one reply schema, when its result carries `items`.
fn row(schema: &Value) -> Option<Row> {
    let mut items = Row::default();
    let result = resolve(schema.pointer("/properties/result")?, schema);
    let branches = std::iter::once(result).chain(
        result
            .get("anyOf")
            .and_then(Value::as_array)
            .into_iter()
            .flatten()
            .map(|branch| resolve(branch, schema)),
    );
    let mut found = false;
    for body in branches {
        if let Some(list) = body.pointer("/properties/items") {
            let list = resolve(list, schema);
            if let Some(item) = list.get("items") {
                collect(item, schema, &mut items);
                found = true;
            }
        }
    }
    found.then_some(items)
}

fn reply_row(id: &str) -> Option<Row> {
    let schema: Value = serde_json::from_str(crate::schema::find(id)?).ok()?;
    row(&schema)
}

/// Every row name any reply of this search can carry.
fn search_row(verb: &str, entity: &str) -> Row {
    let mut union = Row::default();
    let Some((_, _, id)) = crate::grammar::command_paths()
        .into_iter()
        .find(|path| (path.0, path.1) == (verb, entity))
    else {
        return union;
    };
    for reply in crate::schema::replies(id, verb, entity) {
        if let Some(row) = reply_row(&reply) {
            union.names.extend(row.names);
            union.required.extend(row.required);
        }
    }
    union
}

fn help_step(path: (&str, &str)) -> Step {
    Step {
        kind: StepKind::Configure,
        args: vec!["help".into(), path.0.into(), path.1.into()].into(),
    }
}

/// Refuse chosen fields outside an entity search, and names its rows do not carry.
pub(crate) fn check(parsed: &crate::grammar::Parsed) -> Result<(), UsageError> {
    let fields = &parsed.globals.fields;
    if fields.is_empty() || parsed.help.is_some() {
        return Ok(());
    }
    let (request, path) = match &parsed.action {
        Some(Action::Run { request, .. }) => (request, request.path()),
        _ => return Err(UsageError::argument("--fields".into())),
    };
    if path.0 != "search" {
        return Err(UsageError::Arguments(UsageArguments {
            argument: Some("--fields".into()),
            message: Some(biomcp_types::ErrorMessage::FieldsSearchOnly),
            next: vec![help_step(path)],
            ..Default::default()
        }));
    }
    let row = search_row(path.0, path.1);
    if !row.names.is_empty() && fields.iter().all(|name| row.names.contains(name)) {
        return source_check(parsed, request, path);
    }
    Err(UsageError::Arguments(UsageArguments {
        argument: Some("--fields".into()),
        valid_values: row
            .names
            .iter()
            .map(|name| biodata::Text::new(name).ok())
            .collect::<Option<Vec<_>>>()
            .map(Into::into),
        next: vec![help_step(path)],
        ..Default::default()
    }))
}

/// The source a search query selects, with the reply schema of its rows, when the
/// query alone fixes it.
fn selected(request: &Request) -> Option<(Source, &'static str)> {
    match request {
        Request::SearchTrial(query) => Some((
            query.source().source(),
            biomcp_query::trial::reply_schema(query.source()),
        )),
        _ => None,
    }
}

/// Rows of a search by source selector, when its sources return different rows.
fn source_rows(verb: &str, entity: &str) -> Vec<(&'static str, Row)> {
    if (verb, entity) != ("search", "trial") {
        return Vec::new();
    }
    biomcp_query::trial::SOURCES
        .iter()
        .filter_map(|source| {
            reply_row(biomcp_query::trial::reply_schema(*source)).map(|row| (source.as_str(), row))
        })
        .collect()
}

/// The one field of the source's row that contains the asked name or that it contains.
fn nearest<'a>(row: &'a Row, name: &str) -> Option<&'a String> {
    let mut near = row
        .names
        .iter()
        .filter(|field| field.contains(name) || name.contains(field.as_str()));
    near.next().filter(|_| near.next().is_none())
}

/// Refuse chosen fields that the selected source's rows cannot carry, naming that
/// source, its fields, and the same call with each such field moved to its nearest.
fn source_check(
    parsed: &crate::grammar::Parsed,
    request: &Request,
    path: (&str, &str),
) -> Result<(), UsageError> {
    let fields = &parsed.globals.fields;
    let Some((source, row)) =
        selected(request).and_then(|(source, id)| Some((source, reply_row(id)?)))
    else {
        return Ok(());
    };
    if fields.iter().all(|name| row.names.contains(name)) {
        return Ok(());
    }
    let mut chosen: Vec<&str> = Vec::new();
    for name in fields {
        let field = if row.names.contains(name) {
            Some(name)
        } else {
            nearest(&row, name)
        };
        if let Some(field) = field.filter(|field| !chosen.contains(&field.as_str())) {
            chosen.push(field.as_str());
        }
    }
    let mut next = Vec::new();
    if !chosen.is_empty()
        && let Some(action) = &parsed.action
        && let Ok(mut step) = crate::spell::step(StepKind::Configure, action)
    {
        let mut args = step.args.into_vec();
        args.insert(0, format!("--fields={}", chosen.join(",")).into());
        step.args = args.into();
        next.push(step);
    }
    next.push(help_step(path));
    Err(UsageError::SourceSelector {
        source,
        message: ErrorMessage::ArgumentGuidance,
        option: "--fields".into(),
        valid_values: row
            .names
            .iter()
            .filter_map(|name| biodata::Text::new(name).ok())
            .collect(),
        next,
    })
}

/// Keep only the chosen and required keys of each row of a finished reply.
pub(crate) fn project(value: &mut Value, fields: &[String]) -> io::Result<()> {
    let Some(id) = value.get("schema").and_then(Value::as_str) else {
        return Ok(());
    };
    let Some(row) = reply_row(id) else {
        return Ok(());
    };
    for item in value
        .pointer_mut("/result/items")
        .and_then(Value::as_array_mut)
        .into_iter()
        .flatten()
    {
        if let Some(item) = item.as_object_mut() {
            item.retain(|key, _| row.required.contains(key) || fields.contains(key));
        }
    }
    Ok(())
}

/// The one help line naming a search's row fields; none for other commands.
pub(crate) fn help_line(verb: &str, entity: &str) -> Option<String> {
    if verb != "search" {
        return None;
    }
    let choose = "Choose some with --fields a,b (typed: fields list)";
    let sources = source_rows(verb, entity);
    if !sources.is_empty() {
        let rows = sources
            .iter()
            .map(|(source, row)| format!("{source}: {}", listed(row)))
            .collect::<Vec<_>>()
            .join("; ");
        return Some(format!("Fields by --source: {rows}. {choose}."));
    }
    let row = search_row(verb, entity);
    if row.names.is_empty() {
        return None;
    }
    let kept = if row.required.is_empty() {
        String::new()
    } else {
        format!("; always kept: {}", joined(&row.required))
    };
    Some(format!("Fields: {}. {choose}{kept}.", joined(&row.names)))
}

fn joined(names: &BTreeSet<String>) -> String {
    names.iter().cloned().collect::<Vec<_>>().join(", ")
}

/// A row's names, then those it always keeps.
fn listed(row: &Row) -> String {
    let names = joined(&row.names);
    if row.required.is_empty() {
        return names;
    }
    format!("{names} (always kept: {})", joined(&row.required))
}

/// Paging, sort, restart and fresh steps of a search keep the caller's fields.
pub(crate) fn carried(step: &Step) -> bool {
    matches!(
        step.kind,
        StepKind::Page | StepKind::Sort | StepKind::Restart | StepKind::Fresh
    ) && step.args.first().is_some_and(|word| &**word == "search")
}
