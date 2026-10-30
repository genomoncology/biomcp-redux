//! Recent-pool recommendations under the shared source key and quota.
use super::{Paper, SPEC};
use crate::{mapped::Problems, raw::Field};
use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use biomcp_types::ArticleInput;
use serde::{Deserialize, Serialize};
/// Recommendation API shares credentials, publisher terms and pacing with graph calls.
pub static RECOMMEND_SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    base: "https://api.semanticscholar.org/recommendations/v1",
    source: SPEC.source,
    terms: biomcp_fetch::Terms {
        name: SPEC.terms.name,
        url: SPEC.terms.url,
        usage: SPEC.terms.usage,
    },
    key: SPEC.key,
    quota: SPEC.quota,
    deadline: SPEC.deadline,
};
const GET: Endpoint = RECOMMEND_SPEC
    .get("/papers/forpaper")
    .fresh(Freshness::Days(1));
const POST: Endpoint = RECOMMEND_SPEC
    .post_json("/papers/")
    .idempotent()
    .fresh(Freshness::Days(1));
/// Provider-ordered papers, including a successful empty answer.
#[derive(Debug, Deserialize)]
#[serde(from = "RawRecommendations")]
pub struct Recommendations {
    /// Native recommendation order.
    pub recommended_papers: Vec<Paper>,
    /// Rejected independent paper identities.
    pub problems: Vec<biomcp_types::Problem>,
    received: usize,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawRecommendations {
    recommended_papers: Vec<Field<Paper>>,
}
impl From<RawRecommendations> for Recommendations {
    fn from(raw: RawRecommendations) -> Self {
        let received = raw.recommended_papers.len();
        let mut p = Problems {
            source: SPEC.source,
            record: None,
            list: Vec::new(),
        };
        let recommended_papers = raw
            .recommended_papers
            .into_iter()
            .filter_map(|v| p.required("paper_id", v.value()))
            .collect();
        Self {
            recommended_papers,
            problems: p.list,
            received,
        }
    }
}
impl Reply for Recommendations {
    fn check(&self) -> Result<(), BadReply> {
        if self.received > 100 {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
/// Encode a checked paper namespace once for graph and recommendation endpoints.
pub fn paper_id(input: &ArticleInput) -> Result<String, RequestError> {
    match input {
        ArticleInput::Pmid(id) => Ok(format!("PMID:{id}")),
        ArticleInput::Pmcid(id) => Ok(format!("PMCID:{id}")),
        ArticleInput::Doi(id) => Ok(format!("DOI:{id}")),
        ArticleInput::EuropePmc(_) => Err(biomcp_fetch::InputError::Path.into()),
    }
}
/// Use GET for a single positive seed and POST for the other checked combinations.
pub fn recommendations(
    leg: &Leg<'_>,
    seeds: &[ArticleInput],
    negative: &[ArticleInput],
    limit: u32,
) -> Result<Recommendations, RequestError> {
    let args = [
        ("limit", Arg::Num(u64::from(limit))),
        ("fields", Arg::Text("externalIds,title,year")),
    ];
    if let [seed] = seeds
        && negative.is_empty()
    {
        let id = paper_id(seed)?;
        let segment = PathSegment::new(&id).map_err(RequestError::Input)?;
        return leg.json(&GET.clone().with_segments(&[segment]), &args);
    }
    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Body {
        positive_paper_ids: Vec<String>,
        negative_paper_ids: Vec<String>,
    }
    let body = Body {
        positive_paper_ids: seeds.iter().map(paper_id).collect::<Result<_, _>>()?,
        negative_paper_ids: negative.iter().map(paper_id).collect::<Result<_, _>>()?,
    };
    leg.post_json_with_args(&POST, &args, &body)
}
