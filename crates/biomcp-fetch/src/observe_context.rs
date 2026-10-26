//! Optional causal context for bounded source diagnostics, never request behavior.
use crate::{Reply, prepared::Prepared};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

/// A source-owned identifier namespace and complete ordered selection.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Identities {
    /// Checked identifier system, such as PMID.
    pub namespace: String,
    /// Whole received or requested sequence; never silently sampled.
    pub ids: Vec<String>,
}
impl Identities {
    /// Bound diagnostic metadata; absent context cannot establish causation.
    pub fn bounded(namespace: &str, ids: Vec<String>) -> Option<Self> {
        (ids.len() <= 10_000 && ids.iter().all(|id| !id.is_empty() && id.len() <= 256))
            .then(|| Self { namespace: namespace.into(), ids })
    }
}
/// Explicit optional evidence for discovery-driven request changes.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Provenance {
    /// SHA-256 of the received body, without retaining provider values.
    pub response_digest: Option<String>,
    /// Complete checked discovery result identities.
    pub discovery: Option<Identities>,
    /// Explicit request ID inputs, excluding other parameters and credentials.
    pub inputs: Option<Identities>,
}
impl Provenance {
    pub(crate) fn received<T: Reply>(prepared: &Prepared, body: &[u8], status: Option<u16>) -> Self {
        let discovery = status.filter(|status| (200..300).contains(status)).and_then(|status| {
            let reply: T = serde_json::from_slice(body).ok()?;
            reply.check().ok()?;
            reply.check_status(status).ok()?;
            reply.discovery()
        });
        let inputs = T::input_parameter().and_then(|(namespace, parameter)| {
            let url = url::Url::parse(&prepared.url).ok()?;
            let mut values = url.query_pairs().filter(|(name, _)| name == parameter)
                .map(|(_, value)| value.into_owned()).collect::<Vec<_>>();
            if prepared.post.as_ref().is_some_and(|post|
                post.request_content_type == "application/x-www-form-urlencoded") {
                values.extend(url::form_urlencoded::parse(&prepared.body)
                    .filter(|(name, _)| name == parameter).map(|(_, value)| value.into_owned()));
            }
            let [value] = values.as_slice() else { return None; };
            Identities::bounded(namespace, value.split(',').map(str::to_owned).collect())
        });
        Self {
            response_digest: (!body.is_empty()).then(|| Sha256::digest(body).iter()
                .map(|byte| format!("{byte:02x}")).collect()),
            discovery,
            inputs,
        }
    }
}
