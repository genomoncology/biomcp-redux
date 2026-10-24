use crate::{Arg, Endpoint, KeyPlace, KeyUse, Leg, Reply, RequestError, SourceError};
use biodata::Reason;
use std::{io::Read, time::{Duration, Instant}};
use crate::record::{Exchange, Body};

struct Wire { status: u16, content_type: String, body: Vec<u8>, retry_after: Option<Duration> }
enum Failure { Transport(ureq::Error), Local(RequestError) }

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
    ) -> Result<(T, u16, Vec<u8>), RequestError> {
        for attempt in 0..crate::retry::ATTEMPTS {
            let permit = self
                .fetch
                .limiters
                .acquire(self.spec.quota, keyed, self.deadline)
                .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
            let (mut error, retryable, delay) = match self.exchange(url, identity) {
                Ok(response) => {
                    let status = response.status;
                    if (200..300).contains(&status) {
                        return self.decode(response).map_err(Into::into);
                    }
                    let delay = response.retry_after;
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
                Err(Failure::Local(error)) => return Err(error),
                Err(Failure::Transport(error)) => {
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
                return Err(self.failure(Reason::DeadlineExceeded, error.status).into());
            }
            error.retry_after = delay;
            if retryable && delay.is_some_and(|wait| wait >= self.deadline.remaining()) {
                error.reason = Reason::RateLimited;
                return Err(error.into());
            }
            if !retryable || attempt + 1 == crate::retry::ATTEMPTS {
                return Err(error.into());
            }
            let wait = delay.unwrap_or_else(|| crate::retry::backoff(identity, attempt));
            if !crate::retry::wait(wait, self.deadline) {
                return Err(self.failure(Reason::DeadlineExceeded, error.status).into());
            }
        }
        // Every iteration either returns or advances to the next attempt.
        Err(self.failure(Reason::Unavailable, None).into())
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

    fn exchange(&self, url: &str, identity: &[u8;32]) -> Result<Wire, Failure> {
        let local = |error| Failure::Local(RequestError::Recording(error));
        if let Some(exchange) = self.fetch.recordings.replay(self.spec.source, identity, url, self.fetch.config.max_body).map_err(local)? {
            if !self.deadline.advance(Duration::from_millis(exchange.delay_ms)) {
                return Err(Failure::Local(self.failure(Reason::DeadlineExceeded, Some(exchange.status)).into()));
            }
            return Ok(Wire { status: exchange.status, content_type: exchange.content_type,
                body: exchange.body.bytes().to_vec(), retry_after: exchange.retry_after_ms.map(Duration::from_millis) });
        }
        let started = Instant::now();
        let mut response = self.call(url).map_err(Failure::Transport)?;
        let status = response.status().as_u16();
        let content_type = response.headers().get("content-type").and_then(|v| v.to_str().ok()).unwrap_or("").to_owned();
        let retry_after = response.headers().get("retry-after").and_then(|v| v.to_str().ok())
            .and_then(|v| crate::retry::retry_after(v, std::time::SystemTime::now()));
        let mut body = Vec::new();
        let recording = matches!(self.fetch.config.mode, crate::Mode::Record {..});
        if recording || (200..300).contains(&status) {
            response.body_mut().as_reader().take(self.fetch.config.max_body + 1).read_to_end(&mut body).map_err(|error| {
                Failure::Local(self.failure(if error.kind() == std::io::ErrorKind::TimedOut || self.deadline.expired() {
                    Reason::DeadlineExceeded
                } else { Reason::BadReply }, Some(status)).into())
            })?;
            if body.len() as u64 > self.fetch.config.max_body {
                return Err(Failure::Local(self.failure(Reason::BadReply, Some(status)).into()));
            }
        }
        if recording {
            let exchange = Exchange { status, content_type: content_type.clone(), body: Body::capture(body.clone()).map_err(local)?,
                delay_ms: crate::record::millis(started.elapsed()).map_err(local)?,
                retry_after_ms: retry_after.map(crate::record::millis).transpose().map_err(local)? };
            self.fetch.recordings.append(self.spec.source, identity, url, exchange).map_err(local)?;
        }
        Ok(Wire {status, content_type, body, retry_after})
    }

    fn decode<T: Reply>(&self, response: Wire) -> Result<(T, u16, Vec<u8>), SourceError> {
        let status = response.status;
        let bad = || self.failure(Reason::BadReply, Some(status));
        let content_type = response.content_type.split(';').next().unwrap_or("").trim().to_ascii_lowercase();
        if content_type != "application/json"
            && !(content_type.starts_with("application/") && content_type.ends_with("+json")) {
            return Err(bad());
        }
        if self.deadline.expired() { return Err(self.failure(Reason::DeadlineExceeded, Some(status))); }
        let reply: T = serde_json::from_slice(&response.body).map_err(|_| bad())?;
        reply.check().map_err(|_| bad())?;
        if self.deadline.expired() { return Err(self.failure(Reason::DeadlineExceeded, Some(status))); }
        Ok((reply, status, response.body))
    }
}
