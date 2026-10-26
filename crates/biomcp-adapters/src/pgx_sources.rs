//! Shared reviewed policy for both pharmacogenomics publishers.
use biomcp_fetch::Terms;
use biomcp_types::Use;
// The captured policy page was a shell; keep its access restriction explicit.
pub(crate) const TERMS: Terms = Terms {
    name: "ClinPGx data use policy; noncommercial access restriction; captured policy was a JavaScript shell",
    url: "https://www.clinpgx.org/page/dataUsagePolicy",
    usage: Use::Noncommercial,
};
