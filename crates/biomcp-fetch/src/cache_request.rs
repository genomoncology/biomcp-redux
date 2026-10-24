//! Checked cache reuse around the single live request path.
use crate::{
    CacheError, Endpoint, Leg, Mode, Reply, RequestError, SourceError,
    cache::{Entry, now},
};
use biodata::Reason;

const NEGATIVE_SECONDS: i64 = 600;

impl Leg<'_> {
    pub(crate) fn cached<T: Reply>(
        &self,
        endpoint: &Endpoint,
        url: &str,
        key: &[u8; 32],
        live: impl FnOnce() -> Result<(T, u16, Vec<u8>), SourceError>,
    ) -> Result<T, RequestError> {
        if self.deadline.expired() {
            return Err(self.failure(Reason::DeadlineExceeded, None).into());
        }
        let Some((cache, seconds)) = self.fetch.cache.as_ref().zip(endpoint.fresh.seconds()) else {
            return live().map(|(reply, _, _)| reply).map_err(Into::into);
        };
        let mut stale = None;
        if self.fetch.config.mode == (Mode::Live { fresh: false })
            && let Some(entry) = cache.read(key, self.fetch.config.max_body, self.deadline)?
        {
            if entry.status == 404 {
                if entry.fresh_until > now() {
                    return Err(self.failure(Reason::NotFound, Some(404)).into());
                }
            } else {
                let reply: T =
                    serde_json::from_slice(&entry.body).map_err(|_| CacheError::InvalidReply)?;
                reply.check().map_err(|_| CacheError::InvalidReply)?;
                if self.deadline.expired() {
                    return Err(self
                        .failure(Reason::DeadlineExceeded, Some(entry.status))
                        .into());
                }
                if entry.fresh_until > now() {
                    return Ok(reply);
                }
                stale = Some(reply);
            }
        }
        match live() {
            Ok((reply, status, body)) => {
                let fresh_until = now().saturating_add(i64::try_from(seconds).unwrap_or(i64::MAX));
                cache.write(
                    key,
                    self.spec.source,
                    url,
                    &Entry {
                        status,
                        body,
                        fresh_until,
                    },
                    self.deadline,
                )?;
                Ok(reply)
            }
            Err(error) if error.reason == Reason::NotFound => {
                // A current absence replaces a formerly positive entry; no stale
                // result can override it and the provider body is never stored.
                cache.write(
                    key,
                    self.spec.source,
                    url,
                    &Entry {
                        status: 404,
                        body: Vec::new(),
                        fresh_until: now().saturating_add(NEGATIVE_SECONDS),
                    },
                    self.deadline,
                )?;
                Err(error.into())
            }
            Err(error) => {
                if let Some(reply) = stale {
                    let mut report = self.report.borrow_mut();
                    report.stale = true;
                    report.failures.push(error);
                    Ok(reply)
                } else {
                    Err(error.into())
                }
            }
        }
    }
}
