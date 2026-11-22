//! Transport-independent RPC admission and protocol responses.
use super::{current, discovery, legacy, protocol};
use serde_json::{Value, json};
pub(super) enum Message {
    Reply(Value),
    Tool {
        id: Value,
        params: Value,
        modern: bool,
    },
    Notification {
        method: String,
        params: Value,
    },
}
pub(super) fn message(
    frame: &[u8],
    legacy: &mut bool,
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
    let modern = current::identified(&params)
        || method == "server/discover"
        || !*legacy && method != "initialize";
    if modern {
        if let Err(error) = current::validate(id, &params) {
            return Message::Reply(error);
        }
    } else if method == "initialize" {
        let Some(result) = legacy::initialize(&params, mode, context) else {
            return Message::Reply(protocol::error(id.clone(), -32602, "Invalid params"));
        };
        *legacy = true;
        return Message::Reply(protocol::result(id.clone(), result, false));
    }
    let typed = mode == crate::config::ToolMode::Typed;
    let result = match method {
        "ping" => json!({}),
        "server/discover" => current::discover(mode, context),
        "tools/list" => match tools(id, &params, typed, read_only, context) {
            Ok(tools) => json!({"tools":tools}),
            Err(reply) => return Message::Reply(reply),
        },
        "tools/call" if typed && discovery::called(&params) => {
            return Message::Reply(discovery::call(
                id.clone(),
                &params,
                context,
                read_only,
                modern,
            ));
        }
        "tools/call" => {
            return Message::Tool {
                id: id.clone(),
                params,
                modern,
            };
        }
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
    } else if discovery::requested(params) {
        return Ok(vec![discovery::tool()]);
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
