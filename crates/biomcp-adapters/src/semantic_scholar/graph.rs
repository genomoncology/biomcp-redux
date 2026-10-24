//! Bounded paper sections and native citation graph windows.
use biodata::{ArticleInput, Capped, Text, Tldr};
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, PathSegment, Reply, RequestError};
use serde::Deserialize;
const PAPER: Endpoint = super::SPEC.get("/paper").fresh(Freshness::Days(1));
/// Independent machine-written summary and source counts.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaperSummary {
    /// Null means the source has no summary.
    pub tldr: Option<Summary>,
    /// Independently reported citation count.
    pub citation_count: Option<u64>,
    /// Independently reported influential citation count.
    pub influential_citation_count: Option<u64>,
}
/// Source summary and model identifier.
#[derive(Debug, Deserialize)]
pub struct Summary {
    /// Complete source summary.
    pub text: Text,
    /// Source model version.
    pub model: Text,
}
impl Reply for PaperSummary {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl PaperSummary {
    /// Preserve missing counts independently from an available summary.
    pub fn section(self) -> biodata::Section<Tldr> {
        self.tldr.map_or(
            biodata::Section::Empty {
                source: biodata::Source::SemanticScholar,
                record: None,
            },
            |summary| biodata::Section::Data {
                source: biodata::Source::SemanticScholar,
                data: Tldr {
                    text: summary.text,
                    model: summary.model,
                    machine_written: true,
                    cited_by: self.citation_count,
                    influential_cited_by: self.influential_citation_count,
                },
            },
        )
    }
}
fn paper(input: &ArticleInput) -> Result<Endpoint, RequestError> {
    let value = match input {
        ArticleInput::Pmid(id) => format!("PMID:{id}"),
        ArticleInput::Pmcid(id) => format!("PMCID:{id}"),
        ArticleInput::Doi(id) => format!("DOI:{id}"),
    };
    let part = PathSegment::new(&value).map_err(RequestError::Input)?;
    Ok(PAPER.clone().with_segments(&[part]))
}
/// Read the summary under the shared optional-key source declaration.
pub fn summary(leg: &Leg<'_>, input: &ArticleInput) -> Result<PaperSummary, RequestError> {
    leg.json(
        &paper(input)?,
        &[(
            "fields",
            Arg::Text("tldr,citationCount,influentialCitationCount"),
        )],
    )
}
/// One source graph page; `next` is a provider offset, never a client guess.
#[derive(Debug, Deserialize)]
pub struct Edges {
    /// Provider continuation offset.
    pub next: Option<u32>,
    /// Native citation order.
    pub data: Vec<Edge>,
}
/// Source edge metadata with missing fields preserved independently.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Edge {
    /// Present in a citation reply.
    pub citing_paper: Option<super::Paper>,
    /// Present in a references reply.
    pub cited_paper: Option<super::Paper>,
    /// Provider intent labels.
    pub intents: Option<Box<[biodata::CitationIntent]>>,
    /// Missing is distinct from false.
    pub is_influential: Option<bool>,
    /// Missing is distinct from a reported empty list.
    pub contexts: Option<Vec<Text>>,
}
impl Reply for Edges {
    fn check(&self) -> Result<(), BadReply> {
        if self.data.len() > 1000
            || self
                .data
                .iter()
                .any(|e| e.citing_paper.is_some() == e.cited_paper.is_some())
        {
            return Err(BadReply);
        }
        Ok(())
    }
}
impl Edge {
    /// Keep the selected direction's paper and first whole context.
    pub fn item(self, references: bool) -> Result<biodata::ArticleEdgeItem, BadReply> {
        let mut item = if references {
            self.cited_paper
        } else {
            self.citing_paper
        }
        .ok_or(BadReply)?
        .item();
        item.intents = self.intents;
        item.influential = self.is_influential;
        item.contexts = self.contexts.map(|contexts| Capped {
            total: contexts.len() as u64,
            items: contexts.into_iter().take(1).collect(),
        });
        Ok(item)
    }
}
/// Read at most one thousand edges without sorting the provider's rows.
pub fn edges(
    leg: &Leg<'_>,
    input: &ArticleInput,
    references: bool,
    offset: Option<u32>,
) -> Result<Edges, RequestError> {
    let suffix = PathSegment::new(if references {
        "references"
    } else {
        "citations"
    })
    .map_err(RequestError::Input)?;
    let endpoint = paper(input)?.with_segments(&[suffix]);
    let mut args = vec![
        ("limit", Arg::Num(1000)),
        (
            "fields",
            Arg::Text("contexts,intents,isInfluential,externalIds,title,year"),
        ),
    ];
    if let Some(offset) = offset {
        args.push(("offset", Arg::Num(u64::from(offset))));
    }
    leg.json(&endpoint, &args)
}
