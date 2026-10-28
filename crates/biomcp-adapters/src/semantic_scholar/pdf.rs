//! The paper record supplies a link, never credentials for the publisher.

use biodata::License;
use biomcp_fetch::{Arg, BadReply, Leg, Reply, RequestError};
use biomcp_types::ArticleInput;
use biomcp_types::OpenAccessStatus;
use serde::Deserialize;
/// Source PDF-link fields with absence preserved.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PdfRecord {
    /// The source explicitly reports a missing link with null.
    #[serde(deserialize_with = "link_field")]
    pub open_access_pdf: Option<PdfLink>,
}
/// Publisher URL and source-provided access metadata.
#[derive(Debug, Deserialize)]
pub struct PdfLink {
    /// Publisher URL; the transport validates it before use.
    pub url: String,
    /// Open access status, normalized from the provider's uppercase spelling.
    #[serde(deserialize_with = "access")]
    pub status: OpenAccessStatus,
    /// Provider license label, which may be absent.
    pub license: Option<String>,
}
impl Reply for PdfRecord {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
fn link_field<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Option<PdfLink>, D::Error> {
    Option::<PdfLink>::deserialize(decoder)
}
fn access<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<OpenAccessStatus, D::Error> {
    String::deserialize(decoder)?
        .to_lowercase()
        .parse()
        .map_err(serde::de::Error::custom)
}
impl PdfLink {
    /// Preserve absent and unknown license labels without asserting reuse rights.
    pub fn license(&self) -> License {
        match self
            .license
            .as_deref()
            .unwrap_or("")
            .to_ascii_uppercase()
            .as_str()
        {
            "CCBY" | "CC-BY" => License::CcBy,
            "CCBYNC" | "CC-BY-NC" => License::CcByNc,
            "CCBYSA" | "CC-BY-SA" => License::CcBySa,
            "CCBYNCSA" | "CC-BY-NC-SA" => License::CcByNcSa,
            "CCBYND" | "CC-BY-ND" => License::CcByNd,
            "CCBYNCND" | "CC-BY-NC-ND" => License::CcByNcNd,
            "CC0" => License::Cc0,
            _ => License::Other,
        }
    }
}
/// Read only the open-access PDF fields using the shared paper path and API quota.
pub fn pdf(leg: &Leg<'_>, input: &ArticleInput) -> Result<PdfRecord, RequestError> {
    leg.json(
        &super::paper_endpoint(input)?,
        &[("fields", Arg::Text("openAccessPdf"))],
    )
}

/// Credential-free publisher declaration, distinct from the optional-key API.
pub static PUBLISHER: biomcp_fetch::SourceSpec = biomcp_fetch::SourceSpec {
    destinations: biomcp_fetch::Destinations::PublisherPdf,
    user_agent: None,
    source: biomcp_types::Source::SemanticScholar,
    terms: biomcp_fetch::Terms {
        name: "Publisher PDF; article-specific source license",
        url: "https://api.semanticscholar.org/api-docs/graph#tag/Paper-Data",
        usage: biomcp_types::Use::Restricted,
    },
    base: "",
    key: biomcp_fetch::KeyUse::None,
    quota: &crate::quotas::PUBLISHER_PDF,
    deadline: std::time::Duration::from_secs(15),
};
