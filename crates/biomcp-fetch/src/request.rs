use crate::record::{Body, Exchange};
use crate::{Arg, Endpoint, KeyPlace, KeyUse, Leg, Reply, RequestError, SourceError};
use crate::{InputError, declaration::Method, prepared::Prepared};
use biodata::Reason;
use crate::xml::Codec;
use std::{
    io::Read,
    time::{Duration, Instant},
};

struct Wire {
    status: u16,
    content_type: String,
    body: Vec<u8>,
    retry_after: Option<Duration>,
}
enum Failure {
    Transport(ureq::Error),
    Local(RequestError),
}

impl Leg<'_> {
    /// Send checked JSON from a GET or form POST. A missing record returns `not_found`.
    pub fn json<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<T, RequestError> {
        self.parameters(ep, params, Codec::Json)
    }

    /// Send checked XML from a GET or form POST.
    pub fn xml<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<T, RequestError> {
        self.parameters(ep, params, Codec::Xml)
    }

    fn parameters<T: Reply>(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)], codec: Codec) -> Result<T, RequestError> {
        self.validate(ep, false)?;
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path);
        let (url, body) = if ep.method == Method::Get {
            (
                crate::retry::canonical(&url, params, self.spec.key),
                Vec::new(),
            )
        } else {
            (
                url,
                crate::prepared::form(params, self.spec.key, self.fetch.config.max_body)?,
            )
        };
        self.send(ep, Prepared::new(url, body, ep.method, ep.idempotent), codec)
    }

    /// Send checked JSON from a GET or form POST, treating HTTP 404 as an absent record.
    pub fn json_opt<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<Option<T>, RequestError> {
        match self.json(ep, params) {
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

    /// Send checked JSON from a declared JSON POST endpoint.
    pub fn post_json<B: serde::Serialize, T: Reply>(
        &self,
        ep: &Endpoint,
        body: &B,
    ) -> Result<T, RequestError> {
        self.validate(ep, true)?;
        let codec = Codec::Json;
        let body = crate::prepared::json(body, self.fetch.config.max_body)?;
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path);
        self.send(ep, Prepared::new(url, body, ep.method, ep.idempotent), codec)
    }

    fn validate(&self, ep: &Endpoint, json_body: bool) -> Result<(), RequestError> {
        if self.no_key {
            return Err(self.failure(Reason::NoKey, None).into());
        }
        if self.wrong_clock || !std::ptr::eq(self.spec, ep.spec) {
            return Err(self.failure(Reason::Rejected, None).into());
        }
        if (ep.method == Method::Json) != json_body {
            return Err(InputError::Method.into());
        }
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None).into());
        }
        Ok(())
    }
    fn send<T: Reply>(&self, ep: &Endpoint, prepared: Prepared, codec: Codec) -> Result<T, RequestError> {
        let keyed = match self.spec.key {
            KeyUse::Optional(c) | KeyUse::Required(c) => {
                self.fetch.config.secrets.get(c.secret).is_some()
            }
            KeyUse::None => false,
        };
        self.cached(ep, &prepared.url, &prepared.identity, codec, || {
            self.attempts(&prepared, keyed, codec)
        })
    }

    fn attempts<T: Reply>(
        &self,
        prepared: &Prepared,
        keyed: bool,
        codec: Codec,
    ) -> Result<(T, u16, Vec<u8>), RequestError> {
        for attempt in 0..crate::retry::ATTEMPTS {
            let permit = self
                .fetch
                .limiters
                .acquire(self.spec.quota, keyed, self.budget())
                .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
            let (mut error, retryable, delay) = match self.exchange(prepared, codec) {
                Ok(response) => {
                    let status = response.status;
                    if (200..300).contains(&status) {
                        return self.decode(response, codec).map_err(Into::into);
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
            if self.budget().expired() {
                return Err(self.failure(Reason::DeadlineExceeded, error.status).into());
            }
            error.retry_after = delay;
            if prepared.idempotent
                && retryable
                && delay.is_some_and(|wait| wait >= self.budget().remaining())
            {
                error.reason = Reason::RateLimited;
                return Err(error.into());
            }
            if !prepared.idempotent || !retryable || attempt + 1 == crate::retry::ATTEMPTS {
                return Err(error.into());
            }
            let wait = delay.unwrap_or_else(|| crate::retry::backoff(&prepared.identity, attempt));
            if !crate::retry::wait(wait, self.budget()) {
                return Err(self.failure(Reason::DeadlineExceeded, error.status).into());
            }
        }
        // Every iteration either returns or advances to the next attempt.
        Err(self.failure(Reason::Unavailable, None).into())
    }

    fn call(&self, prepared: &Prepared, codec: Codec) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let mut request = if prepared.post.is_some() {
            self.fetch.agent.post(&prepared.url)
        } else {
            self.fetch.agent.get(&prepared.url).force_send_body()
        }
        .header("Accept", codec.name());
        if let KeyUse::Optional(c) | KeyUse::Required(c) = self.spec.key
            && let Some(secret) = self.fetch.config.secrets.get(c.secret)
        {
            request = match c.place {
                KeyPlace::Query(name) => request.query(name, secret),
                KeyPlace::Header(name) => request.header(name, secret),
            };
        }
        if let Some(post) = &prepared.post {
            request = request.header("Content-Type", &post.request_content_type);
        }
        let remaining = self.budget().remaining();
        if remaining.is_zero() {
            return Err(ureq::Error::Timeout(ureq::Timeout::Global));
        }
        request
            .config()
            .timeout_global(Some(remaining))
            .build()
            .send(prepared.body.as_slice())
    }

    fn exchange(&self, prepared: &Prepared, codec: Codec) -> Result<Wire, Failure> {
        let local = |error| Failure::Local(RequestError::Recording(error));
        if let Some(exchange) = self
            .fetch
            .recordings
            .replay(
                self.spec.source,
                &prepared.identity,
                &prepared.url,
                prepared.post.as_ref(),
                self.fetch.config.max_body,
            )
            .map_err(local)?
        {
            if !self
                .budget()
                .advance(Duration::from_millis(exchange.delay_ms))
            {
                return Err(Failure::Local(
                    self.failure(Reason::DeadlineExceeded, Some(exchange.status))
                        .into(),
                ));
            }
            return Ok(Wire {
                status: exchange.status,
                content_type: exchange.content_type,
                body: exchange.body.bytes().to_vec(),
                retry_after: exchange.retry_after_ms.map(Duration::from_millis),
            });
        }
        let started = Instant::now();
        let mut response = self.call(prepared, codec).map_err(Failure::Transport)?;
        let status = response.status().as_u16();
        let content_type = response
            .headers()
            .get("content-type")
            .and_then(|v| v.to_str().ok())
            .unwrap_or("")
            .to_owned();
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| crate::retry::retry_after(v, std::time::SystemTime::now()));
        let recording = matches!(self.fetch.config.mode, crate::Mode::Record { .. });
        let body = self.read_body(&mut response, status, recording)?;
        if recording {
            let exchange = Exchange {
                status,
                content_type: content_type.clone(),
                body: Body::capture(body.clone()).map_err(local)?,
                delay_ms: crate::record::millis(started.elapsed()).map_err(local)?,
                retry_after_ms: retry_after
                    .map(crate::record::millis)
                    .transpose()
                    .map_err(local)?,
            };
            self.fetch
                .recordings
                .append(
                    self.spec.source,
                    &prepared.identity,
                    &prepared.url,
                    prepared.post.as_ref(),
                    exchange,
                )
                .map_err(local)?;
        }
        Ok(Wire {
            status,
            content_type,
            body,
            retry_after,
        })
    }

    fn read_body(
        &self,
        response: &mut ureq::http::Response<ureq::Body>,
        status: u16,
        recording: bool,
    ) -> Result<Vec<u8>, Failure> {
        let mut body = Vec::new();
        if recording || (200..300).contains(&status) {
            response
                .body_mut()
                .as_reader()
                .take(self.fetch.config.max_body + 1)
                .read_to_end(&mut body)
                .map_err(|error| {
                    Failure::Local(
                        self.failure(
                            if error.kind() == std::io::ErrorKind::TimedOut
                                || self.budget().expired()
                            {
                                Reason::DeadlineExceeded
                            } else {
                                Reason::BadReply
                            },
                            Some(status),
                        )
                        .into(),
                    )
                })?;
            if body.len() as u64 > self.fetch.config.max_body {
                return Err(Failure::Local(
                    self.failure(Reason::BadReply, Some(status)).into(),
                ));
            }
        }
        Ok(body)
    }

    fn decode<T: Reply>(&self, response: Wire, codec: Codec) -> Result<(T, u16, Vec<u8>), SourceError> {
        let status = response.status;
        let bad = || self.failure(Reason::BadReply, Some(status));
        if !codec.accepts(&response.content_type) {
            return Err(bad());
        }
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, Some(status)));
        }
        let reply = codec.decode(&response.body).ok_or_else(bad)?;
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, Some(status)));
        }
        Ok((reply, status, response.body))
    }
}
