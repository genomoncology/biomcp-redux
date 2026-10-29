//! ClinVar VCV XML by a checked Variation ID, without text-search identity.
mod detail;
mod map;
mod raw;
mod search;
use crate::quotas;
use biomcp_fetch::{
    Arg, BadReply, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, RequestError,
    SecretName, SourceSpec,
};
use biomcp_types::{Source, VariantAssertion};
pub use search::{SearchPage, search};
use std::time::Duration;
/// ClinVar shares the existing NCBI quota and optional user key.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "NCBI policy; submission rights remain source-specific",
        url: "https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/",
        usage: biomcp_types::Use::Restricted,
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
pub fn assertions(
    leg: &Leg<'_>,
    id: &str,
    selection: Option<(&str, Option<&str>)>,
) -> Result<crate::Mapped<Vec<VariantAssertion>>, RequestError> {
    let vcv = selection.filter(|(id, _)| id.starts_with("VCV"));
    let selected = vcv.map(|(record, version)| {
        version.map_or_else(|| record.to_owned(), |v| format!("{record}.{v}"))
    });
    let mut params = vec![
        ("db", Arg::Text("clinvar")),
        ("id", Arg::Text(selected.as_deref().unwrap_or(id))),
        ("rettype", Arg::Text("vcv")),
    ];
    if vcv.is_none() {
        params.push(("is_variationid", Arg::Text("true")));
    }
    params.extend([("retmode", Arg::Text("xml")), ("tool", Arg::Text("biomcp"))]);
    let (reply, _) = leg.xml_received_checked::<raw::Records>(&FETCH, &params, |r| {
        if r.id.as_deref().is_none_or(|v| v == id)
            && vcv.is_none_or(|(record, version)| {
                r.id.is_none()
                    || r.accession.as_deref() == Some(record)
                        && version.is_none_or(|v| r.version.as_deref() == Some(v))
            })
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    })?;
    Ok(crate::Mapped {
        value: reply.rows,
        problems: reply.problems,
    })
}
