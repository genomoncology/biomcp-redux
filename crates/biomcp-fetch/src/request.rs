use crate::record::{Body, Exchange};
use crate::xml::{Decoder, XmlReply};
use crate::{Arg, Endpoint, Leg, Reply, RequestError, SourceError};
use crate::{InputError, declaration::Method, prepared::Prepared};
use biomcp_types::Reason;
use std::time::{Duration, Instant};

pub(crate) type Decoded<T> = (
    T,
    u16,
    Vec<u8>,
    i64,
    Option<String>,
    Option<crate::Pagination>,
);
#[derive(serde::Serialize)]
struct Wire {
    fetched: i64,
    status: u16,
    content_type: String,
    content_range: Option<String>,
    pagination: Option<crate::Pagination>,
    #[serde(skip)] // Raw and decoded body scanning stays separate from metadata.
    body: Vec<u8>,
    retry_after: Option<Duration>,
}
pub(crate) enum Failure {
    Transport(ureq::Error),
    Local(RequestError),
}

impl Leg<'_> {
    /// Read opaque bytes from an endpoint declaring its own bounded binary policy.
    pub fn bytes(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<Vec<u8>, RequestError> {
        let limit = ep
            .byte_limit
            .filter(|v| *v > 0 && *v < u64::MAX)
            .ok_or(InputError::BodyLimit)?;
        self.parameters(ep, params, Decoder::bytes(limit))
            .map(|(v, _)| v.0)
    }
    /// Read binary bytes only after the source-owned parser accepts them for cache admission.
    pub fn bytes_checked(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        check: impl Fn(&[u8]) -> Result<(), Reason>,
    ) -> Result<Vec<u8>, RequestError> {
        let limit = ep
            .byte_limit
            .filter(|v| *v > 0 && *v < u64::MAX)
            .ok_or(InputError::BodyLimit)?;
        self.parameters(ep, params, Decoder::checked_bytes(limit, &check))
            .map(|(v, _)| v.0)
    }
    /// Probe declared reachability through the same bounded transport, quota and replay policy.
    /// Probe replies never enter the checked data cache.
    pub fn probe(&self, ep: &Endpoint, params: &[(&str, Arg<'_>)]) -> Result<(), RequestError> {
        let mut ep = ep.clone().fresh(crate::Freshness::Never);
        // Reachability is one observation, never a retrying data request.
        ep.idempotent = false;
        self.parameters(&ep, params, Decoder::probe()).map(|_| ())
    }
    /// Probe a declared read-only JSON endpoint with a bounded request body.
    pub fn probe_json<B: serde::Serialize>(
        &self,
        ep: &Endpoint,
        body: &B,
    ) -> Result<(), RequestError> {
        let mut ep = ep.clone().fresh(crate::Freshness::Never);
        // Reachability is one observation, never a retrying data request.
        ep.idempotent = false;
        self.validate(&ep, true)?;
        let body = crate::prepared::json(body, self.fetch.config.max_body)?;
        let url = if ep.publisher {
            ep.path.to_string()
        } else {
            format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path)
        };
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

    /// Read checked JSON and the timestamp belonging to that received record.
    pub fn json_received<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.parameters(ep, params, Decoder::json())
    }

    /// Apply a request-bound JSON check before cache admission and on cache or replay reads.
    pub fn json_received_checked<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        check: impl Fn(&T) -> Result<(), crate::BadReply>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.parameters(ep, params, Decoder::checked_json(&check))
    }
    /// Decode exactly one JSON-string wrapper, then validate the typed inner reply.
    /// Cache and replay reads use the same decoder and request-bound check.
    pub fn json_string_received_checked<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        check: impl Fn(&T) -> Result<(), crate::BadReply>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.parameters(ep, params, Decoder::checked_json_string(&check))
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

    pub(crate) fn parameters<T: Reply>(
        &self,
        ep: &Endpoint,
        params: &[(&str, Arg<'_>)],
        codec: Decoder<T>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        self.validate(ep, false)?;
        let url = if ep.publisher {
            ep.path.to_string()
        } else {
            format!("{}{}", self.spec.base.trim_end_matches('/'), ep.path)
        };
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
    pub(crate) fn validate(&self, ep: &Endpoint, json_body: bool) -> Result<(), RequestError> {
        self.check_source()?;
        if !std::ptr::eq(self.spec, ep.spec) {
            return Err(self.failure(Reason::Rejected, None).into());
        }
        if (ep.method == Method::Json) != json_body {
            return Err(InputError::Method.into());
        }
        if ep.publisher {
            let uri = ep
                .path
                .parse::<ureq::http::Uri>()
                .map_err(|_| InputError::Path)?;
            if self
                .fetch
                .config
                .network
                .requires_proxy(self.fetch.config.proxy.is_some(), &uri)
            {
                return Err(self.failure(Reason::Proxy, None).into());
            }
        }
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None).into());
        }
        Ok(())
    }
    pub(crate) fn send<T: Reply>(
        &self,
        ep: &Endpoint,
        mut prepared: Prepared,
        codec: Decoder<T>,
    ) -> Result<(T, crate::ReplyMetadata), RequestError> {
        prepared.identity_encoding = ep.identity_encoding;
        prepared.exact_count = ep.exact_count;
        if ep.exact_count {
            use sha2::Digest;
            let mut hash = sha2::Sha256::new();
            hash.update(b"Prefer:count=exact\0");
            hash.update(prepared.identity);
            prepared.identity = hash.finalize().into();
        }
        prepared.publisher = ep.publisher;
        let mut codec = codec.octet_stream(ep.octet_stream);
        if let Some(limit) = ep.byte_limit {
            if limit == 0 || limit == u64::MAX {
                return Err(InputError::BodyLimit.into());
            }
            codec.limit = Some(codec.limit.map_or(limit, |old| old.min(limit)));
        }
        codec.json_rejections = ep.json_rejections && codec.name() == "application/json";
        codec.json_absence = ep.json_absence && codec.name() == "application/json";
        let keyed = self.keyed();
        let (reply, fetched, content_range, pagination) =
            self.cached(ep, &prepared, &codec, || {
                self.attempts(&prepared, keyed, &codec)
            })?;
        let metadata = crate::ReplyMetadata {
            identity: prepared.identity,
            fetched,
            content_range,
            pagination,
        };
        self.report
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .replies
            .push(metadata.clone());
        Ok((reply, metadata))
    }

    fn attempts<T: Reply>(
        &self,
        prepared: &Prepared,
        keyed: bool,
        codec: &Decoder<T>,
    ) -> Result<Decoded<T>, RequestError> {
        for attempt in 0..crate::retry::ATTEMPTS {
            let permit = self
                .fetch
                .limiters
                .acquire(self.spec.quota, keyed, self.budget())
                .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
            let started = Instant::now();
            let exchange = self.exchange(prepared, codec);
            self.observe_exchange::<T>(prepared, &exchange, started.elapsed());
            let (mut error, retryable, delay) = match exchange {
                Ok(response) => {
                    let status = response.status;
                    if (200..300).contains(&status)
                        || codec.json_rejections && matches!(status, 400 | 422)
                        || codec.json_absence && status == 404
                    {
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

    fn observe_exchange<T: Reply>(
        &self,
        prepared: &Prepared,
        exchange: &Result<Wire, Failure>,
        elapsed: Duration,
    ) {
        let (status, body, media) = match exchange {
            Ok(response) => (
                Some(response.status),
                response.body.as_slice(),
                response.content_type.as_str(),
            ),
            Err(Failure::Local(RequestError::Source(error))) => (error.status, &[][..], ""),
            Err(_) => (None, &[][..], ""),
        };
        if self.fetch.config.observations.is_some() {
            self.observed(
                &prepared.identity,
                status,
                body.len() as u64,
                crate::Shape::inspect(body, media),
                elapsed,
                (
                    std::any::type_name::<T>(),
                    false,
                    &prepared.url,
                    crate::Provenance::received::<T>(prepared, body, status),
                ),
            );
        }
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
                codec.limit.unwrap_or(self.fetch.config.max_body),
            )
            .map_err(|error| {
                if codec.limit.is_some() && error == crate::RecordError::Limit {
                    Failure::Local(self.failure(Reason::BadReply, None).into())
                } else {
                    local(error)
                }
            })?
        {
            return self.replayed_wire(prepared, exchange);
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
        let content_range = response
            .headers()
            .get("content-range")
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let pagination = self.pagination_headers(prepared, &response, status)?;
        let retry_after = response
            .headers()
            .get("retry-after")
            .and_then(|v| v.to_str().ok())
            .and_then(|v| crate::retry::retry_after(v, std::time::SystemTime::now()));
        let fetched = crate::cache::now();
        let recording = matches!(self.fetch.mode, crate::Mode::Record { .. });
        let body = self.read_body(
            &mut response,
            status,
            recording
                || self.keyed()
                || !prepared.outgoing.is_empty()
                || codec.json_rejections && matches!(status, 400 | 422)
                || codec.json_absence && status == 404,
            codec.limit.unwrap_or(self.fetch.config.max_body),
        )?;
        let wire = Wire {
            fetched,
            status,
            content_type,
            content_range,
            pagination,
            body,
            retry_after,
        };
        self.contain_response(prepared, &wire, &wire.body, status)
            .map_err(Failure::Local)?;
        if recording {
            self.record_wire(prepared, codec, &wire, started)
                .map_err(local)?;
        }
        Ok(wire)
    }
    fn replayed_wire(&self, prepared: &Prepared, mut exchange: Exchange) -> Result<Wire, Failure> {
        if !self
            .budget()
            .advance(Duration::from_millis(exchange.delay_ms))
        {
            return Err(Failure::Local(
                self.failure(Reason::DeadlineExceeded, Some(exchange.status))
                    .into(),
            ));
        }
        // Sidecars become non-serializable Bytes on read. Scan their raw body
        // separately while deriving metadata from every remaining exchange field.
        let body = std::mem::replace(&mut exchange.body, Body::Text(String::new()));
        self.contain_response(prepared, &exchange, body.bytes(), exchange.status)
            .map_err(Failure::Local)?;
        Ok(Wire {
            fetched: exchange.fetched,
            status: exchange.status,
            content_type: exchange.content_type,
            content_range: exchange.content_range,
            pagination: exchange.pagination,
            body: body.bytes().to_vec(),
            retry_after: exchange.retry_after_ms.map(Duration::from_millis),
        })
    }
    fn record_wire<T: Reply>(
        &self,
        prepared: &Prepared,
        codec: &Decoder<T>,
        wire: &Wire,
        started: Instant,
    ) -> Result<(), crate::RecordError> {
        let exchange = Exchange {
            rpc: None,
            fetched: wire.fetched,
            status: wire.status,
            content_type: wire.content_type.clone(),
            content_range: wire.content_range.clone(),
            pagination: wire.pagination.clone(),
            body: if codec.binary() {
                Body::Bytes(wire.body.clone())
            } else {
                Body::capture(wire.body.clone())?
            },
            delay_ms: crate::record::millis(started.elapsed())?,
            retry_after_ms: wire.retry_after.map(crate::record::millis).transpose()?,
        };
        self.fetch.recordings.append(
            self.spec.source,
            &prepared.identity,
            &prepared.url,
            prepared.post.as_ref(),
            exchange,
        )
    }

    fn decode<T: Reply>(
        &self,
        response: Wire,
        codec: &Decoder<T>,
    ) -> Result<Decoded<T>, SourceError> {
        let status = response.status;
        let bad = || self.failure(Reason::BadReply, Some(status));
        if !codec.accepts(&response.content_type) {
            return Err(bad());
        }
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, Some(status)));
        }
        codec
            .validate(&response.body)
            .map_err(|reason| self.failure(reason, Some(status)))?;
        let reply = codec
            .decode_response(&response.body, response.content_range.as_deref(), status)
            .ok_or_else(bad)?;
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, Some(status)));
        }
        Ok((
            reply,
            status,
            response.body,
            response.fetched,
            response.content_range,
            response.pagination,
        ))
    }
}
