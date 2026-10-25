//! Checked cache reuse around the single live request path.
use crate::xml::Decoder;
use crate::{
    Endpoint, Leg, Mode, Reply, RequestError,
    cache::{Cache, Entry, now},
};
use biomcp_types::Reason;

const NEGATIVE_SECONDS: u64 = 600;
enum Checked<T> {
    Positive(T, bool, i64, Option<crate::ContentRange>),
    Absent,
}

impl Leg<'_> {
    /// Apply request-bound checks on every read and admit only cacheable decoded replies.
    /// A valid provider rejection can return to the caller without entering the cache.
    pub fn json_checked_admission<T: Reply>(
        &self,
        endpoint: &Endpoint,
        params: &[(&str, crate::Arg<'_>)],
        check: impl Fn(&T) -> Result<(), crate::BadReply>,
        cache_if: impl Fn(&T) -> bool,
    ) -> Result<T, RequestError> {
        self.parameters(
            endpoint,
            params,
            Decoder::checked_json(&check).cache_if(&cache_if),
        )
        .map(|(reply, _)| reply)
    }

    /// Return checked JSON while withholding a response from cache when request identity differs.
    /// The same predicate rejects pre-existing entries before either fresh or stale reuse.
    pub fn json_checked_cache<T: Reply>(
        &self,
        endpoint: &Endpoint,
        params: &[(&str, crate::Arg<'_>)],
        cache_if: impl Fn(&T) -> bool,
    ) -> Result<T, RequestError> {
        self.parameters(endpoint, params, Decoder::json().cache_if(&cache_if))
            .map(|(reply, _)| reply)
    }

    pub(crate) fn cached<T: Reply>(
        &self,
        endpoint: &Endpoint,
        prepared: &crate::prepared::Prepared,
        codec: &Decoder<T>,
        live: impl FnOnce() -> Result<crate::request::Decoded<T>, RequestError>,
    ) -> Result<(T, i64, Option<crate::ContentRange>), RequestError> {
        let (url, key) = (prepared.url.as_str(), &prepared.identity);
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None).into());
        }
        if self.fetch.config.mode == Mode::Offline {
            return self.offline(endpoint, prepared, codec);
        }
        let Some((cache, seconds)) = self.fetch.cache.as_ref().zip(endpoint.fresh.seconds()) else {
            return live().map(|(reply, _, _, fetched, range)| {
                (
                    reply,
                    fetched,
                    range.as_deref().and_then(crate::ContentRange::parse),
                )
            });
        };
        let mut stale = None;
        if matches!(self.fetch.config.mode, Mode::Live { .. }) && !self.fresh {
            match self.checked_cached(cache, prepared, codec)? {
                Some(Checked::Positive(reply, true, fetched, range)) => {
                    return Ok((reply, fetched, range));
                }
                Some(Checked::Positive(reply, false, fetched, range)) => {
                    stale = Some((reply, fetched, range))
                }
                Some(Checked::Absent) => {
                    return Err(self.failure(Reason::NotFound, Some(404)).into());
                }
                None => {}
            }
        }
        match live() {
            Ok((reply, status, body, fetched, range)) => {
                let mut entry = Entry::new(status, body, seconds, codec.name());
                entry.fetched_at = fetched;
                entry.content_range = range;
                if (200..300).contains(&status) && codec.cacheable(&reply) {
                    cache.write(key, self.spec.source, url, &entry, self.budget())?;
                } else if codec.json_absence && status == 404 {
                    cache.remove(key, self.budget())?;
                }
                Ok((
                    reply,
                    fetched,
                    entry
                        .content_range
                        .as_deref()
                        .and_then(crate::ContentRange::parse),
                ))
            }
            Err(RequestError::Source(error)) if error.reason == Reason::NotFound => {
                // A current absence replaces a formerly positive entry; no stale
                // result can override it and the provider body is never stored.
                cache.write(
                    key,
                    self.spec.source,
                    url,
                    &Entry::new(404, Vec::new(), NEGATIVE_SECONDS, codec.name()),
                    self.budget(),
                )?;
                Err(error.into())
            }
            Err(RequestError::Source(error)) => {
                if let Some(reply) = stale
                    && !(matches!(self.spec.key, crate::KeyUse::Required(_))
                        && matches!(error.status, Some(401 | 403)))
                {
                    let mut report = self
                        .report
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    report.stale = true;
                    report.failures.push(error);
                    Ok(reply)
                } else {
                    Err(error.into())
                }
            }
            Err(error) => Err(error),
        }
    }

    fn offline<T: Reply>(
        &self,
        endpoint: &Endpoint,
        prepared: &crate::prepared::Prepared,
        codec: &Decoder<T>,
    ) -> Result<(T, i64, Option<crate::ContentRange>), RequestError> {
        if let Some(cache) = self
            .fetch
            .cache
            .as_ref()
            .filter(|_| endpoint.fresh.seconds().is_some())
        {
            match self.checked_cached(cache, prepared, codec)? {
                Some(Checked::Positive(reply, fresh, fetched, range)) => {
                    if !fresh {
                        self.report
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .stale = true;
                    }
                    return Ok((reply, fetched, range));
                }
                Some(Checked::Absent) => {
                    return Err(self.failure(Reason::NotFound, Some(404)).into());
                }
                None => {}
            }
        }
        Err(self.failure(Reason::Offline, None).into())
    }

    fn checked_cached<T: Reply>(
        &self,
        cache: &Cache,
        prepared: &crate::prepared::Prepared,
        codec: &Decoder<T>,
    ) -> Result<Option<Checked<T>>, RequestError> {
        let key = &prepared.identity;
        let Some(entry) = cache.read(
            key,
            codec.limit.unwrap_or(self.fetch.config.max_body),
            self.budget(),
        )?
        else {
            return Ok(None);
        };
        self.contain_response(
            prepared,
            entry.content_type.as_deref().unwrap_or(""),
            &entry.body,
            entry.status,
        )?;
        let fresh = entry.fresh_until > now();
        if entry.status == 404 {
            if fresh {
                self.observed(
                    key,
                    Some(404),
                    0,
                    crate::Shape::default(),
                    std::time::Duration::ZERO,
                    (std::any::type_name::<T>(), true),
                );
            }
            return Ok(fresh.then_some(Checked::Absent));
        }
        let reply = (entry.content_type.as_deref() == Some(codec.name())
            && codec.validate(&entry.body).is_ok())
        .then(|| codec.decode_response(&entry.body, entry.content_range.as_deref(), entry.status))
        .flatten()
        .filter(|reply| codec.cacheable(reply));
        let Some(reply) = reply else {
            // A type or limit change invalidates this entry. It is a miss and
            // cannot become a stale fallback after the live request fails.
            cache.remove(key, self.budget())?;
            return Ok(None);
        };
        if self.budget().expired() {
            return Err(self
                .failure(Reason::DeadlineExceeded, Some(entry.status))
                .into());
        }
        if self.fetch.config.observations.is_some()
            && (fresh || self.fetch.config.mode == Mode::Offline)
        {
            self.observed(
                key,
                Some(entry.status),
                0,
                crate::Shape::inspect(&entry.body, codec.name()),
                std::time::Duration::ZERO,
                (std::any::type_name::<T>(), true),
            );
        }
        Ok(Some(Checked::Positive(
            reply,
            fresh,
            entry.fetched_at,
            entry
                .content_range
                .as_deref()
                .and_then(crate::ContentRange::parse),
        )))
    }
}
