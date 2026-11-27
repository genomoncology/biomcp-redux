//! Confirmation for the typed saved tool's delete-all action.
use super::{protocol, research, typed};
use crate::{command::Context, saved, saved_mcp};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use biomcp_query::store::saved::DeletePreview;
use ring::{
    hmac,
    rand::{SecureRandom, SystemRandom},
};
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

// Use one process key to authenticate the saved targets, caller account and expiration.
// No saved identifiers or pending confirmations travel in the token or live on the server.
const STATE_BYTES: usize = 8 + ring::digest::SHA256_OUTPUT_LEN;
#[expect(
    clippy::disallowed_types,
    reason = "Generate the process signing key once"
)]
static KEY: std::sync::OnceLock<Option<hmac::Key>> = std::sync::OnceLock::new();

fn key() -> Option<&'static hmac::Key> {
    KEY.get_or_init(|| {
        let mut bytes = [0; 32];
        SystemRandom::new().fill(&mut bytes).ok()?;
        Some(hmac::Key::new(hmac::HMAC_SHA256, &bytes))
    })
    .as_ref()
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

fn binding(expiration: &[u8], account: Option<&str>, preview: &DeletePreview) -> Vec<u8> {
    [
        b"saved/delete/all",
        expiration,
        &preview.digest,
        account.unwrap_or_default().as_bytes(),
    ]
    .concat()
}

pub(super) fn state(call: &Call<'_>, preview: &DeletePreview) -> Option<String> {
    // A form has its own lifetime across requests and holds no tool slot.
    let expiration = now().checked_add(300)?.to_be_bytes();
    let tag = hmac::sign(key()?, &binding(&expiration, call.context.account, preview));
    Some(URL_SAFE_NO_PAD.encode([expiration.as_slice(), tag.as_ref()].concat()))
}

pub(super) fn matches_state(state: &str, call: &Call<'_>, preview: &DeletePreview) -> bool {
    let mut bytes = [0; STATE_BYTES];
    if state.len() != (STATE_BYTES * 4).div_ceil(3)
        || URL_SAFE_NO_PAD.decode_slice(state, &mut bytes).ok() != Some(STATE_BYTES)
    {
        return false;
    }
    let Some((expiration, tag)) = bytes.split_at_checked(8) else {
        return false;
    };
    u64::from_be_bytes(expiration.try_into().unwrap_or_default()) >= now()
        && key().is_some_and(|key| {
            hmac::verify(
                key,
                &binding(expiration, call.context.account, preview),
                tag,
            )
            .is_ok()
        })
}
