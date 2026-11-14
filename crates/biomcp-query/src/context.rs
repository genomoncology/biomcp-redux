use biomcp_fetch::Fetch;
use std::time::Duration;
/// Shared transport, command budget and outgoing-only source contact.
#[derive(Debug)]
pub struct Ctx<'a> {
    /// Optional outgoing-only NCBI contact.
    pub ncbi_email: Option<biomcp_fetch::Secret>,
    /// Shared checked transport.
    pub fetch: &'a Fetch,
    /// Absolute command budget shared by source legs.
    pub budget: Duration,
    /// Bypass cached reads on this request only.
    pub fresh: bool,
    /// Most names one search may send for a drug.
    pub drug_name_limit: usize,
    /// Most escaped bytes of label section text one reply returns, across all its sections.
    pub label_bytes: usize,
    /// Longest wait for each VariantValidator call of the transcript-fit check.
    pub fit_deadline: Duration,
}
/// Default for [`Ctx::drug_name_limit`].
pub const DRUG_NAME_LIMIT: usize = 30;
/// Default for [`Ctx::label_bytes`]: Keytruda's Indications and Usage, the largest measured
/// on 2026-11-13 at 23,683 bytes, takes four whole 6,144-byte fragments (24,576); that plus
/// 15 percent is 28,262, rounded up to whole KiB
/// (sdlc/tickets/0884-label-pages-fill-their-budget.md).
/// One reply shares it across the label sections it names.
pub const LABEL_BYTES: usize = 28 * 1024;
/// The PM ruling of 2026-11-13 allows label section text up to 32 KiB a reply.
pub const MAX_LABEL_BYTES: usize = 32 * 1024;
/// Default for [`Ctx::fit_deadline`], the limit for each VariantValidator call of the fit
/// check: the transcript list, then each group of up to 20 transcripts. Measured 2026-11-13:
/// the transcript list took at most 1.46 s; a fit query on 20 transcripts of CAMK2D, ANK2 or
/// TPM1 took at most 3.59 s, and on all of SCN1A's 14 other NM_ transcripts 2.48 s. The
/// slowest group plus one second of VariantValidator's request pacing is 4.6 s; 8 s leaves
/// room for the service running slower under load, as when CAMK2D's 47 transcripts took
/// 7.7 s in one run and 25.2 s in another (job j39842). When nothing can answer, the service
/// may instead fail after about 60 s (sdlc/tickets/0892-transcript-fit-is-quick-and-leads-on.md).
pub const FIT_DEADLINE: Duration = Duration::from_secs(8);
