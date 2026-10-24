//! Checked cache reuse around the single live request path.
use crate::{
    Endpoint, Leg, Mode, Reply, RequestError,
    cache::{Cache, Entry, now},
};
use biodata::Reason;

const NEGATIVE_SECONDS: u64 = 600;
enum Checked<T> {
    Positive(T, bool),
    Absent,
}

impl Leg<'_> {
    pub(crate) fn cached<T: Reply>(
        &self,
        endpoint: &Endpoint,
        url: &str,
        key: &[u8; 32],
        live: impl FnOnce() -> Result<(T, u16, Vec<u8>), RequestError>,
    ) -> Result<T, RequestError> {
        if self.deadline.expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None).into());
        }
        if self.fetch.config.mode == Mode::Offline {
            return self.offline(endpoint, key);
        }
        let Some((cache, seconds)) = self.fetch.cache.as_ref().zip(endpoint.fresh.seconds()) else {
            return live().map(|(reply, _, _)| reply);
        };
        let mut stale = None;
        if self.fetch.config.mode == (Mode::Live { fresh: false }) {
            match self.checked_cached(cache, key)? {
                Some(Checked::Positive(reply, true)) => return Ok(reply),
                Some(Checked::Positive(reply, false)) => stale = Some(reply),
                Some(Checked::Absent) => {
                    return Err(self.failure(Reason::NotFound, Some(404)).into());
                }
                None => {}
            }
        }
        match live() {
            Ok((reply, status, body)) => {
                cache.write(
                    key,
                    self.spec.source,
                    url,
                    &Entry::new(status, body, seconds),
                    self.deadline,
                )?;
                Ok(reply)
            }
            Err(RequestError::Source(error)) if error.reason == Reason::NotFound => {
                // A current absence replaces a formerly positive entry; no stale
                // result can override it and the provider body is never stored.
                cache.write(
                    key,
                    self.spec.source,
                    url,
                    &Entry::new(404, Vec::new(), NEGATIVE_SECONDS),
                    self.deadline,
                )?;
                Err(error.into())
            }
            Err(RequestError::Source(error)) => {
                if let Some(reply) = stale {
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

    fn offline<T: Reply>(&self, endpoint: &Endpoint, key: &[u8; 32]) -> Result<T, RequestError> {
        if let Some(cache) = self
            .fetch
            .cache
            .as_ref()
            .filter(|_| endpoint.fresh.seconds().is_some())
        {
            match self.checked_cached(cache, key)? {
                Some(Checked::Positive(reply, fresh)) => {
                    if !fresh {
                        self.report
                            .lock()
                            .unwrap_or_else(std::sync::PoisonError::into_inner)
                            .stale = true;
                    }
                    return Ok(reply);
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
    ) -> Result<Option<Checked<T>>, RequestError> {
        let Some(entry) = cache.read(key, self.fetch.config.max_body, self.deadline)? else {
            return Ok(None);
        };
        let fresh = entry.fresh_until > now();
        if entry.status == 404 {
            return Ok(fresh.then_some(Checked::Absent));
        }
        let reply = serde_json::from_slice::<T>(&entry.body)
            .ok()
            .filter(|reply| reply.check().is_ok());
        let Some(reply) = reply else {
            // A type or limit change invalidates this entry. It is a miss and
            // cannot become a stale fallback after the live request fails.
            cache.remove(key, self.deadline)?;
            return Ok(None);
        };
        if self.deadline.expired() {
            return Err(self
                .failure(Reason::DeadlineExceeded, Some(entry.status))
                .into());
        }
        Ok(Some(Checked::Positive(reply, fresh)))
    }
}
