use crate::record::{Body, Exchange};
use crate::xml::{Decoder, XmlReply};
use crate::{Arg, Endpoint, KeyPlace, KeyUse, Leg, Reply, RequestError, SourceError};
use crate::{InputError, declaration::Method, prepared::Prepared};
use biodata::Reason;
use std::{
    io::Read,
    time::{Duration, Instant},
};

struct Wire {
    fetched: i64,
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
    /// Probe declared reachability through the same bounded transport, quota and replay policy.
    /// Probe replies never enter the checked data cache.
    pub fn probe(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<(), RequestError> {
        let ep = ep.clone().fresh(crate::Freshness::Never);
        self.parameters(&ep, params, Decoder::probe()).map(|_| ())
    }
    /// Probe a declared read-only JSON endpoint with a bounded request body.
    pub fn probe_json<B: serde::Serialize>(
        &self,
        ep: &Endpoint,
        body: &B,
    ) -> Result<(), RequestError> {
        let ep = ep.clone().fresh(crate::Freshness::Never);
        self.validate(&ep, true)?;
        let body = crate::prepared::json(body, self.fetch.config.max_body)?;
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path);
        self.send(
            &ep,
            Prepared::new(url, body, ep.method, ep.idempotent),
            Decoder::probe(),
        )
        .map(|_| ())
    }

    /// Send checked JSON from a GET or form POST. A missing record returns `not_found`.
    pub fn json<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<T, RequestError> {
        self.parameters(ep, params, Decoder::json())
            .map(|(reply, _)| reply)
    }

    /// Send checked XML from a GET or form POST.
    pub fn xml<T: XmlReply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<T, RequestError> {
        self.parameters(ep, params, Decoder::xml())
            .map(|(reply, _)| reply)
    }

    /// Read XML and its own original fetch time without borrowing another reply's metadata.
    pub fn xml_received<T: XmlReply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.parameters(ep, params, Decoder::xml())
    }

    fn parameters<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        codec: Decoder<T>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
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
        let mut prepared = Prepared::new(url, body, ep.method, ep.idempotent);
        prepared.outgoing = params
            .iter()
            .filter_map(|(name, value)| match value {
                Arg::Contact(secret) => {
                    Some(((*name).to_owned(), crate::Secret::new(secret.value())))
                }
                _ => None,
            })
            .collect();
        self.send(ep, prepared, codec)
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
        let codec = Decoder::json();
        let body = crate::prepared::json(body, self.fetch.config.max_body)?;
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path);
        self.send(
            ep,
            Prepared::new(url, body, ep.method, ep.idempotent),
            codec,
        )
        .map(|(reply, _)| reply)
    }

    pub(crate) fn check_source(&self) -> Result<(), RequestError> {
        if self.fetch.disabled(self.spec.source) {
            return Err(self.failure(Reason::Disabled, None).into());
        }
        if self.no_key {
            return Err(self.failure(Reason::NoKey, None).into());
        }
        if self.wrong_clock {
            return Err(self.failure(Reason::Rejected, None).into());
        }
        Ok(())
    }
    fn validate(&self, ep: &Endpoint, json_body: bool) -> Result<(), RequestError> {
        self.check_source()?;
        if !std::ptr::eq(self.spec, ep.spec) {
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
    fn send<T: Reply>(
        &self,
        ep: &Endpoint,
        mut prepared: Prepared,
        codec: Decoder<T>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        prepared.identity_encoding = ep.identity_encoding;
        let codec = codec.octet_stream(ep.octet_stream);
        let keyed = self.keyed();
        let (reply, fetched) =
            self.cached(ep, &prepared.url, &prepared.identity, &codec, || {
                self.attempts(&prepared, keyed, &codec)
            })?;
        let metadata = crate::ReplyMetadata {
            identity: prepared.identity,
            fetched,
        };
        self.report
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .replies
            .push(metadata);
        Ok((reply, metadata))
    }

    pub(crate) fn keyed(&self) -> bool {
        match self.spec.key {
            KeyUse::Optional(c) | KeyUse::Required(c) => {
                self.fetch.config.secrets.get(c.secret).is_some()
            }
            KeyUse::None => false,
        }
    }
    fn attempts<T: Reply>(
        &self,
        prepared: &Prepared,
        keyed: bool,
        codec: &Decoder<T>,
    ) -> Result<(T, u16, Vec<u8>, i64), RequestError> {
        for attempt in 0..crate::retry::ATTEMPTS {
            let permit = self
                .fetch
                .limiters
                .acquire(self.spec.quota, keyed, self.budget())
                .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
            let started = Instant::now();
            let exchange = self.exchange(prepared, codec);
            self.observe_exchange(prepared, &exchange, started.elapsed());
            let (mut error, retryable, delay) = match exchange {
                Ok(response) => {
                    let status = response.status;
                    if (200..300).contains(&status) {
                        return self.decode(response, codec).map_err(Into::into);
                    }
                    let delay = response.retry_after;
                    (
                        self.failure(crate::retry::status_reason(status), Some(status)),
                        crate::retry::retry_status(status),
                        delay,
                    )
                }
                Err(Failure::Local(error)) => return Err(error),
                Err(Failure::Transport(error)) => {
                    let retryable = crate::retry::retry_error(&error);
                    let reason = crate::tls::reason(&error);
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

    fn observe_exchange(
        &self,
        prepared: &Prepared,
        exchange: &Result<Wire, Failure>,
        elapsed: Duration,
    ) {
        match exchange {
            Ok(response) => self.observe(
                &prepared.identity,
                Some(response.status),
                &response.body,
                &response.content_type,
                elapsed,
            ),
            Err(Failure::Local(RequestError::Source(error))) => {
                self.observe(&prepared.identity, error.status, &[], "", elapsed)
            }
            Err(_) => self.observe(&prepared.identity, None, &[], "", elapsed),
        }
    }

    pub(crate) fn wire_call(
        &self,
        prepared: &Prepared,
        accept: &str,
        headers: &[(&str, String)],
    ) -> Result<ureq::http::Response<ureq::Body>, ureq::Error> {
        let agent = self.fetch.agents.for_url(
            &self.fetch.config.network,
            self.fetch.config.proxy.is_some(),
            &prepared.url,
        );
        let mut request = if prepared.post.is_some() {
            agent.post(&prepared.url)
        } else {
            agent.get(&prepared.url).force_send_body()
        }
        .header("Accept", accept);
        for (name, value) in &prepared.outgoing {
            request = request.query(name, value.value());
        }
        if prepared.identity_encoding {
            request = request.header("Accept-Encoding", "identity");
        }
        for (name, value) in headers {
            request = request.header(*name, value);
        }
        if let KeyUse::Optional(c) | KeyUse::Required(c) = self.spec.key
            && let Some(secret) = self.fetch.config.secrets.get(c.secret)
        {
            request = match c.place {
                KeyPlace::Query(name) => request.query(name, secret),
                KeyPlace::Header(name) => request.header(name, secret),
                KeyPlace::Bearer => request.header("Authorization", format!("Bearer {secret}")),
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

    fn exchange<T: Reply>(&self, prepared: &Prepared, codec: &Decoder<T>) -> Result<Wire, Failure> {
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
                fetched: exchange.fetched,
                status: exchange.status,
                content_type: exchange.content_type,
                body: exchange.body.bytes().to_vec(),
                retry_after: exchange.retry_after_ms.map(Duration::from_millis),
            });
        }
        let started = Instant::now();
        let mut response = self
            .wire_call(prepared, codec.name(), &[])
            .map_err(Failure::Transport)?;
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
        let fetched = crate::cache::now();
        let recording = matches!(self.fetch.config.mode, crate::Mode::Record { .. });
        let body = self.read_body(&mut response, status, recording)?;
        if recording {
            let exchange = Exchange {
                fetched,
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
            fetched,
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

    fn decode<T: Reply>(
        &self,
        response: Wire,
        codec: &Decoder<T>,
    ) -> Result<(T, u16, Vec<u8>, i64), SourceError> {
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
        Ok((reply, status, response.body, response.fetched))
    }
}
