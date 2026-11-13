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
    /// Longest wait for VariantValidator's transcript-fit check after a reference mismatch.
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
/// Default for [`Ctx::fit_deadline`]. Measured 2026-11-13 on VariantValidator for SCN1A,
/// DMD, NF1, BRCA2, TTN, CDKL5, NRXN1, CACNA1C, ANK3, GAPDH and TP53 (2 to 24 transcripts): the
/// transcript list took at most 1.43 s and the fit query at most 4.47 s (NRXN1). Their sum,
/// plus one second of VariantValidator's request pacing, is 6.9 s; that plus 15 percent,
/// rounded up, is 8 s. When nothing fits the service may instead answer HTTP 500 after
/// about 60 s (sdlc/tickets/0892-transcript-fit-is-quick-and-leads-on.md).
pub const FIT_DEADLINE: Duration = Duration::from_secs(8);
