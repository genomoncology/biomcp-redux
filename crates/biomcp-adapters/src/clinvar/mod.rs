//! ClinVar VCV XML by a checked Variation ID, without text-search identity.
mod detail;
mod map;
mod raw;
use crate::quotas;
use biodata::{Source, VariantAssertion};
use biomcp_fetch::{
    Arg, BadReply, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, RequestError,
    SecretName, SourceSpec,
};
use std::time::Duration;
/// ClinVar shares the existing NCBI quota and optional user key.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "NCBI policy; submission rights remain source-specific",
        url: "https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/",
        usage: biodata::Use::Restricted,
    },
    source: Source::ClinVar,
    base: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils",
    key: KeyUse::Optional(Credential {
        secret: SecretName::NcbiApiKey,
        place: KeyPlace::Query("api_key"),
    }),
    quota: &quotas::NCBI,
    deadline: Duration::from_secs(15),
};
const FETCH: Endpoint = SPEC.get("/efetch.fcgi").fresh(Freshness::Days(1));
/// Fetch exactly one Variation ID and validate the XML identity before cache admission.
pub fn assertions(leg: &Leg<'_>, id: &str) -> Result<Vec<VariantAssertion>, RequestError> {
    let params = [
        ("db", Arg::Text("clinvar")),
        ("id", Arg::Text(id)),
        ("rettype", Arg::Text("vcv")),
        ("is_variationid", Arg::Text("true")),
        ("retmode", Arg::Text("xml")),
        ("tool", Arg::Text("biomcp")),
    ];
    let (reply, _) = leg.xml_received_checked::<raw::Records>(&FETCH, &params, |r| {
        if r.id.as_deref().is_none_or(|v| v == id) {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    Ok(reply.rows)
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::ClinVar,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
