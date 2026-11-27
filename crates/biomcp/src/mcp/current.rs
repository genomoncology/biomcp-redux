//! Stateless 2026-07-28 protocol edge.
use super::protocol::{self, CAPABILITIES_KEY, VERSION_KEY};
use serde_json::{Value, json};

// Catalogs stay stable until restart or reconfiguration. Recheck within a minute
// so clients pick up those changes promptly.
const CATALOG_TTL_MS: u64 = 60_000;

pub(super) fn add_catalog_cache_fields(result: &mut Value, hosted: bool) {
    result["ttlMs"] = json!(CATALOG_TTL_MS);
    result["cacheScope"] = json!(if hosted { "private" } else { "public" });
}

pub(super) fn identified(params: &Value) -> bool {
    params
        .get("_meta")
        .and_then(Value::as_object)
        .is_some_and(|meta| {
            [
                VERSION_KEY,
                CAPABILITIES_KEY,
                "io.modelcontextprotocol/clientInfo",
            ]
            .iter()
            .any(|key| meta.contains_key(*key))
        })
}

pub(super) fn validate(id: &Value, params: &Value, revisions: &[&str]) -> Result<(), Value> {
    let meta = params.get("_meta");
    let version = meta
        .and_then(|m| m.get(VERSION_KEY))
        .and_then(Value::as_str);
    if version.is_none()
        || meta
            .and_then(|m| m.get(CAPABILITIES_KEY))
            .is_none_or(|v| !v.is_object())
    {
        return Err(protocol::error(id.clone(), -32602, "Invalid params"));
    }
    if version != revisions.first().copied() {
        return Err(json!({"jsonrpc":"2.0", "id":id, "error":{
            "code":-32022,"message":"Unsupported protocol version",
            "data":{"supported":revisions,"requested":version}}}));
    }
    Ok(())
}

pub(super) fn discover(
    mode: crate::config::ToolMode,
    context: crate::command::Context<'_>,
    revisions: &[&str],
) -> Value {
    json!({"supportedVersions":revisions,"instructions":protocol::instructions(mode, context),
        "capabilities":super::skills::capabilities(context, true)})
}

/// The retried request echoes the form response and opaque state at the params level.
pub(super) enum Confirmation {
    Initial,
    Answer { state: String, result: Value },
    Invalid,
}

pub(super) fn confirmation(params: &Value) -> Option<Confirmation> {
    let supports_form = params
        .get("_meta")
        .and_then(|meta| meta.get(CAPABILITIES_KEY))
        .is_some_and(super::elicitation::supports_form);
    let state = params.get("requestState");
    let responses = params.get("inputResponses");
    match (supports_form, state, responses) {
        (false, None, None) => None,
        (true, None, None) => Some(Confirmation::Initial),
        (true, Some(Value::String(state)), Some(Value::Object(responses))) => {
            Some(match responses.get(super::elicitation::INPUT) {
                Some(result) => Confirmation::Answer {
                    state: state.clone(),
                    result: result.clone(),
                },
                None => Confirmation::Initial,
            })
        }
        _ => Some(Confirmation::Invalid),
    }
}

impl Confirmation {
    pub(super) fn answer(
        self,
        call: &super::elicitation::Call<'_>,
        preview: &biomcp_query::store::saved::DeletePreview,
    ) -> super::elicitation::Decision {
        use super::elicitation::{self, Decision};
        match self {
            Self::Initial => {
                let Some(state) = elicitation::state(call, preview) else {
                    return Decision::Reply(protocol::error(
                        call.id.clone(),
                        -32603,
                        "Internal error",
                    ));
                };
                Decision::Reply(protocol::input_required(
                    call.id.clone(),
                    json!({"inputRequests":{(elicitation::INPUT):{
                        "method":elicitation::METHOD,"params":elicitation::form(preview, true)
                    }}, "requestState":state}),
                ))
            }
            Self::Answer { state, result } if elicitation::matches_state(&state, call, preview) => {
                if elicitation::approved(&result) {
                    Decision::Approve
                } else {
                    Decision::Decline
                }
            }
            Self::Answer { .. } => Decision::Stale,
            Self::Invalid => {
                Decision::Reply(protocol::error(call.id.clone(), -32602, "Invalid params"))
            }
        }
    }
}
