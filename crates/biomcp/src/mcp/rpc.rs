//! Transport-independent RPC admission and protocol responses.
use super::protocol::ReplyVersion;
use super::{current, discovery, legacy, protocol};
use serde_json::{Value, json};
pub(super) enum Message {
    Reply(Value),
    Tool {
        id: Value,
        params: Value,
        version: ReplyVersion,
    },
    Notification {
        method: String,
        params: Value,
    },
}
pub(super) fn message(
    frame: &[u8],
    version: &mut ReplyVersion,
    revisions: &[&'static str],
    read_only: bool,
    context: crate::command::Context<'_>,
    mode: crate::config::ToolMode,
) -> Message {
    let Ok(value) = serde_json::from_slice::<Value>(frame) else {
        return Message::Reply(protocol::error(Value::Null, -32700, "Parse error"));
    };
    if !protocol::validate(&value) {
        let id = value
            .get("id")
            .filter(|id| protocol::valid_id(id))
            .cloned()
            .unwrap_or(Value::Null);
        return Message::Reply(protocol::error(id, -32600, "Invalid Request"));
    }
    let method = value
        .get("method")
        .and_then(Value::as_str)
        .unwrap_or_default();
    let params = value.get("params").cloned().unwrap_or_else(|| json!({}));
    let Some(id) = value.get("id") else {
        return Message::Notification {
            method: method.into(),
            params,
        };
    };
    if !protocol::valid_params(method, &params) {
        return Message::Reply(protocol::error(id.clone(), -32602, "Invalid params"));
    }
    let modern = *version != ReplyVersion::November2024
        && (current::identified(&params)
            || method == "server/discover"
            || *version == ReplyVersion::Current && method != "initialize");
    if modern {
        if let Err(error) = current::validate(id, &params, revisions) {
            return Message::Reply(error);
        }
    } else if method == "initialize" {
        let Some((revision, result)) = legacy::initialize(&params, mode, context, revisions) else {
            return Message::Reply(protocol::error(id.clone(), -32602, "Invalid params"));
        };
        *version = if revision == protocol::NOVEMBER_2024 {
            ReplyVersion::November2024
        } else {
            ReplyVersion::Legacy
        };
        return Message::Reply(protocol::result(id.clone(), result, false));
    }
    let version = if modern {
        ReplyVersion::Current
    } else {
        *version
    };
    let typed = mode == crate::config::ToolMode::Typed;
    let result = match method {
        "ping" => json!({}),
        "server/discover" if modern => current::discover(mode, context, revisions),
        "tools/list" => match tools(id, &params, typed, read_only, context) {
            Ok(mut tools) => {
                if version == ReplyVersion::November2024 {
                    legacy::project_tools(&mut tools);
                }
                json!({"tools":tools})
            }
            Err(reply) => return Message::Reply(reply),
        },
        "tools/call" => return call(id.clone(), params, typed, read_only, version, context),
        _ => return Message::Reply(protocol::error(id.clone(), -32601, "Method not found")),
    };
    Message::Reply(protocol::result(id.clone(), result, modern))
}

fn tools(
    id: &Value,
    params: &Value,
    typed: bool,
    read_only: bool,
    context: crate::command::Context<'_>,
) -> Result<Vec<Value>, Value> {
    let definitions = if !typed {
        protocol::tools(context.root, read_only)
    } else if discovery::configured(context) || discovery::requested(params) {
        return Ok(discovery::tools(read_only));
    } else {
        super::typed::tools(context, read_only)
    };
    definitions.map_err(|error| {
        let message = if error.kind() == std::io::ErrorKind::InvalidInput {
            error.to_string()
        } else {
            "Output schema unavailable".into()
        };
        protocol::error(id.clone(), -32603, &message)
    })
}

fn call(
    id: Value,
    mut params: Value,
    typed: bool,
    read_only: bool,
    version: ReplyVersion,
    context: crate::command::Context<'_>,
) -> Message {
    if typed && discovery::called(&params) {
        return Message::Reply(discovery::call(id, &params, context, read_only, version));
    }
    if typed && params.get("name").and_then(Value::as_str) == Some(discovery::DISPATCH) {
        let Some(target) = discovery::target(&params) else {
            return Message::Reply(protocol::error(id, -32602, "Invalid params"));
        };
        params = target;
    }
    Message::Tool {
        id,
        params,
        version,
    }
}
