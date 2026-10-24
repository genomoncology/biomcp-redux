use biomcp_fetch::Fetch;
use std::time::Duration;
/// Shared transport and command budget; no persistent query store is needed.
#[derive(Debug)]
pub struct Ctx<'a> {
    /// Checked transport.
    /// Optional outgoing-only NCBI contact.
    pub ncbi_email: Option<biomcp_fetch::Secret>,
    /// Shared checked transport.
    pub fetch: &'a Fetch,
    /// Absolute command budget shared by source legs.
    pub budget: Duration,
    /// Bypass cached reads on this request only.
    pub fresh: bool,
}
