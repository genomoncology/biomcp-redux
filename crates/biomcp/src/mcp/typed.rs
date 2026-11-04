//! Typed tools derive their contracts and CLI spelling from the command registry.
mod arguments;
mod description;
mod summary;
mod validation;
pub(super) fn summarize(
    mut reply: Value,
    root: &crate::grammar::Root,
    mode: crate::config::ToolMode,
    args: &[std::ffi::OsString],
) -> Value {
    if mode == crate::config::ToolMode::Typed
        && let Some(value) = reply.pointer("/result/structuredContent")
    {
        let text = summary::render(root, value, args);
        let content = json!([{"type":"text","text":text}]);
        let mut result = reply["result"].clone();
        result["content"] = content;
        if super::protocol::within_cap(&result) {
            reply["result"] = result;
        }
    }
    reply
}

use serde_json::{Value, json};
use std::io;

fn name(verb: &str, entity: &str) -> String {
    format!("{verb}_{entity}").replace('-', "_")
}
fn schema(id: &str) -> io::Result<Value> {
    serde_json::from_str(
        crate::schema::find(id).ok_or_else(|| io::Error::other("missing query schema"))?,
    )
    .map_err(Into::into)
}
pub(super) fn tools(_root: &crate::grammar::Root, read_only: bool) -> io::Result<Vec<Value>> {
    crate::grammar::command_paths()
        .iter()
        .map(|&(verb, entity, id)| {
            let schema = schema(id)?;
            Ok(json!({"name":name(verb, entity),
            "description":description::render(verb, entity, &schema),
            "inputSchema":arguments::input(&schema),
            "outputSchema":super::output_schema::for_command(verb, entity, id)?,
            "annotations":{"readOnlyHint":read_only,"destructiveHint":false,"openWorldHint":true}}))
        })
        .collect()
}
pub(super) fn parse(
    root: &crate::grammar::Root,
    params: &Value,
    mode: crate::config::ToolMode,
) -> Option<super::research::Call> {
    if mode == crate::config::ToolMode::Single {
        return super::research::parse(params);
    }
    let called = params.get("name")?.as_str()?;
    let &(verb, entity, id) = crate::grammar::command_paths()
        .iter()
        .find(|&&(verb, entity, _)| name(verb, entity) == called)?;
    let schema = schema(id).ok()?;
    let input = arguments::input(&schema);
    let values = params.get("arguments")?;
    if !validation::valid(&input, values) {
        return None;
    }
    let args = arguments::cli(root, verb, entity, &schema, values)?;
    Some(super::research::Call {
        args,
        projection: super::research::Projection::Cli,
    })
}
