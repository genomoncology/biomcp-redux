//! Read-only research projections over the same parsed CLI commands.
mod records;
use serde_json::{Value, json};
use std::{ffi::OsString, io, sync::Arc};

pub(super) fn tools() -> Vec<Value> {
    [("search", "query", "Search BioMCP source headers. Prefix a CLI entity and its arguments, or supply article query text.", "biomcp.research-search/1"),
     ("fetch", "id", "Fetch a search result using its exact id. Returns the CLI record as Markdown with source metadata.", "biomcp.research-fetch/1")]
        .into_iter().map(|(name, key, description, schema)| {
            let output: Value = crate::schema::find(schema).and_then(|s| serde_json::from_str(s).ok()).unwrap_or(json!({"type":"object"}));
            json!({"name":name,"description":description,"inputSchema":{"type":"object","properties":{key:{"type":"string"}},"required":[key],"additionalProperties":false},"outputSchema":output,
                "annotations":{"readOnlyHint":true,"destructiveHint":false,"openWorldHint":true}})
        }).collect()
}
#[derive(Clone)]
pub(super) enum Projection { Cli, Search(String), Fetch(String) }
pub(super) struct Call {
    pub(super) args: Vec<OsString>,
    pub(super) projection: Projection,
}
pub(super) fn parse(params: &Value) -> Option<Call> {
    let name = params.get("name")?.as_str()?;
    let object = params.get("arguments")?.as_object()?;
    if object.len() != 1 { return None; }
    if name == "biomcp" {
        let args = object.get("args")?.as_array()?.iter().map(|a| a.as_str().map(OsString::from)).collect::<Option<_>>()?;
        return Some(Call { args, projection: Projection::Cli });
    }
    let key = match name { "search" => "query", "fetch" => "id", _ => return None };
    let input = object.get(key)?.as_str()?;
    let (args, projection) = if name == "search" {
        (search_args(input), Projection::Search(input.into()))
    } else {
        let mut args = vec!["get".into()];
        args.extend(shlex::split(input).unwrap_or_else(|| vec![input.into()]));
        (args, Projection::Fetch(input.into()))
    };
    Some(Call { args: args.into_iter().map(OsString::from).collect(), projection })
}
fn search_args(input: &str) -> Vec<String> {
    let first = input.split_whitespace().next().unwrap_or_default();
    if crate::grammar::QUERIES.iter().any(|(verb, entity, _)| *verb == "search" && *entity == first) {
        let mut args = vec!["search".into()];
        args.extend(shlex::split(input).unwrap_or_else(|| vec![input.into()]));
        args
    } else { vec!["search".into(), "article".into(), input.into()] }
}
pub(super) fn execute(call: &Call, globals: &crate::grammar::Globals, fetch: &Result<Arc<biomcp_fetch::Fetch>, ()>, http: bool) -> io::Result<crate::command::Outcome> {
    let mut outcome = if http { crate::command::tool_http(&call.args, globals, fetch)? } else { crate::command::tool(&call.args, globals, fetch)? };
    if !matches!(outcome.status, crate::exit::Status::Success | crate::exit::Status::Absent) { return Ok(outcome); }
    outcome.value = match &call.projection {
        Projection::Cli => return Ok(outcome),
        Projection::Search(_) => search(call, outcome.value)?,
        Projection::Fetch(id) => {
            let entity = call.args.get(1).and_then(|s| s.to_str()).unwrap_or_default();
            let hit = records::fetch(entity, id, &outcome.value);
            let Some(hit) = hit else { return Ok(outcome); };
            serde_json::to_value(biomcp_types::ResearchFetch { id: id.clone(), title: hit.title, url: hit.url,
                text: String::from_utf8(crate::view::markdown(&outcome.value)?).map_err(io::Error::other)?, metadata: outcome.value })?
        }
    };
    Ok(outcome)
}
fn search(call: &Call, envelope: Value) -> io::Result<Value> {
    let entity = call.args.get(1).and_then(|s| s.to_str()).unwrap_or_default();
    let rows = envelope.pointer("/result/items").and_then(Value::as_array);
    let results = rows.into_iter().flatten().filter_map(|row| records::search(entity, row, &envelope)).collect::<Vec<_>>();
    let omitted_rows = rows.map_or(usize::from(!envelope.get("result").is_none_or(Value::is_null)), Vec::len).saturating_sub(results.len());
    serde_json::to_value(biomcp_types::ResearchSearch { results, metadata: biomcp_types::ResearchSearchMetadata {
        envelope, omitted_rows, note: "Rows without an exact citable get target remain available through the biomcp tool.".into() } }).map_err(Into::into)
}
