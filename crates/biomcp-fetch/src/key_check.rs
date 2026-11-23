//! A credential probe shares source admission and replay, with one uncached GET.
use crate::{
    Arg, Leg, Mode, Read, RequestError, declaration::Method, prepared::Prepared, request::Failure,
};
use biomcp_types::Reason;
use std::time::Instant;

impl Leg<'_> {
    /// Inspect the status and contained body of one authenticated probe reply.
    /// The adapter must publish only recognized publisher reasons, never raw response text.
    pub fn inspect_key<T>(
        &self,
        path: &'static str,
        params: &[(&str, Arg<'_>)],
        inspect: impl FnOnce(u16, &[u8]) -> T,
    ) -> Result<T, RequestError> {
        self.validate(&self.spec.get(path), false)?;
        if !self.keyed() {
            return Err(self.failure(Reason::NoKey, None).into());
        }
        if self.fetch.mode == Mode::Offline {
            return Err(self.failure(Reason::Offline, None).into());
        }
        let url = format!("{}{}", self.spec.base.trim_end_matches('/'), path);
        let url = crate::retry::canonical(&url, params, self.spec.key);
        self.spec.admit(&url, false)?;
        let request = Prepared::new(url, Vec::new(), Method::Get, false);
        let read =
            Read::probe().bounded(self.fetch.config.max_body.min(256 * 1024), Reason::TooLarge);
        let _permit = self
            .acquire_quota()
            .map_err(|delay| self.quota_failure(delay))?
            .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
        let started = Instant::now();
        let exchange = self.exchange(&request, &read, None);
        self.observe_exchange(&request, &exchange, started.elapsed(), &read);
        match exchange {
            Ok(wire) => {
                let result = inspect(wire.status, &wire.body);
                if self.budget().expired() {
                    return Err(self
                        .failure(Reason::DeadlineExceeded, Some(wire.status))
                        .into());
                }
                Ok(result)
            }
            Err(Failure::Local(error)) => Err(error),
            Err(Failure::Transport(error)) => {
                let mut failure = self.failure(crate::tls::reason(&error), None);
                failure.tls_cause = crate::tls::cause(&error);
                Err(failure.into())
            }
        }
    }
}
