//! LitSense2 sentence resemblance in the provider's original order.
use crate::quotas;
use biodata::Text;
use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, KeyUse, Leg, Reply, RequestError, SourceSpec,
};
use biomcp_types::{ArticleItem, ArticleKey, ArticleSentence, ArticleSource, Source};
use serde::Deserialize;
use std::time::Duration;
/// Public access does not grant rights in the sentences returned from publishers.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "NCBI policies; publisher rights in input text",
        url: "https://www.ncbi.nlm.nih.gov/home/about/policies/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::LitSense,
    base: "https://www.ncbi.nlm.nih.gov/research/litsense2-api",
    key: KeyUse::None,
    quota: &quotas::LITSENSE,
    deadline: Duration::from_secs(15),
};
pub(crate) const SENTENCES: Endpoint = SPEC.get("/api/sentences/").fresh(Freshness::Days(1));
/// Read one bounded provider reply and keep each paper's first sentence, without score sorting.
pub fn search(leg: &Leg<'_>, term: &str) -> Result<Vec<ArticleItem>, RequestError> {
    let reply = leg.json::<Sentences>(
        &SENTENCES,
        &[("query", Arg::Text(term)), ("rerank", Arg::Text("true"))],
    )?;
    let mut items: Vec<ArticleItem> = Vec::new();
    for (index, sentence) in reply.0.into_iter().enumerate() {
        if items.iter().any(|item| item.pmid == Some(sentence.pmid.0)) {
            continue;
        }
        items.push(ArticleItem {
            key: Some(ArticleKey::Pubmed(sentence.pmid.0)),
            pmid: Some(sentence.pmid.0),
            found_by: [(ArticleSource::Litsense, (index + 1) as u32)].into(),
            cited_by: None,
            intents: None,
            influential: None,
            contexts: None,
            header: None,
            sentence: Some(ArticleSentence {
                section: sentence.section.and_then(crate::raw::Field::value),
                text: sentence.text,
            }),
        });
    }
    Ok(items)
}
#[derive(Deserialize)]
struct Sentences(Vec<Sentence>);
#[derive(Deserialize)]
struct Sentence {
    pmid: crate::raw::NumericPmid,
    #[serde(default)]
    section: Option<crate::raw::Field<Text>>,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    text: biodata::Text,
}
impl Reply for Sentences {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.len() > 100 {
            return Err(BadReply);
        }
        Ok(())
    }
}
