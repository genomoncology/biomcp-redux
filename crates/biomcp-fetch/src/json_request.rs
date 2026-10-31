//! JSON POST preparation uses the shared checked transport and cache path.
use crate::{Arg, Endpoint, Leg, Reply, RequestError, prepared::Prepared, read::Read};

impl Leg<'_> {
    /// Send checked JSON from a declared JSON POST endpoint.
    pub fn post_json<B: serde::Serialize, T: Reply>(
        &self,
        ep: &Endpoint,
        body: &B,
    ) -> Result<T, RequestError> {
        self.post_json_with_args(ep, &[], body)
    }

    /// Send a checked JSON body with shared canonical query arguments.
    pub fn post_json_with_args<B: serde::Serialize, T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        body: &B,
    ) -> Result<T, RequestError> {
        self.post_json_decoded(ep, params, body, Read::json())
            .map(|(reply, _)| reply)
    }

    /// Check request-bound JSON POST identity before admission and on cache/replay reads.
    pub fn post_json_received_checked<B: serde::Serialize, T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        body: &B,
        check: impl Fn(&T) -> Result<(), crate::BadReply>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.post_json_decoded(ep, params, body, Read::checked_json(&check))
    }

    /// Keep valid source rows while denying cache admission for mixed request identities.
    /// The same predicate rejects pre-existing entries before fresh or stale reuse.
    pub fn post_json_checked_cache<B: serde::Serialize, T: Reply>(
        &self,
        ep: &Endpoint,
        body: &B,
        cache_if: impl Fn(&T) -> bool,
    ) -> Result<T, RequestError> {
        self.post_json_decoded(ep, &[], body, Read::json().cache_if(&cache_if))
            .map(|(reply, _)| reply)
    }

    fn post_json_decoded<B: serde::Serialize, T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        body: &B,
        codec: Read<'_, T>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.validate(ep, true)?;
        let body = crate::prepared::json(body, self.fetch.config.max_body)?;
        let url = if ep.publisher {
            ep.path.to_string()
        } else {
            format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path)
        };
        let url = crate::retry::canonical(&url, params, self.spec.key);
        self.send(
            ep,
            Prepared::new(url, body, ep.method, ep.idempotent),
            codec,
        )
    }
}
