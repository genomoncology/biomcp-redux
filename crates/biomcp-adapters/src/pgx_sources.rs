//! Shared reviewed policy for both pharmacogenomics publishers.
use biomcp_fetch::Terms;
use biomcp_types::Use;
// The captured policy page was a shell, so this names the PM's reviewed product ruling.
pub(crate) const TERMS: Terms = Terms {
    name: "ClinPGx data use policy; noncommercial by PM ruling; captured policy was a JavaScript shell",
    url: "https://www.clinpgx.org/page/dataUsagePolicy",
    usage: Use::Noncommercial,
};
