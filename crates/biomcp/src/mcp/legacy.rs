//! Initialization-based protocol edge.
use super::protocol::{self, ReplyVersion};
use serde_json::{Value, json};

pub(super) fn initialize(
    params: &Value,
    mode: crate::config::ToolMode,
    context: crate::command::Context<'_>,
    revisions: &[&'static str],
) -> Option<(ReplyVersion, Value)> {
    let revision = revision(params.get("protocolVersion")?.as_str()?, revisions);
    params.get("capabilities")?.as_object()?;
    let info = params.get("clientInfo")?;
    info.get("name")?.as_str()?;
    info.get("version")?.as_str()?;
    let version = ReplyVersion::for_revision(revision);
    let result = json!({"protocolVersion":revision,
        "capabilities":{"tools":{}}, "serverInfo":protocol::info(version), "instructions":protocol::instructions(mode, context)});
    Some((version, result))
}

// Pinned lifecycle version negotiation preserves a supported requested revision.
// https://modelcontextprotocol.io/specification/2025-06-18/basic/lifecycle#version-negotiation
pub(super) fn revision(requested: &str, revisions: &[&'static str]) -> &'static str {
    revisions
        .iter()
        .skip(1)
        .copied()
        .find(|revision| *revision == requested)
        .unwrap_or(protocol::REVISIONS[1])
}

pub(super) fn project_tools(tools: &mut [Value], version: ReplyVersion) {
    if matches!(version, ReplyVersion::Current | ReplyVersion::Legacy) {
        return;
    }
    for tool in tools.iter_mut().filter_map(Value::as_object_mut) {
        let title = tool.shift_remove("title");
        if version == ReplyVersion::November2024 {
            tool.shift_remove("annotations");
            tool.shift_remove("outputSchema");
        } else if let Some(title) = title
            && let Some(annotations) = tool.get_mut("annotations").and_then(Value::as_object_mut)
        {
            // March 2025 defines the tool title inside annotations.
            annotations.insert("title".into(), title);
        }
    }
}
