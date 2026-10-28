//! PubMed native search order and citation metadata, without abstracts.
mod efetch;
mod indexing;
mod map;
pub use efetch::{FetchedArticle, fetch_articles, resolve_ids};
pub(crate) use map::publication_type;
mod raw;
use crate::{Mapped, quotas};
use biodata::{Pmid, Text};

use biomcp_fetch::{
    Arg, Credential, Endpoint, Freshness, KeyPlace, KeyUse, Leg, RequestError, Secret, SecretName,
    SourceSpec,
};
use biomcp_types::ArticleHeader;
use biomcp_types::{ArticleSort, Source, SourceNotice};
use std::time::Duration;
/// Shared NCBI E-utilities policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "NCBI policies; publisher rights in abstracts",
        url: "https://www.ncbi.nlm.nih.gov/home/about/policies/",
        usage: biomcp_types::Use::Restricted,
    },
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
// https://www.ncbi.nlm.nih.gov/books/NBK25499/ recommends POST for long ESearch terms.
// Keep conservative GET headroom without reimplementing request percent encoding.
const MAX_GET_TERM_BYTES: usize = 1000;
const SEARCH_POST: Endpoint = SPEC
    .post_form("/esearch.fcgi")
    .idempotent()
    .fresh(Freshness::Days(1));
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
/// Retrieve the application's fixed requested window in the requested native provider order.
pub fn search_ids(
    leg: &Leg<'_>,
    term: &str,
    sort: ArticleSort,
    email: Option<&Secret>,
) -> Result<SearchIds, RequestError> {
    let mut params = vec![
        ("db", Arg::Text("pubmed")),
        ("term", Arg::Text(term)),
        ("retmax", Arg::Num(9999)),
        ("retmode", Arg::Text("json")),
        ("tool", Arg::Text("biomcp")),
    ];
    if sort != ArticleSort::Date {
        params.insert(2, ("sort", Arg::Text("relevance")));
    }
    if let Some(email) = email {
        params.push(("email", Arg::Contact(email)));
    }
    let endpoint = if term.len() > MAX_GET_TERM_BYTES {
        &SEARCH_POST
    } else {
        &SEARCH
    };
    let raw = leg.json::<raw::Search>(endpoint, &params)?.esearchresult;
    let mut notices: Vec<_> = raw
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
    notices.extend(
        raw.warninglist
            .into_iter()
            .flatten()
            .map(|(name, phrases)| SourceNotice::PubmedWarning {
                name,
                phrases: phrases.into(),
            }),
    );
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
