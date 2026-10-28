//! Opt-in complete JSON observations distinguish byte overflow from malformed replies.
use crate::{Arg, Endpoint, InputError, Leg, Reply, RequestError, xml::Decoder};
impl Leg<'_> {
    /// Read one checked JSON observation under a byte bound. Overflow reports `too_large`.
    /// Existing JSON and opaque byte readers retain their own error policy.
    pub fn json_bounded<T: Reply>(
        &self,
        endpoint: &Endpoint,
        params: &[(&str, Arg<'_>)],
        limit: u64,
    ) -> Result<T, RequestError> {
        if limit == 0 || limit == u64::MAX {
            return Err(InputError::BodyLimit.into());
        }
        let mut codec = Decoder::json();
        codec.limit = Some(limit.min(self.fetch.config.max_body));
        codec.overflow_reason = biomcp_types::Reason::TooLarge;
        self.parameters(endpoint, params, codec)
            .map(|(reply, _)| reply)
    }
}
