//! Reject reflected values from this request's credential snapshot before they escape.
use crate::{KeyUse, Leg, RequestError, prepared::Prepared};
use biodata::Reason;

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
        // Scan tokens independently: object decoding drops duplicate fields and
        // its nesting limit can hide strings before recording rejects the reply.
        if reflected(media.as_bytes()) || reflected(body) || json_strings_contain(body, &reflected) {
            return Err(self.failure(Reason::BadReply, Some(status)).into());
        }
        Ok(())
    }
}

fn json_strings_contain(body: &[u8], reflected: &impl Fn(&[u8]) -> bool) -> bool {
    let mut start = None;
    let mut escaped = false;
    for (index, byte) in body.iter().copied().enumerate() {
        if escaped {
            escaped = false;
        } else if byte == b'\\' && start.is_some() {
            escaped = true;
        } else if byte == b'"' {
            if let Some(begin) = start.take() {
                if body.get(begin..=index).is_some_and(|token| {
                    serde_json::from_slice::<String>(token)
                        .is_ok_and(|text| reflected(text.as_bytes()))
                }) {
                    return true;
                }
            } else {
                start = Some(index);
            }
        }
    }
    false
}
