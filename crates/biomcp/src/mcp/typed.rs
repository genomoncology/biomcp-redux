//! Typed tools derive their contracts and CLI spelling from the command registry.
mod arguments;
mod description;
mod summary;
mod validation;
pub(super) fn tool_result(
    id: Value,
    outcome: &crate::command::Outcome,
    modern: bool,
    globals: &crate::grammar::Globals,
    settings: &crate::config::Settings,
    root: &crate::grammar::Root,
    args: &[std::ffi::OsString],
) -> Value {
    let form = crate::config::mcp_reply_form(settings);
    let reply = super::protocol::tool_result(id, outcome, modern, form);
    summarize(
        reply,
        root,
        crate::config::mcp_tool_mode(globals, settings),
        args,
        form,
    )
}

fn summarize(
    mut reply: Value,
    root: &crate::grammar::Root,
    mode: crate::config::ToolMode,
    args: &[std::ffi::OsString],
    form: crate::config::ReplyForm,
) -> Value {
    if mode != crate::config::ToolMode::Typed {
        return reply;
    }
    let value = reply
        .pointer("/result/structuredContent")
        .cloned()
        .or_else(|| {
            let text = reply.pointer("/result/content/0/text")?.as_str()?;
            serde_json::from_str(text).ok()
        });
    if let Some(value) = value {
        let summary = summary::render(root, &value, args);
        let text = if matches!(form, crate::config::ReplyForm::StructuredOnly) {
            summary
        } else {
            format!("{summary}\n{value}")
        };
        if let Some(result) = reply.get_mut("result").and_then(Value::as_object_mut) {
            result.insert("content".into(), json!([{"type":"text","text":text}]));
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
pub(super) fn tools(
    _root: &crate::grammar::Root,
    read_only: bool,
    settings: &crate::config::Settings,
) -> io::Result<Vec<Value>> {
    let (output_schema, enum_limit) = crate::config::mcp_typed_schema_options(settings);
    crate::grammar::command_paths()
        .iter()
        .map(|&(verb, entity, id)| {
            let schema = schema(id)?;
            let mut tool = json!({"name":name(verb, entity),
            "description":description::render(verb, entity, &schema),
            "inputSchema":arguments::input_with_limit(&schema, enum_limit),
            "annotations":{"readOnlyHint":read_only,"destructiveHint":false,"openWorldHint":true}});
            if output_schema {
                tool["outputSchema"] = super::output_schema::for_command(verb, entity, id)?;
            }
            Ok(tool)
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
