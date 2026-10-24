//! PubMed native search order and citation metadata, without abstracts.
mod map;
mod efetch;
mod element;
pub use efetch::{fetch_articles, resolve_ids};
mod raw;
use crate::{Mapped, quotas};
use biodata::{ArticleHeader, Pmid, Source, SourceNotice, Text};
use biomcp_fetch::{
    Arg, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, RequestError, Secret, SecretName,
    SourceSpec,
};
use std::time::Duration;
/// Shared NCBI E-utilities policy.
pub static SPEC: SourceSpec = SourceSpec {
    source: Source::PubMed,
    base: "https://eutils.ncbi.nlm.nih.gov/entrez/eutils",
    key: KeyUse::Optional(Credential {
        secret: SecretName::NcbiApiKey,
        place: KeyPlace::Query("api_key"),
    }),
    quota: &quotas::NCBI,
    deadline: Duration::from_secs(15),
};
const SEARCH: Endpoint = SPEC.get("/esearch.fcgi").fresh(Freshness::Days(1));
const SUMMARY: Endpoint = SPEC
    .post_form("/esummary.fcgi")
    .idempotent()
    .fresh(Freshness::Days(1));
/// PubMed's original full requested window and query interpretation.
#[derive(Debug)]
pub struct SearchIds {
    /// Ordered unique checked identities.
    pub ids: Vec<Pmid>,
    /// All matching PubMed records.
    pub count: u64,
    /// PubMed words it could not find.
    pub notices: Vec<SourceNotice>,
}
/// Retrieve the application's fixed requested window in native relevance order.
pub fn search_ids(
    leg: &Leg<'_>,
    term: &str,
    email: Option<&Secret>,
) -> Result<SearchIds, RequestError> {
    let mut params = vec![
        ("db", Arg::Text("pubmed")),
        ("term", Arg::Text(term)),
        ("sort", Arg::Text("relevance")),
        ("retmax", Arg::Num(9999)),
        ("retmode", Arg::Text("json")),
        ("tool", Arg::Text("biomcp")),
    ];
    if let Some(email) = email {
        params.push(("email", Arg::Contact(email)));
    }
    let raw = leg.json::<raw::Search>(&SEARCH, &params)?.esearchresult;
    let notices = raw
        .errorlist
        .and_then(|e| e.phrasesnotfound)
        .map(|words| SourceNotice::NotFoundWords {
            words: words
                .into_iter()
                .filter_map(|s| Text::new(&s).ok())
                .collect(),
        })
        .into_iter()
        .collect();
    Ok(SearchIds {
        ids: raw.idlist,
        count: raw.count,
        notices,
    })
}
/// Retrieve a selected page's metadata. Each result is joined by checked PMID.
pub fn summaries(
    leg: &Leg<'_>,
    ids: &[Pmid],
    email: Option<&Secret>,
) -> Result<Mapped<Vec<ArticleHeader>>, RequestError> {
    let ids: Vec<_> = ids
        .iter()
        .map(|id| id.to_string().into_boxed_str())
        .collect();
    let mut params = vec![
        ("db", Arg::Text("pubmed")),
        ("id", Arg::Ids(&ids)),
        ("retmode", Arg::Text("json")),
        ("tool", Arg::Text("biomcp")),
    ];
    if let Some(email) = email {
        params.push(("email", Arg::Contact(email)));
    }
    let reply = leg.json::<raw::Summary>(&SUMMARY, &params)?;
    Ok(map::summaries(reply, &ids))
}
