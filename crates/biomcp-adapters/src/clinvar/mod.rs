//! ClinVar archives by checked Variation ID or accession.
use biodata::VcvId;
use biomcp_fetch::{Input, Read};
mod detail;
mod gene;
mod gene_reply;
mod gene_review;
pub use gene::gene_counts;
mod map;
mod raw;
mod reference;
pub(crate) use reference::reference_agrees;
mod search;
mod subject;
use crate::quotas;
use biomcp_fetch::{
    Arg, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, RequestError, SecretName,
    SourceSpec,
};
use biomcp_types::{ClinvarIncluded, Source, VariantAssertion};
pub use search::{SearchPage, search};
use std::time::Duration;
/// ClinVar shares the existing NCBI quota and optional user key.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "NCBI policy; submission rights remain source-specific",
        url: "https://www.ncbi.nlm.nih.gov/clinvar/docs/maintenance_use/",
        usage: biomcp_types::Use::Open,
    },
    source: Source::ClinVar,
    base: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils",
    key: KeyUse::Optional(Credential {
        tier: biomcp_types::KeyTier::Fallback,
        secret: SecretName::NcbiApiKey,
        place: KeyPlace::Query("api_key"),
    }),
    quota: &quotas::NCBI,
    deadline: Duration::from_secs(15),
    command_budget: None,
};
/// ClinVar gene counts: about seventeen searches run one at a time on NCBI's shared pacing.
/// Keyless BRCA1 reads took 7.3 to 7.6 s once paced (2026-11-14), and reads that met NCBI
/// throttle replies, 2 s each, ran 14 to 17 s and failed the 15 s other ClinVar reads keep.
/// Like Monarch similarity search, the section gets 30 s and its command 30 s too.
pub static GENE_COUNTS: SourceSpec = SourceSpec {
    destinations: SPEC.destinations,
    user_agent: SPEC.user_agent,
    terms: SPEC.terms,
    source: Source::ClinVar,
    base: SPEC.base,
    key: SPEC.key,
    quota: &quotas::NCBI,
    deadline: Duration::from_secs(30),
    command_budget: Some(Duration::from_secs(30)),
};
const FETCH: Endpoint = SPEC.get("/efetch.fcgi").fresh(Freshness::Days(1));
/// Assertions and included-only relationships from a single ClinVar archive.
#[derive(Debug)]
pub struct Assertions {
    /// Checked accession and version of the archive served.
    pub archive: Option<VcvId>,
    /// Direct assertion rows.
    pub rows: Vec<VariantAssertion>,
    /// The archive's including records when it owns no submissions.
    pub included_only: Option<ClinvarIncluded>,
}
/// Fetch exactly one Variation ID and validate the XML identity before cache admission.
pub fn assertions(
    leg: &Leg<'_>,
    id: &str,
    archive: Option<VcvId>,
    selection: Option<(&biomcp_types::AssertionId, Option<&str>)>,
) -> Result<crate::Mapped<Assertions>, RequestError> {
    let archive = archive
        .filter(|id| id.version().is_some())
        .map(|id| id.to_string());
    let selected_vcv = if let Some((record, version)) =
        archive.as_deref().and_then(|value| value.split_once('.'))
    {
        Some((record.to_owned(), Some(version)))
    } else {
        selection
            .filter(|(id, _)| matches!(id, biomcp_types::AssertionId::Vcv(_)))
            .map(|(id, version)| (id.to_string(), version))
    };
    let vcv = selected_vcv
        .as_ref()
        .map(|(id, version)| (id.as_str(), *version));
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
    let (mut reply, _) = leg
        .call(
            &FETCH,
            Input::Params(&params),
            Read::<raw::Records>::xml()
                .cache_if(&|reply| reply.identity_problem(id, vcv).is_none()),
        )
        .map(|received| (received.value, received.meta))?;
    if let Some(field) = reply.identity_problem(id, vcv) {
        reply.refusals = crate::Refusal::Refused;
        reply.accession = None;
        reply.rows.clear();
        reply.included_only = None;
        if !reply
            .problems
            .iter()
            .any(|problem| problem.field.as_ref() == field)
        {
            reply.problems.push(crate::mapped::problem(
                Source::ClinVar,
                crate::mapped::record(id),
                field,
            ));
        }
    }
    Ok(crate::Mapped {
        refusals: reply.refusals,
        value: Assertions {
            archive: reply.archive(),
            rows: reply.rows,
            included_only: reply.included_only,
        },
        problems: reply.problems,
    })
}

/// Find a condition or submission accession and verify membership in the current archive.
pub fn by_assertion(
    leg: &Leg<'_>,
    accession: &biomcp_types::VariantInput,
) -> Result<Option<crate::Mapped<Assertions>>, RequestError> {
    let input = accession.as_str();
    let (record, version) = input
        .split_once('.')
        .map_or((input, None), |(id, version)| (id, Some(version)));
    let Some(id) = search::variation_id(leg, record)? else {
        return Ok(None);
    };
    let mut mapped = assertions(leg, &id.get().to_string(), None, None)?;
    mapped.value.rows.retain_mut(|row| {
        row.matched = row.record.as_ref().is_some_and(|id| id.as_str() == record)
            && version.is_none_or(|expected| {
                row.attribution()
                    .and_then(|a| a.version.as_ref())
                    .is_some_and(|v| v.as_str() == expected)
            });
        row.matched
    });
    if mapped.value.rows.is_empty() {
        mapped.value.archive = None;
        mapped.value.included_only = None;
    }
    Ok(Some(mapped))
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
