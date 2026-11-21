//! The one reviewed path that re-reads a saved search from its source for `saved changed`
//! (sdlc/planning/saved-work.md, Change detection). It never reads the reply cache: every
//! page runs fresh. The once-an-hour floor in the saved file decides whether it runs at all.
//! The command line and the typed `saved` tool reach it; a caller's own `--fresh` stays
//! refused in hosted mode, since only this floor-bound path reads fresh.
use super::{Encoding, Outcome};
use biodata::Text;
use biomcp_query::Action;
use biomcp_query::store::saved::{NewSnapshot, SNAPSHOT_ROWS, SnapshotRow};
use biomcp_types::{SavedSnapshotSource, Source, SourceTimestamp, StepKind, UsageError};
use serde_json::Value;
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, ffi::OsString, io};

/// Run a saved search's stored words from the source and collect its matching IDs, page by
/// page, up to the snapshot cap. A search that no longer parses answers its usage error with
/// the original words; a failed or partial read answers that reply and keeps nothing.
pub(crate) fn recheck(
    root: &crate::grammar::Root,
    settings: &crate::config::Settings,
    process: &crate::grammar::Globals,
    words: &[Box<str>],
    encoding: Encoding,
    context: Option<super::Context<'_>>,
) -> io::Result<Result<NewSnapshot, Outcome>> {
    let args: Vec<OsString> = words.iter().map(|word| OsString::from(&**word)).collect();
    let mut parsed = match super::parse_arguments(root, &args, encoding)? {
        Ok(parsed) => parsed,
        Err(outcome) => return Ok(Err(outcome)),
    };
    if parsed.globals.inherit(process).is_err() {
        return Ok(Err(super::usage(root, &args, encoding)?));
    }
    // The core stores only words that passed this policy, but the file is not trusted: an
    // MCP re-check, hosted ones included, runs under the policy of any MCP call.
    if context.is_some()
        && let Some(error) = super::mcp_policy::refusal(&parsed, false)
    {
        return Ok(Err(super::usage_envelope(error, encoding)?));
    }
    // Only a first search page is ever saved, so only one is re-read.
    let request = match parsed.action.take().map(super::prepare) {
        Some(Ok(Action::Run { request, .. }))
            if parsed.help.is_none()
                && request.path().0 == "search"
                && crate::grammar::first_page(&request) =>
        {
            request
        }
        Some(Err(error)) => return Ok(Err(super::usage_envelope(error, encoding)?)),
        _ => return Ok(Err(super::usage(root, &args, encoding)?)),
    };
    // A source the call leaves out, or its use policy keeps off, refuses the re-check as the
    // live call's arguments are refused; running the rest would report false gone rows.
    if let Some(context) = context {
        let excluded = crate::mcp::typed::catalog::excluded(context)?;
        if !crate::mcp::typed::catalog::request_allowed(&request, &excluded)? {
            return Ok(Err(super::usage_envelope(
                UsageError::arguments(),
                encoding,
            )?));
        }
    }
    let mut globals = parsed.globals;
    // Every page reads the source, and whole rows are fingerprinted.
    globals.fresh = true;
    globals.fields.clear();
    // An MCP call re-reads through its own transport: its keys, left-out sources and deadline.
    let transport = match super::transport::fetch(&globals, settings, context)? {
        Ok(transport) => transport,
        Err(error) => return Ok(Err(super::local::startup(error, encoding)?)),
    };
    let settings = transport.settings.as_deref().unwrap_or(settings);
    let entity = crate::spell::command_entity(&request);
    let mut walk = Walk::default();
    let mut next = Some(request);
    while let Some(request) = next.take() {
        let result = super::run_request(&transport.fetch, &request, true, settings, false, &[])?;
        let page = result
            .next
            .iter()
            .find(|step| step.0 == StepKind::Page)
            .and_then(|step| match &step.1 {
                Action::Run { request, .. } => Some(request.clone()),
                _ => None,
            });
        let outcome = super::response(
            &transport.fetch,
            request,
            result,
            true,
            &globals,
            Encoding::Json,
        )?;
        let envelope: Value = serde_json::from_slice(outcome.bytes()?).map_err(io::Error::other)?;
        let status = envelope.get("status").and_then(Value::as_str);
        if !matches!(status, Some("complete" | "absent")) {
            return Ok(Err(encoding.encode(envelope, outcome.status)?));
        }
        let read = walk.page(entity, &envelope)?;
        next = page.filter(|_| read > 0 && walk.room());
        if next.is_none() && page_left(&envelope) {
            walk.complete = false;
        }
    }
    walk.finish().map(Ok)
}

/// Whether a page still offers a further page.
fn page_left(envelope: &Value) -> bool {
    envelope
        .get("next")
        .and_then(Value::as_array)
        .is_some_and(|steps| {
            steps
                .iter()
                .any(|step| step.get("kind").and_then(Value::as_str) == Some("page"))
        })
}

#[derive(Debug)]
struct Walk {
    rows: Vec<SnapshotRow>,
    seen: BTreeSet<(String, String, String)>,
    total: Option<u64>,
    complete: bool,
    row_schema: Option<Text>,
    sources: Vec<SavedSnapshotSource>,
}
impl Default for Walk {
    fn default() -> Self {
        Self {
            rows: Vec::new(),
            seen: BTreeSet::new(),
            total: None,
            complete: true,
            row_schema: None,
            sources: Vec::new(),
        }
    }
}

impl Walk {
    fn room(&self) -> bool {
        self.complete && self.rows.len() < SNAPSHOT_ROWS
    }

    /// Take one page's rows; return how many rows it held.
    fn page(&mut self, entity: &str, envelope: &Value) -> io::Result<usize> {
        if self.row_schema.is_none() {
            self.row_schema = Some(row_schema(envelope)?);
            self.total = envelope.pointer("/page/total").and_then(Value::as_u64);
            self.sources = sources(envelope);
            if self
                .total
                .is_some_and(|total| total > u64::try_from(SNAPSHOT_ROWS).unwrap_or(u64::MAX))
            {
                self.complete = false;
            }
        }
        let rows = envelope
            .pointer("/result/items")
            .or_else(|| envelope.pointer("/result/rows"))
            .and_then(Value::as_array)
            .map(Vec::as_slice)
            .unwrap_or_default();
        for row in rows {
            let Some(row) = snapshot_row(entity, row, envelope)? else {
                // A row without an exact get target cannot be followed by ID.
                self.complete = false;
                continue;
            };
            let key = (
                row.entity.to_string(),
                row.id.to_string(),
                row.source
                    .as_ref()
                    .map(ToString::to_string)
                    .unwrap_or_default(),
            );
            if self.seen.insert(key) && self.rows.len() < SNAPSHOT_ROWS {
                self.rows.push(row);
            }
        }
        Ok(rows.len())
    }

    fn finish(self) -> io::Result<NewSnapshot> {
        let held = u64::try_from(self.rows.len()).map_err(io::Error::other)?;
        let total = self.total.unwrap_or(held);
        Ok(NewSnapshot {
            total,
            complete: self.complete && total == held,
            row_schema: self
                .row_schema
                .ok_or_else(|| io::Error::other("no page read"))?,
            sources: self.sources,
            rows: self.rows,
        })
    }
}

/// The reply schema the rows were read under, with a digest of the row definition alone
/// (descriptions removed), so a row format change rebaselines while wording and other parts
/// of the reply do not.
fn row_schema(envelope: &Value) -> io::Result<Text> {
    let id = envelope
        .get("schema")
        .and_then(Value::as_str)
        .unwrap_or("unknown");
    let schema: Value = crate::schema::find(id)
        .and_then(|text| serde_json::from_str(text).ok())
        .unwrap_or(Value::Null);
    let shape = row_definition(&schema).unwrap_or_else(|| strip(&schema));
    let digest = Sha256::digest(serde_json::to_vec(&shape).map_err(io::Error::other)?);
    let hex: String = digest
        .iter()
        .take(8)
        .map(|byte| format!("{byte:02x}"))
        .collect();
    Text::new(&format!("{id}#{hex}")).map_err(io::Error::other)
}

/// The row node of a reply schema and every definition it reaches, without descriptions.
fn row_definition(schema: &Value) -> Option<Value> {
    let definitions = schema.get("$defs")?;
    let body = first_ref(schema.pointer("/properties/result")?)?;
    let properties = definitions.get(&body)?.get("properties")?;
    let rows = properties.get("items").or_else(|| properties.get("rows"))?;
    let mut reached = std::collections::BTreeMap::new();
    let mut pending = Vec::new();
    refs(rows, &mut pending);
    while let Some(name) = pending.pop() {
        if reached.contains_key(&name) {
            continue;
        }
        let definition = definitions.get(&name)?;
        refs(definition, &mut pending);
        reached.insert(name, strip(definition));
    }
    Some(serde_json::json!({"rows": strip(rows), "defs": reached}))
}

fn first_ref(node: &Value) -> Option<String> {
    let mut found = Vec::new();
    refs(node, &mut found);
    found.into_iter().next()
}

/// Definition names a node refers to.
fn refs(node: &Value, names: &mut Vec<String>) {
    match node {
        Value::Object(fields) => {
            for (key, value) in fields {
                if key == "$ref"
                    && let Some(name) = value.as_str().and_then(|r| r.strip_prefix("#/$defs/"))
                {
                    names.push(name.to_owned());
                } else {
                    refs(value, names);
                }
            }
        }
        Value::Array(values) => values.iter().for_each(|value| refs(value, names)),
        _ => {}
    }
}

/// A schema node without its prose, in key order.
fn strip(node: &Value) -> Value {
    match node {
        Value::Object(fields) => Value::Object(
            fields
                .iter()
                // A property may itself be named description; only prose values go.
                .filter(|(key, value)| match key.as_str() {
                    "description" | "title" => !value.is_string(),
                    "examples" | "x-enum-descriptions" => !value.is_array(),
                    _ => true,
                })
                .map(|(key, value)| (key.clone(), strip(value)))
                .collect::<std::collections::BTreeMap<_, _>>()
                .into_iter()
                .collect(),
        ),
        Value::Array(values) => Value::Array(values.iter().map(strip).collect()),
        other => other.clone(),
    }
}

fn sources(envelope: &Value) -> Vec<SavedSnapshotSource> {
    let date = envelope
        .pointer("/result/source_updated")
        .and_then(|value| serde_json::from_value::<SourceTimestamp>(value.clone()).ok());
    let asked = envelope
        .get("sources")
        .and_then(Value::as_array)
        .map(Vec::as_slice)
        .unwrap_or_default();
    asked
        .iter()
        .filter_map(|outcome| {
            Some(SavedSnapshotSource {
                source: outcome.get("source")?.as_str()?.parse().ok()?,
                source_date: date.clone().filter(|_| asked.len() == 1),
                count: outcome.get("count").and_then(Value::as_u64),
            })
        })
        .collect()
}

/// The record a search row names, by the get command that reads it, and the row's fingerprint.
fn snapshot_row(entity: &str, row: &Value, envelope: &Value) -> io::Result<Option<SnapshotRow>> {
    let Some(hit) = crate::mcp::research::records::search(entity, row, envelope) else {
        return Ok(None);
    };
    let Some(words) = shlex::split(hit.id.as_str()) else {
        return Ok(None);
    };
    let (Some(entity), Some(id)) = (words.first(), words.get(1)) else {
        return Ok(None);
    };
    let source = words
        .iter()
        .position(|word| word == "--source")
        .and_then(|index| words.get(index + 1))
        .and_then(|source| source.parse::<Source>().ok());
    let digest = Sha256::digest(serde_json::to_vec(row).map_err(io::Error::other)?);
    let fingerprint: String = digest.iter().map(|byte| format!("{byte:02x}")).collect();
    Ok(Some(SnapshotRow {
        entity: Text::new(entity).map_err(io::Error::other)?,
        id: Text::new(id).map_err(io::Error::other)?,
        source,
        fingerprint: Text::new(&fingerprint).map_err(io::Error::other)?,
    }))
}
