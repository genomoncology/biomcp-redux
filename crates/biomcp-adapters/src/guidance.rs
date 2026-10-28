//! Reviewed source timing guidance shared by replies and command help.
use biomcp_types::{Reason, Source, SourceNotice};
/// Explain a source deadline without inferring an outage or a provider completion guarantee.
pub fn deadline(source: Source) -> Option<&'static str> {
    Some(match source {
        Source::Gwas => {
            "BioMCP's local request deadline expired. GWAS Catalog response times can vary; retry later or use \
            https://www.ebi.ac.uk/gwas/. Contact the GWAS Catalog helpdesk if this persists."
        }
        Source::InterPro => {
            "BioMCP's local request deadline expired. The health probe uses a shorter local cap than protein requests. \
            InterPro documents that some queries take minutes; this timeout does not establish an outage. Please \
            retry later or check https://www.ebi.ac.uk/interpro/. See https://interpro-documentation.readthedocs.io/en/latest/faq.html."
        }
        Source::Vaers => {
            "CDC WONDER can take longer than BioMCP's local command deadline. Use the interactive form at \
            https://wonder.cdc.gov/vaers.html for a longer query. See https://wonder.cdc.gov/wonder/help/faq.html."
        }
        _ => return None,
    })
}
/// Explain immediate local refusal under CDC's automated-query spacing guidance.
pub const WONDER_QUOTA: &str = "BioMCP's local quota refused this request before contacting CDC WONDER. Retry after the reported seconds \
            left. Automated queries must follow the spacing at https://wonder.cdc.gov/wonder/help/wonder-api.html.";
/// Preserve a latency or admission reason and attach the corresponding next step.
pub fn notices(
    source: Source,
    reason: Option<Reason>,
    retry: Option<std::time::Duration>,
) -> Vec<SourceNotice> {
    let message = match reason {
        Some(Reason::DeadlineExceeded) => deadline(source),
        Some(Reason::LocalQuota) if source == Source::Vaers => Some(WONDER_QUOTA),
        _ => None,
    };
    message
        .map(|message| SourceNotice::RequestGuidance {
            message: message.into(),
            retry_after_seconds: retry.map(|d| d.as_secs() + u64::from(d.subsec_nanos() != 0)),
        })
        .into_iter()
        .collect()
}
