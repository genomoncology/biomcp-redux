use crate::{Arg, Endpoint, KeyPlace, KeyUse, Leg, Reply, SourceError};
use biodata::Reason;
use std::io::Read;

impl Leg<'_> {
    /// Send a checked JSON GET. A missing record returns `not_found`.
    pub fn json<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<T, SourceError> {
        self.send(ep, params)
    }

    /// Send a checked JSON GET, treating HTTP 404 as an absent record.
    pub fn json_opt<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<Option<T>, SourceError> {
        match self.send(ep, params) {
            Err(error) if error.reason == Reason::NotFound => Ok(None),
            result => result.map(Some),
        }
    }

    fn failure(&self, reason: Reason, status: Option<u16>) -> SourceError {
        SourceError {
            source: self.spec.source,
            reason,
            status,
        }
    }

    fn send<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, SourceError> {
        if self.no_key {
            return Err(self.failure(Reason::NoKey, None));
        }
        if !std::ptr::eq(self.spec, ep.spec) {
            return Err(self.failure(Reason::Rejected, None));
        }
        if self.deadline.expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None));
        }
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path);
        let mut request = self
            .fetch
            .agent
            .get(&url)
            .header("Accept", "application/json");
        for (name, value) in params {
            request = request.query(name, value.value());
        }
        if let KeyUse::Optional(credential) | KeyUse::Required(credential) = self.spec.key
            && let Some(secret) = self.fetch.config.secrets.get(credential.secret)
        {
            request = match credential.place {
                KeyPlace::Query(name) => request.query(name, secret),
                KeyPlace::Header(name) => request.header(name, secret),
            };
        }
        let remaining = self.deadline.remaining();
        if remaining.is_zero() {
            return Err(self.failure(Reason::DeadlineExceeded, None));
        }
        let mut response = request
            .config()
            .timeout_global(Some(remaining))
            .build()
            .call()
            .map_err(|error| {
                self.failure(
                    if matches!(error, ureq::Error::Timeout(_)) {
                        Reason::DeadlineExceeded
                    } else {
                        Reason::Unavailable
                    },
                    None,
                )
            })?;
        let status = response.status().as_u16();
        if !(200..300).contains(&status) {
            return Err(self.failure(
                match status {
                    404 => Reason::NotFound,
                    429 => Reason::RateLimited,
                    500..=599 => Reason::Unavailable,
                    _ => Reason::Rejected,
                },
                Some(status),
            ));
        }
        self.decode(&mut response, status)
    }

    fn decode<T: Reply>(
        &self,
        response: &mut ureq::http::Response<ureq::Body>,
        status: u16,
    ) -> Result<T, SourceError> {
        let bad = || self.failure(Reason::BadReply, Some(status));
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|value| value.to_str().ok())
            .unwrap_or("")
            .split(';')
            .next()
            .unwrap_or("")
            .trim()
            .to_ascii_lowercase();
        if content_type != "application/json"
            && !(content_type.starts_with("application/") && content_type.ends_with("+json"))
        {
            return Err(bad());
        }
        let mut bytes = Vec::new();
        response
            .body_mut()
            .as_reader()
            .take(self.fetch.config.max_body + 1)
            .read_to_end(&mut bytes)
            .map_err(|error| {
                self.failure(
                    if error.kind() == std::io::ErrorKind::TimedOut || self.deadline.expired() {
                        Reason::DeadlineExceeded
                    } else {
                        Reason::BadReply
                    },
                    Some(status),
                )
            })?;
        if bytes.len() as u64 > self.fetch.config.max_body {
            return Err(bad());
        }
        if self.deadline.expired() {
            return Err(self.failure(Reason::DeadlineExceeded, Some(status)));
        }
        let reply: T = serde_json::from_slice(&bytes).map_err(|_| bad())?;
        reply.check().map_err(|_| bad())?;
        Ok(reply)
    }
}
