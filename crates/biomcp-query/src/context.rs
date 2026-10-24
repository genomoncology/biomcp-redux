use biomcp_fetch::Fetch;
use std::time::Duration;
/// Shared transport and command budget; no persistent query store is needed.
#[derive(Debug)]
pub struct Ctx<'a> {
    /// Checked transport.
    pub fetch: &'a Fetch,
    /// Absolute command budget shared by source legs.
    pub budget: Duration,
}
