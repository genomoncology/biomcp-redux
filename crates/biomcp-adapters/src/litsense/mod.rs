//! LitSense2 sentence resemblance in the provider's original order.
use crate::{
    Mapped,
    mapped::{Problems, record},
    quotas,
    raw::Field,
};
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
pub fn search(leg: &Leg<'_>, term: &str) -> Result<Mapped<Vec<ArticleItem>>, RequestError> {
    let reply = leg.json::<Sentences>(
        &SENTENCES,
        &[("query", Arg::Text(term)), ("rerank", Arg::Text("true"))],
    )?;
    let mut items: Vec<ArticleItem> = Vec::new();
    let mut problems = Problems {
        source: Source::LitSense,
        record: None,
        list: Vec::new(),
    };
    for (index, sentence) in reply.0.into_iter().enumerate() {
        if items.iter().any(|item| item.pmid == Some(sentence.pmid.0)) {
            continue;
        }
        problems.record = record(sentence.pmid.0);
        let section = problems.field("section", sentence.section);
        let text = problems.source_field("text", sentence.text);
        items.push(ArticleItem {
            key: Some(ArticleKey::Pubmed(sentence.pmid.0)),
            pmid: Some(sentence.pmid.0),
            found_by: [(ArticleSource::Litsense, (index + 1) as u32)].into(),
            cited_by: None,
            intents: None,
            influential: None,
            contexts: None,
            header: None,
            sentence: Some(ArticleSentence { section, text }),
        });
    }
    Ok(Mapped {
        value: items,
        problems: problems.list,
    })
}
#[derive(Deserialize)]
struct Sentences(Vec<Sentence>);
#[derive(Deserialize)]
struct Sentence {
    pmid: crate::raw::NumericPmid,
    section: Option<Field<Text>>,
    text: Option<Field<crate::native_text::NativeText>>,
}
impl Reply for Sentences {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.len() > 100 {
            return Err(BadReply);
        }
        Ok(())
    }
}
