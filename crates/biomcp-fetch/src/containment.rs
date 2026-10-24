//! Reject reflected values from this request's credential snapshot before they escape.
use crate::{KeyUse, Leg, RequestError, prepared::Prepared};
use biodata::Reason;
use serde_json::Value;

impl Leg<'_> {
    pub(crate) fn keyed(&self) -> bool {
        match self.spec.key {
            KeyUse::Optional(c) | KeyUse::Required(c) => self.fetch.secrets.get(c.secret).is_some(),
            KeyUse::None => false,
        }
    }

    pub(crate) fn contain_response(
        &self,
        prepared: &Prepared,
        media: &str,
        body: &[u8],
        status: u16,
    ) -> Result<(), RequestError> {
        let key = match self.spec.key {
            KeyUse::Optional(c) | KeyUse::Required(c) => self.fetch.secrets.get(c.secret),
            KeyUse::None => None,
        };
        let secrets = key
            .into_iter()
            .chain(prepared.outgoing.iter().map(|(_, secret)| secret.value()))
            .filter(|secret| !secret.is_empty())
            .collect::<Vec<_>>();
        if secrets.is_empty() {
            return Ok(());
        }
        let reflected = |bytes: &[u8]| {
            secrets.iter().any(|secret| {
                bytes
                    .windows(secret.len())
                    .any(|part| part == secret.as_bytes())
            })
        };
        // Check bytes for every media type, and decoded JSON strings (including
        // field names) so JSON escaping cannot conceal a value from this boundary.
        // Parsing retains serde_json's normal nesting and the caller's body limit.
        if reflected(media.as_bytes())
            || reflected(body)
            || serde_json::from_slice::<Value>(body)
                .is_ok_and(|json| json_contains(&json, &reflected))
        {
            return Err(self.failure(Reason::BadReply, Some(status)).into());
        }
        Ok(())
    }
}

fn json_contains(value: &Value, reflected: &impl Fn(&[u8]) -> bool) -> bool {
    match value {
        Value::String(text) => reflected(text.as_bytes()),
        Value::Array(values) => values.iter().any(|value| json_contains(value, reflected)),
        Value::Object(values) => values
            .iter()
            .any(|(name, value)| reflected(name.as_bytes()) || json_contains(value, reflected)),
        _ => false,
    }
}
