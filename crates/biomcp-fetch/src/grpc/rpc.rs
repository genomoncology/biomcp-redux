//! Admit blocking RPC callers through the existing source policy, quota and deadline.
use super::{rpc_call::Event, worker::Command};
use crate::{BadReply, InputError, KeyPlace, KeyUse, Leg, Mode, RequestError, Secret, SourceSpec};
use biomcp_types::Reason;

/// A declared streaming method belonging to exactly one source.
#[derive(Clone, Copy, Debug)]
pub struct GrpcEndpoint {
    spec: &'static SourceSpec,
    path: &'static str,
}
impl SourceSpec {
    /// Declare one read-only streaming method. Every invocation makes one application attempt.
    pub const fn grpc(&'static self, path: &'static str) -> GrpcEndpoint {
        GrpcEndpoint { spec: self, path }
    }
}
impl crate::Fetch {
    /// Cancel queued and active RPCs and join the shared worker. Further RPCs fail closed.
    /// A started platform DNS lookup retains the documented system resolver exception.
    pub fn shutdown_grpc(&self) {
        self.grpc_worker.shutdown();
    }
}
impl Leg<'_> {
    /// Consume bounded protobuf messages using one shared quota permit through terminal status.
    /// Callbacks may see a prefix before an error; callers must publish results only after success.
    pub fn grpc(
        &self,
        endpoint: &GrpcEndpoint,
        body: &[u8],
        mut message: impl FnMut(&[u8]) -> Result<(), BadReply>,
    ) -> Result<(), RequestError> {
        self.check_source()?;
        if !std::ptr::eq(self.spec, endpoint.spec) || self.budget().virtual_time() {
            return Err(self.failure(Reason::Rejected, None).into());
        }
        if !matches!(self.fetch.mode, Mode::Live { .. }) {
            let reason = if self.fetch.mode == Mode::Offline {
                Reason::Offline
            } else {
                Reason::Rejected
            };
            return Err(self.failure(reason, None).into());
        }
        let limits = self.fetch.config.grpc_limits;
        limits
            .check_request(body)
            .map_err(|_| InputError::BodyLimit)?;
        let (uri, path) = checked(self.spec.base, endpoint.path, limits.metadata_bytes)?;
        let key = self.rpc_key()?;
        let proxy = self
            .fetch
            .config
            .network
            .rpc_proxy(self.fetch.config.proxy.as_ref(), &uri)
            .map_err(|_| self.failure(Reason::Proxy, None))?;
        let _permit = self
            .fetch
            .limiters
            .acquire(self.spec.quota, self.keyed(), self.budget())
            .ok_or_else(|| self.failure(Reason::DeadlineExceeded, None))?;
        let deadline = self.budget().local();
        let (events, _guard) = self
            .fetch
            .grpc_worker
            .submit(&deadline, |events, done, stop, state| Command {
                uri,
                path,
                proxy,
                key: key.map(|(name, value)| (name, Secret::new(value))),
                body: prost::bytes::Bytes::copy_from_slice(body),
                limits,
                deadline: self.budget().local(),
                stop,
                events,
                done,
                state,
            })
            .map_err(|reason| self.failure(reason, None))?;
        loop {
            if self.budget().expired() {
                return Err(self.failure(Reason::DeadlineExceeded, None).into());
            }
            match events.recv_timeout(self.budget().wait_slice(self.budget().remaining())) {
                Ok(Event::Message(bytes)) => {
                    message(&bytes).map_err(|_| self.failure(Reason::BadReply, None))?;
                    if self.budget().expired() {
                        return Err(self.failure(Reason::DeadlineExceeded, None).into());
                    }
                }
                Ok(Event::End(result)) => {
                    return result.map_err(|reason| self.failure(reason, None).into());
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                Err(std::sync::mpsc::RecvTimeoutError::Disconnected) => {
                    return _guard
                        .state
                        .outcome()
                        .unwrap_or(Err(Reason::Unavailable))
                        .map_err(|reason| self.failure(reason, None).into());
                }
            }
        }
    }
    fn rpc_key(&self) -> Result<Option<(&'static str, &str)>, RequestError> {
        let credential = match self.spec.key {
            KeyUse::None => return Ok(None),
            KeyUse::Required(c) | KeyUse::Optional(c) => c,
        };
        let Some(value) = self.fetch.secrets.get(credential.secret) else {
            return Ok(None);
        };
        let name = match credential.place {
            KeyPlace::Header(name) => name,
            _ => return Err(self.failure(Reason::Rejected, None).into()),
        };
        if name.ends_with("-bin")
            || value.len().saturating_add(name.len()) > self.fetch.config.grpc_limits.metadata_bytes
        {
            return Err(self.failure(Reason::Rejected, None).into());
        }
        Ok(Some((name, value)))
    }
}
fn checked(
    base: &str,
    path: &str,
    limit: usize,
) -> Result<(ureq::http::Uri, ureq::http::uri::PathAndQuery), RequestError> {
    if base.len().saturating_add(path.len()) > limit {
        return Err(InputError::Path.into());
    }
    let parsed = url::Url::parse(base).map_err(|_| InputError::Path)?;
    if parsed.scheme() != "https"
        || !parsed.username().is_empty()
        || parsed.password().is_some()
        || parsed.path() != "/"
        || parsed.query().is_some()
        || parsed.fragment().is_some()
        || !path.starts_with('/')
        || path.contains(['?', '#'])
        || path.split('/').count() != 3
    {
        return Err(InputError::Path.into());
    }
    Ok((
        base.parse().map_err(|_| InputError::Path)?,
        path.parse().map_err(|_| InputError::Path)?,
    ))
}
