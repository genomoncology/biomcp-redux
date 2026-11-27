//! Confirmation for the typed saved tool's delete-all action.
use super::{protocol, research, typed};
use crate::{command::Context, saved, saved_mcp};
use biomcp_fetch::TrialCursorKey;
use biomcp_query::store::saved::DeletePreview;
use serde_json::{Value, json};
use std::{
    io,
    time::{SystemTime, UNIX_EPOCH},
};

pub(super) const METHOD: &str = "elicitation/create";
pub(super) const INPUT: &str = "confirm_delete_all";
const NOTHING: &str = "Nothing was deleted.";
const CHANGED: &str = "Saved work changed or confirmation expired. Nothing was deleted.";

pub(super) fn supports_form(capabilities: &Value) -> bool {
    capabilities
        .get("elicitation")
        .and_then(Value::as_object)
        .is_some_and(|modes| modes.is_empty() || modes.get("form").is_some_and(Value::is_object))
}

fn delete_all(call: &research::Call) -> bool {
    matches!(call.projection, research::Projection::Saved)
        && call.args == ["saved", "delete", "--all"]
}

pub(super) fn approved(result: &Value) -> bool {
    result.get("action").and_then(Value::as_str) == Some("accept")
        && result.pointer("/content/confirm") == Some(&Value::Bool(true))
}

pub(super) struct Call<'a> {
    pub(super) id: Value,
    pub(super) version: protocol::ReplyVersion,
    pub(super) context: Context<'a>,
}

impl Call<'_> {
    pub(super) fn run(self, call: &research::Call, http: bool) -> io::Result<Value> {
        let outcome = research::execute(call, self.context, http)?;
        Ok(typed::tool_result(
            self.id,
            &outcome,
            self.version,
            self.context.settings,
        ))
    }

    pub(super) fn confirm(
        self,
        call: &research::Call,
        http: bool,
        answer: impl FnOnce(&Self, &DeletePreview) -> Decision,
    ) -> io::Result<Value> {
        if !delete_all(call) {
            return self.run(call, http);
        }
        let encoding = crate::command::Encoding::Mcp;
        let file = match saved_mcp::file(self.context) {
            Ok(file) => file,
            Err(error) => return self.reply(saved::usage(encoding, error)?),
        };
        let at = saved::At {
            encoding,
            file: file.as_deref(),
            context: Some(self.context),
        };
        let path = match saved::path(at)? {
            Ok(path) => path,
            Err(outcome) => return self.reply(outcome),
        };
        let preview = match DeletePreview::open(&path) {
            Ok(preview) => preview,
            Err(error) => return self.reply(saved::failure(encoding, error, "")?),
        };
        match answer(&self, &preview) {
            Decision::Reply(reply) => Ok(reply),
            Decision::Decline => self.nothing(NOTHING),
            Decision::Stale => self.nothing(CHANGED),
            Decision::Approve => {
                let result = match saved::open(at, false)? {
                    Err(outcome) => return self.reply(outcome),
                    Ok(None) if preview.counts == [0; 3] => Ok(Some(Default::default())),
                    Ok(None) => Ok(None),
                    Ok(Some(mut store)) => store.delete_confirmed(&preview, || {
                        self.context
                            .scope
                            .is_some_and(|scope| scope.remaining().is_zero())
                    }),
                };
                match result.transpose() {
                    Some(result) => self.reply(saved::reply(encoding, result, "NAME")?),
                    None => self.nothing(CHANGED),
                }
            }
        }
    }

    fn reply(self, outcome: crate::command::Outcome) -> io::Result<Value> {
        Ok(protocol::tool_result(
            self.id,
            &outcome,
            self.version,
            self.context.settings,
        ))
    }

    fn nothing(self, text: &str) -> io::Result<Value> {
        let outcome = saved::reply(
            crate::command::Encoding::Mcp,
            Ok(Default::default()),
            "NAME",
        )?;
        Ok(protocol::result(
            self.id,
            json!({"isError":false,"content":[{"type":"text","text":text}],
                "structuredContent":outcome.value()?}),
            self.version == protocol::ReplyVersion::Current,
        ))
    }
}

pub(super) enum Decision {
    Approve,
    Decline,
    Stale,
    Reply(Value),
}

pub(super) fn form(preview: &DeletePreview, explicit_mode: bool) -> Value {
    let [searches, lists, notes] = preview.counts;
    let mut form = json!({
        "message":format!("Permanently delete {searches} searches, {lists} lists and {notes} notes?"),
        "requestedSchema":{"type":"object","properties":{"confirm":{"type":"boolean"}},"required":["confirm"]}
    });
    if explicit_mode {
        form["mode"] = json!("form");
    }
    form
}

// Reuse the hosted continuation key across workers, with a distinct binding.
#[expect(
    clippy::disallowed_types,
    reason = "Generate the local continuation key once"
)]
static LOCAL_KEY: std::sync::OnceLock<Option<TrialCursorKey>> = std::sync::OnceLock::new();

fn key<'a>(context: Context<'a>) -> Option<&'a TrialCursorKey> {
    context
        .fetch
        .as_ref()
        .ok()
        .and_then(|fetch| fetch.trial_cursor_key())
        .or_else(|| LOCAL_KEY.get_or_init(TrialCursorKey::generate).as_ref())
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn binding(call: &Call<'_>, preview: &DeletePreview) -> Vec<u8> {
    [
        b"mcp/saved/delete/all/1",
        preview.digest.as_slice(),
        call.context.account.unwrap_or_default().as_bytes(),
    ]
    .concat()
}

pub(super) fn state(call: &Call<'_>, preview: &DeletePreview) -> Option<String> {
    // A form has its own lifetime across requests and holds no tool slot.
    let expiration = now().checked_add(300)?.to_be_bytes().to_vec();
    key(call.context)?
        .seal(expiration, &binding(call, preview))
        .ok()
}

pub(super) fn matches_state(state: &str, call: &Call<'_>, preview: &DeletePreview) -> bool {
    let Some(bytes) = key(call.context).and_then(|key| key.open(state, &binding(call, preview)))
    else {
        return false;
    };
    <[u8; 8]>::try_from(bytes).is_ok_and(|expires| u64::from_be_bytes(expires) > now())
}
