//! Checked cache reuse around the single live request path.
use crate::xml::Decoder;
use crate::{
    Endpoint, Leg, Mode, Reply, RequestError,
    cache::{Cache, Entry, now},
};
use biodata::Reason;

const NEGATIVE_SECONDS: u64 = 600;
enum Checked<T> {
    Positive(T, bool, i64),
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
        url: &str,
        key: &[u8; 32],
        codec: &Decoder<T>,
        live: impl FnOnce() -> Result<(T, u16, Vec<u8>, i64), RequestError>,
    ) -> Result<(T, i64), RequestError> {
        if self.budget().expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None).into());
        }
        if self.fetch.config.mode == Mode::Offline {
            return self.offline(endpoint, key, codec);
        }
        let Some((cache, seconds)) = self.fetch.cache.as_ref().zip(endpoint.fresh.seconds()) else {
            return live().map(|(reply, _, _, fetched)| (reply, fetched));
        };
        let mut stale = None;
        if matches!(self.fetch.config.mode, Mode::Live { .. }) && !self.fresh {
            match self.checked_cached(cache, key, codec)? {
                Some(Checked::Positive(reply, true, fetched)) => return Ok((reply, fetched)),
                Some(Checked::Positive(reply, false, fetched)) => stale = Some((reply, fetched)),
                Some(Checked::Absent) => {
                    return Err(self.failure(Reason::NotFound, Some(404)).into());
                }
                None => {}
            }
        }
        match live() {
            Ok((reply, status, body, fetched)) => {
                let mut entry = Entry::new(status, body, seconds, codec.name());
                entry.fetched_at = fetched;
                if (200..300).contains(&status) && codec.cacheable(&reply) {
                    cache.write(key, self.spec.source, url, &entry, self.budget())?;
                }
                Ok((reply, fetched))
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
        key: &[u8; 32],
        codec: &Decoder<T>,
    ) -> Result<(T, i64), RequestError> {
        if let Some(cache) = self
            .fetch
            .cache
            .as_ref()
            .filter(|_| endpoint.fresh.seconds().is_some())
        {
            match self.checked_cached(cache, key, codec)? {
                Some(Checked::Positive(reply, fresh, fetched)) => {
                    if !fresh {
                        self.report
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .stale = true;
                    }
                    return Ok((reply, fetched));
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
        key: &[u8; 32],
        codec: &Decoder<T>,
    ) -> Result<Option<Checked<T>>, RequestError> {
        let Some(entry) = cache.read(
            key,
            codec.limit.unwrap_or(self.fetch.config.max_body),
            self.budget(),
        )?
        else {
            return Ok(None);
        };
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
        .then(|| codec.decode(&entry.body))
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
        Ok(Some(Checked::Positive(reply, fresh, entry.fetched_at)))
    }
}
