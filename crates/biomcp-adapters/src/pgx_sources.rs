//! Shared reviewed policy for both pharmacogenomics publishers.
use biomcp_fetch::Terms;
use biomcp_types::Use;
// The captured policy page was a shell, so this names the reviewed policy.
pub(crate) const TERMS: Terms = Terms {
    name: "ClinPGx data use policy; noncommercial by reviewed policy; captured policy was a JavaScript shell",
    url: "https://www.clinpgx.org/page/dataUsagePolicy",
    usage: Use::Noncommercial,
};
