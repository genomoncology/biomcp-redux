use crate::{Arg, Endpoint, KeyPlace, KeyUse, Leg, Reply, RequestError, SourceError};
use biodata::Reason;
use std::io::Read;

impl Leg<'_> {
    /// Send a checked JSON GET. A missing record returns `not_found`.
    pub fn json<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<T, RequestError> {
        self.send(ep, params)
    }

    /// Send a checked JSON GET, treating HTTP 404 as an absent record.
    pub fn json_opt<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<Option<T>, RequestError> {
        match self.send(ep, params) {
            Err(RequestError::Source(error)) if error.reason == Reason::NotFound => Ok(None),
            result => result.map(Some),
        }
    }

    pub(crate) fn failure(&self, reason: Reason, status: Option<u16>) -> SourceError {
        SourceError {
            source: self.spec.source,
            reason,
            status,
            retry_after: None,
        }
    }

    fn send<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, RequestError> {
        if self.no_key {
            return Err(self.failure(Reason::NoKey, None).into());
        }
        if !std::ptr::eq(self.spec, ep.spec) {
            return Err(self.failure(Reason::Rejected, None).into());
        }
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path);
        let canonical = crate::retry::canonical(&url, params, self.spec.key);
        let identity = crate::retry::identity(&canonical);
        let keyed = match self.spec.key {
            KeyUse::Optional(c) | KeyUse::Required(c) => {
                self.fetch.config.secrets.get(c.secret).is_some()
            }
            KeyUse::None => false,
        };
        self.cached(ep, &canonical, &identity, || {
            self.attempts(&canonical, &identity, keyed)
        })
    }

    fn attempts<T: Reply>(
        &self,
        url: &str,
        identity: &[u8; 32],
        keyed: bool,
    ) -> Result<(T, u16, Vec<u8>), SourceError> {
        for attempt in 0..crate::retry::ATTEMPTS {
            let permit = self
                .fetch
                .limiters
                .acquire(self.spec.quota, keyed, self.deadline)
                .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
            let (mut error, retryable, delay) = match self.call(url) {
                Ok(mut response) => {
                    let status = response.status().as_u16();
                    if (200..300).contains(&status) {
                        return self.decode(&mut response, status);
                    }
                    let delay = response
                        .headers()
                        .get("retry-after")
                        .and_then(|v| v.to_str().ok())
                        .and_then(|v| crate::retry::retry_after(v, std::time::SystemTime::now()));
                    (
                        self.failure(
                            match status {
                                404 => Reason::NotFound,
                                429 => Reason::RateLimited,
                                500..=599 => Reason::Unavailable,
                                _ => Reason::Rejected,
                            },
                            Some(status),
                        ),
                        crate::retry::retry_status(status),
                        delay,
                    )
                }
                Err(error) => {
                    let retryable = crate::retry::retry_error(&error);
                    let reason = if matches!(error, ureq::Error::Timeout(_)) && !retryable {
                        Reason::DeadlineExceeded
                    } else {
                        Reason::Unavailable
                    };
                    (self.failure(reason, None), retryable, None)
                }
            };
            drop(permit);
            if self.deadline.expired() {
                return Err(self.failure(Reason::DeadlineExceeded, error.status));
            }
            error.retry_after = delay;
            if retryable && delay.is_some_and(|wait| wait >= self.deadline.remaining()) {
                error.reason = Reason::RateLimited;
                return Err(error);
            }
            if !retryable || attempt + 1 == crate::retry::ATTEMPTS {
                return Err(error);
            }
            let wait = delay.unwrap_or_else(|| crate::retry::backoff(identity, attempt));
            if !crate::retry::wait(wait, self.deadline) {
                return Err(self.failure(Reason::DeadlineExceeded, error.status));
            }
        }
        // Every iteration either returns or advances to the next attempt.
        Err(self.failure(Reason::Unavailable, None))
    }

    fn call(&self, url: &str) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let mut request = self
            .fetch
            .agent
            .get(url)
            .header("Accept", "application/json");
        if let KeyUse::Optional(c) | KeyUse::Required(c) = self.spec.key
            && let Some(secret) = self.fetch.config.secrets.get(c.secret)
        {
            request = match c.place {
                KeyPlace::Query(name) => request.query(name, secret),
                KeyPlace::Header(name) => request.header(name, secret),
            };
        }
        let remaining = self.deadline.remaining();
        if remaining.is_zero() {
            return Err(ureq::Error::Timeout(ureq::Timeout::Global));
        }
        request
            .config()
            .timeout_global(Some(remaining))
            .build()
            .call()
    }

    fn decode<T: Reply>(
        &self,
        response: &mut ureq::http::Response<ureq::Body>,
        status: u16,
    ) -> Result<(T, u16, Vec<u8>), SourceError> {
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
        if self.deadline.expired() {
            return Err(self.failure(Reason::DeadlineExceeded, Some(status)));
        }
        Ok((reply, status, bytes))
    }
}
