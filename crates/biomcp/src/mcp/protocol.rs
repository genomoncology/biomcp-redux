//! Shared RPC validation and advertised CLI tool.
use serde_json::{Value, json};

// Official contracts: https://modelcontextprotocol.io/specification/2026-07-28/basic/versioning
// and https://modelcontextprotocol.io/specification/2025-11-25/basic/lifecycle
pub(super) const INSTRUCTIONS: &str = crate::help::FRONT_MAP;
pub(super) fn instructions(
    mode: crate::config::ToolMode,
    context: crate::command::Context<'_>,
) -> String {
    let compact = mode == crate::config::ToolMode::Typed && super::discovery::configured(context);
    let mut instructions = if compact {
        super::discovery::INSTRUCTIONS.into()
    } else if mode == crate::config::ToolMode::Typed {
        super::typed::instructions(context)
            .unwrap_or_else(|_| "Use help for command arguments.".into())
    } else if context.globals.hosted {
        super::hosted::instructions(context)
            .unwrap_or_else(|_| "Use help for command arguments.".into())
    } else {
        INSTRUCTIONS.into()
    };
    instructions.push('\n');
    instructions.push_str(crate::grammar::ENUM_CASE_GUIDANCE);
    if crate::config::mcp_skills(context.settings) {
        instructions.push_str("\nLoad ");
        instructions.push_str(super::skills::GUIDE_URI);
        instructions.push_str(" for multi-step work.");
    }
    instructions
}
pub(super) const NOVEMBER_2024: &str = "2024-11-05";
const MARCH_2025: &str = "2025-03-26";
pub(super) const REVISIONS: [&str; 5] = [
    "2026-07-28",
    "2025-11-25",
    "2025-06-18",
    MARCH_2025,
    NOVEMBER_2024,
];
pub(super) const HTTP_REVISIONS: &[&str] = REVISIONS.split_at(4).0;
pub(super) const VERSION_KEY: &str = "io.modelcontextprotocol/protocolVersion";
pub(super) const CAPABILITIES_KEY: &str = "io.modelcontextprotocol/clientCapabilities";

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum ReplyVersion {
    Current,
    Legacy,
    March2025,
    November2024,
}

impl ReplyVersion {
    pub(super) fn for_revision(revision: &str) -> Self {
        match revision {
            NOVEMBER_2024 => Self::November2024,
            MARCH_2025 => Self::March2025,
            _ if revision == REVISIONS[0] => Self::Current,
            _ => Self::Legacy,
        }
    }

    fn form(self, settings: &crate::config::Settings) -> ReplyForm {
        if self == Self::November2024 {
            ReplyForm::TextOnly
        } else {
            crate::config::mcp_reply_form(settings)
        }
    }
}

pub(super) fn info(version: ReplyVersion) -> Value {
    let mut info = json!({"name":"biomcp", "version":env!("CARGO_PKG_VERSION")});
    if matches!(version, ReplyVersion::Current | ReplyVersion::Legacy)
        && let Some(fields) = info.as_object_mut()
    {
        fields.insert("title".into(), json!("BioMCP"));
    }
    info
}

pub(super) fn named_tool(title: &str, mut tool: Value) -> Value {
    if let Some(fields) = tool.as_object_mut() {
        fields.insert("title".into(), json!(title));
        if let Some(annotations) = fields.get_mut("annotations").and_then(Value::as_object_mut)
            && annotations.get("readOnlyHint").and_then(Value::as_bool) == Some(true)
        {
            annotations.insert("idempotentHint".into(), json!(true));
        }
    }
    tool
}

pub(super) fn display_name(name: &str) -> String {
    name.split('_')
        .map(|word| {
            let mut chars = word.chars();
            chars
                .next()
                .into_iter()
                .map(|first| first.to_ascii_uppercase())
                .chain(chars)
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join(" ")
}

pub(super) fn error(id: Value, code: i32, message: &str) -> Value {
    json!({"jsonrpc":"2.0", "id":id, "error":{"code":code,"message":message}})
}
/// Keeps the code and leading text hosting relies on, then says how long to wait.
pub(super) fn capacity(
    id: Value,
    limit: usize,
    waited: std::time::Duration,
    retry: std::time::Duration,
) -> Value {
    let waited = waited.as_secs();
    let retry = retry
        .as_secs()
        .saturating_add(u64::from(retry.subsec_nanos() > 0))
        .max(1);
    let message = format!(
        "Tool capacity exceeded: this server runs at most {limit} tool calls at once, \
         and this call waited {waited} seconds for a turn. \
         Retry this call after {retry} seconds."
    );
    json!({"jsonrpc":"2.0", "id":id, "error":{"code":-31000,"message":message,
        "data":{"retry_after_seconds":retry}}})
}
pub(super) fn timeout(id: Value) -> Value {
    json!({"jsonrpc":"2.0", "id":id, "error":{"code":-32000,"message":"Tool deadline exceeded",
        "data":{"reason":biomcp_types::Reason::DeadlineExceeded}}})
}

pub(super) fn valid_id(value: &Value) -> bool {
    value.is_string() || value.is_i64() || value.is_u64()
}

pub(super) fn validate(value: &Value) -> bool {
    value.is_object()
        && value.get("jsonrpc").and_then(Value::as_str) == Some("2.0")
        && value.get("method").is_some_and(Value::is_string)
        && value.get("id").is_none_or(valid_id)
}

pub(super) fn valid_params(method: &str, params: &Value) -> bool {
    params.is_object()
        && params.get("_meta").is_none_or(Value::is_object)
        && (method != "tools/list" || params.get("cursor").is_none_or(Value::is_string))
}

// Tool schema dialect and result contract:
// https://modelcontextprotocol.io/specification/2025-11-25/server/tools
pub(super) const SCHEMA_DIALECT: &str = "https://json-schema.org/draft/2020-12/schema";
const RESULT_CHARACTER_CAP: usize = 150_000;

use crate::config::ReplyForm;

const DESCRIPTION: &str = concat!(
    "Run a BioMCP CLI command. Example: {\"args\":[\"help\"]}. Start with help for the command map, ",
    "help <entity> for source reads, help <command> for options, and help skill render for the agent guide. ",
    "Check the JSON reply's status, problems and next commands before using the evidence."
);

pub(super) fn tool(root: &crate::grammar::Root, read_only: bool) -> std::io::Result<Value> {
    Ok(named_tool(
        "BioMCP",
        json!({"name":"biomcp", "description":DESCRIPTION,
        "annotations":{"readOnlyHint":read_only,"destructiveHint":false,"openWorldHint":true},
        "inputSchema":{"$schema":SCHEMA_DIALECT,"type":"object","properties":{
            "args":{"type":"array","items":{"type":"string"},
                "description":"CLI command words, without biomcp. Example: [\"help\"]"}},
            "required":["args"],"additionalProperties":false},"outputSchema":super::output_schema::for_tool(root, "biomcp")?}),
    ))
}

pub(super) fn tools(root: &crate::grammar::Root, read_only: bool) -> std::io::Result<Vec<Value>> {
    let mut tools = vec![tool(root, read_only)?];
    tools.extend(super::research::tools(root)?);
    Ok(tools)
}

pub(super) fn result(id: Value, value: Value, modern: bool) -> Value {
    if modern {
        current_result(id, value, "complete")
    } else {
        json!({"jsonrpc":"2.0","id":id,"result":value})
    }
}

pub(super) fn input_required(id: Value, value: Value) -> Value {
    current_result(id, value, "input_required")
}

fn current_result(id: Value, mut value: Value, kind: &str) -> Value {
    if let Some(object) = value.as_object_mut() {
        object.insert("resultType".into(), json!(kind));
        let metadata = object.entry("_meta").or_insert_with(|| json!({}));
        metadata["io.modelcontextprotocol/serverInfo"] = info(ReplyVersion::Current);
    }
    json!({"jsonrpc":"2.0","id":id,"result":value})
}

// Tool results use the configured content form and negotiated protocol metadata.
// JSON-RPC correlation IDs belong to the outer response, outside CallToolResult.
pub(super) fn tool_result(
    id: Value,
    outcome: &crate::command::Outcome,
    version: ReplyVersion,
    settings: &crate::config::Settings,
) -> Value {
    let modern = version == ReplyVersion::Current;
    let form = version.form(settings);
    let failed = !matches!(
        outcome.status,
        crate::exit::Status::Success | crate::exit::Status::Absent
    );
    let Ok(value) = outcome.value() else {
        return self::error(id, -32603, "Internal error");
    };
    if within_cap(value) {
        return result(id, content(value.clone(), failed, form), modern);
    }
    let Ok(error) = serde_json::to_value(biomcp_types::ErrorEnvelope::from(
        biomcp_types::UsageError::McpResultTooBroad {
            cap: RESULT_CHARACTER_CAP as u64,
        },
    )) else {
        return self::error(id, -32603, "Internal error");
    };
    result(id, content(error, true, form), modern)
}
pub(super) fn discovery_result(
    id: Value,
    value: Value,
    version: ReplyVersion,
    context: crate::command::Context<'_>,
) -> Value {
    if !within_cap(&value) {
        return error(id, -32603, "Output schema unavailable");
    }
    let form = version.form(context.settings);
    result(
        id,
        content(value, false, form),
        version == ReplyVersion::Current,
    )
}

// The JSON value is the whole text: no status line or summary precedes it.
fn content(value: Value, failed: bool, form: ReplyForm) -> Value {
    let text = || json!([{"type":"text","text":value.to_string()}]);
    match form {
        ReplyForm::Once => {
            json!({"content":[], "structuredContent":value, "isError":failed})
        }
        ReplyForm::Both => {
            json!({"content":text(), "structuredContent":value, "isError":failed})
        }
        ReplyForm::StructuredOnly => {
            json!({"content":[{"type":"text","text":"JSON reply is in structuredContent."}],
            "structuredContent":value, "isError":failed})
        }
        ReplyForm::TextOnly => json!({"content":text(), "isError":failed}),
    }
}
pub(super) fn within_cap(value: &Value) -> bool {
    // Admit the JSON value once, independent of its configured wire representation.
    // Count Unicode scalars after JSON escaping, not UTF-8 bytes or source text.
    value.to_string().chars().take(RESULT_CHARACTER_CAP).count() < RESULT_CHARACTER_CAP
}
