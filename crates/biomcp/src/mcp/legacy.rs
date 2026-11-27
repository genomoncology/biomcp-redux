//! Initialization-based protocol edge.
use super::protocol::{self, ReplyVersion};
use serde_json::{Value, json};

pub(super) fn initialize(
    params: &Value,
    mode: crate::config::ToolMode,
    context: crate::command::Context<'_>,
    revisions: &[&'static str],
) -> Option<(ReplyVersion, Value, Option<bool>)> {
    let revision = revision(params.get("protocolVersion")?.as_str()?, revisions);
    let capabilities = params.get("capabilities")?;
    capabilities.as_object()?;
    let info = params.get("clientInfo")?;
    info.get("name")?.as_str()?;
    info.get("version")?.as_str()?;
    let version = ReplyVersion::for_revision(revision);
    let result = json!({"protocolVersion":revision,
        "capabilities":super::skills::capabilities(context, false),
        "serverInfo":protocol::info(version), "instructions":protocol::instructions(mode, context)});
    // June 2025 introduced forms. November 2025 added the explicit mode field.
    let form = match revision {
        "2025-06-18" if super::elicitation::supports_form(capabilities) => Some(false),
        "2025-11-25" if super::elicitation::supports_form(capabilities) => Some(true),
        _ => None,
    };
    Some((version, result, form))
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
