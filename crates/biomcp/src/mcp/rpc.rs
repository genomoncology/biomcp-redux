//! Transport-independent RPC admission and protocol responses.
use super::protocol::ReplyVersion;
use super::{current, discovery, legacy, protocol};
use serde_json::{Value, json};

const CATALOG_BUDGET_KEY: &str = "biomcp/catalogBudget";

pub(super) enum Message {
    Reply(Value),
    Initialized {
        reply: Value,
        form: Option<bool>,
    },
    ClientResponse {
        id: Value,
        approved: bool,
    },
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
    if let Some(response) = client_response(&value) {
        return response;
    }
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
        && (current::identified(&params) || method == "server/discover");
    if modern {
        if let Err(error) = current::validate(id, &params, revisions) {
            return Message::Reply(error);
        }
    } else if method == "initialize" {
        return initialize(id, &params, version, revisions, context, mode);
    } else if *version == ReplyVersion::Current {
        return Message::Reply(json!({"jsonrpc":"2.0","id":id,"error":{
            "code":-32602,
            "message":"Server is not initialized yet. Send initialize first.",
            "data":{"requiredMethod":"initialize"}
        }}));
    }
    let version = if modern {
        ReplyVersion::Current
    } else {
        *version
    };
    let typed = mode == crate::config::ToolMode::Typed;
    let mut result = match method {
        "ping" => json!({}),
        "server/discover" if modern => current::discover(mode, context, revisions),
        "tools/list" => match tools(id, &params, typed, read_only, context, version) {
            Ok(result) => result,
            Err(reply) => return Message::Reply(reply),
        },
        "tools/call" => return call(id.clone(), params, typed, read_only, version, context),
        _ if super::skills::handles(method, context, modern) => {
            return Message::Reply(super::skills::reply(
                id.clone(),
                method,
                &params,
                context,
                modern,
            ));
        }
        _ => return Message::Reply(protocol::error(id.clone(), -32601, "Method not found")),
    };
    if modern && matches!(method, "server/discover" | "tools/list") {
        current::add_catalog_cache_fields(&mut result, context.globals.hosted);
    }
    Message::Reply(protocol::result(id.clone(), result, modern))
}

fn initialize(
    id: &Value,
    params: &Value,
    version: &mut ReplyVersion,
    revisions: &[&'static str],
    context: crate::command::Context<'_>,
    mode: crate::config::ToolMode,
) -> Message {
    let Some((negotiated, result, form)) = legacy::initialize(params, mode, context, revisions)
    else {
        return Message::Reply(protocol::error(id.clone(), -32602, "Invalid params"));
    };
    *version = negotiated;
    Message::Initialized {
        reply: protocol::result(id.clone(), result, false),
        form,
    }
}

fn client_response(value: &Value) -> Option<Message> {
    if value.get("jsonrpc").and_then(Value::as_str) != Some("2.0")
        || value.get("method").is_some()
        || value.get("params").is_some()
    {
        return None;
    }
    let id = value.get("id").filter(|id| protocol::valid_id(id))?;
    let approved = match (value.get("result"), value.get("error")) {
        (Some(result), None) if result.is_object() => super::elicitation::approved(result),
        (None, Some(error)) if error.is_object() => false,
        _ => return None,
    };
    Some(Message::ClientResponse {
        id: id.clone(),
        approved,
    })
}

fn tools(
    id: &Value,
    params: &Value,
    typed: bool,
    read_only: bool,
    context: crate::command::Context<'_>,
    version: ReplyVersion,
) -> Result<Value, Value> {
    let compact = typed && (discovery::configured(context) || discovery::requested(params));
    let definitions = if !typed {
        protocol::tools(context.root, read_only)
    } else if compact {
        Ok(discovery::tools(read_only))
    } else {
        super::typed::tools(context, read_only)
    };
    let mut tools = definitions.map_err(|error| {
        let message = if error.kind() == std::io::ErrorKind::InvalidInput {
            error.to_string()
        } else {
            "Output schema unavailable".into()
        };
        protocol::error(id.clone(), -32603, &message)
    })?;
    legacy::project_tools(&mut tools, version);
    let mut result = json!({"tools":tools});
    if typed && !compact && params["_meta"][CATALOG_BUDGET_KEY] == true {
        let (_, byte_limit) = crate::config::mcp_typed_budget(context.settings);
        result["_meta"][CATALOG_BUDGET_KEY] = json!({"definitionBytes": byte_limit});
    }
    Ok(result)
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
    if typed
        && params.get("name").and_then(Value::as_str) == Some(discovery::DISPATCH)
        && let Some(fields) = params.as_object_mut()
    {
        let Some(target) = discovery::target(fields) else {
            return Message::Reply(protocol::error(id, -32602, "Invalid params"));
        };
        fields.extend(target);
    }
    Message::Tool {
        id,
        params,
        version,
    }
}
