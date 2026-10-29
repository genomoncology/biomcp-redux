//! Bounded paper sections and native citation graph windows.
use biodata::Text;
use biomcp_fetch::{Arg, BadReply, Endpoint, Freshness, Leg, PathSegment, Reply, RequestError};
use biomcp_types::ArticleInput;
use biomcp_types::{Capped, Tldr};
use serde::Deserialize;
const PAPER: Endpoint = super::SPEC.get("/paper").fresh(Freshness::Days(1));
/// Independent machine-written summary and source counts.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PaperSummary {
    /// Null means the source has no summary.
    #[serde(deserialize_with = "summary_field")]
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
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub text: biodata::Text,
    /// Source model version.
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub model: biodata::Text,
}
impl Reply for PaperSummary {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl PaperSummary {
    /// Preserve missing counts independently from an available summary.
    pub fn section(self) -> biomcp_types::Section<Tldr> {
        self.tldr.map_or(
            biomcp_types::Section::Empty {
                source: biomcp_types::Source::SemanticScholar,
                record: None,
            },
            |summary| biomcp_types::Section::Data {
                source: biomcp_types::Source::SemanticScholar,
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
/// Bind a checked article identity to the shared graph source without accepting a free URL.
pub fn paper_endpoint(input: &ArticleInput) -> Result<Endpoint, RequestError> {
    let value = super::paper_id(input)?;
    let part = PathSegment::new(&value).map_err(RequestError::Input)?;
    Ok(PAPER.clone().with_segments(&[part]))
}
/// Read the summary under the shared optional-key source declaration.
pub fn summary(leg: &Leg<'_>, input: &ArticleInput) -> Result<PaperSummary, RequestError> {
    leg.json(
        &paper_endpoint(input)?,
        &[(
            "fields",
            Arg::Text("tldr,citationCount,influentialCitationCount"),
        )],
    )
}
/// One source graph page; `next` is a provider offset, never a client guess.
#[derive(Debug)]
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
    pub intents: Option<Box<[biomcp_types::CitationIntent]>>,
    /// Missing is distinct from false.
    pub is_influential: Option<bool>,
    /// Missing is distinct from a reported empty list.
    pub contexts: Option<Vec<String>>,
}
#[derive(Debug, Deserialize)]
struct RawEdges {
    next: Option<u32>,
    #[serde(deserialize_with = "edge_data")]
    data: Option<Vec<Edge>>,
}
impl Reply for RawEdges {
    fn check(&self) -> Result<(), BadReply> {
        if self.data.as_ref().is_some_and(|data| {
            data.len() > 1000
                || data
                    .iter()
                    .any(|e| e.citing_paper.is_some() == e.cited_paper.is_some())
                || data.iter().any(|edge| {
                    edge.contexts
                        .as_ref()
                        .is_some_and(|contexts| contexts.iter().any(|text| text.trim().is_empty()))
                })
        }) {
            return Err(BadReply);
        }
        Ok(())
    }
}
fn edge_data<'de, D: serde::Deserializer<'de>>(decoder: D) -> Result<Option<Vec<Edge>>, D::Error> {
    Option::<Vec<Edge>>::deserialize(decoder)
}
impl Edge {
    /// Keep the selected direction's paper and first whole context.
    pub fn item(self, references: bool) -> Result<biomcp_types::ArticleEdgeItem, BadReply> {
        let mut item = if references {
            self.cited_paper
        } else {
            self.citing_paper
        }
        .ok_or(BadReply)?
        .item();
        item.intents = self.intents;
        item.influential = self.is_influential;
        item.contexts = self
            .contexts
            .map(|contexts| {
                Ok(Capped {
                    total: contexts.len() as u64,
                    items: contexts
                        .into_iter()
                        .take(1)
                        .map(|text| Text::from_source(&text).map_err(|_| BadReply))
                        .collect::<Result<_, _>>()?,
                })
            })
            .transpose()?;
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
    let endpoint = paper_endpoint(input)?.with_segments(&[suffix]);
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
    let raw = leg.json::<RawEdges>(&endpoint, &args)?;
    let data = raw.data.ok_or(biomcp_fetch::SourceError {
        source: biomcp_types::Source::SemanticScholar,
        reason: biomcp_types::Reason::Unavailable,
        status: Some(200),
        retry_after: None,
    })?;
    Ok(Edges {
        next: raw.next,
        data,
    })
}

fn summary_field<'de, D: serde::Deserializer<'de>>(
    decoder: D,
) -> Result<Option<Summary>, D::Error> {
    Option::<Summary>::deserialize(decoder)
}
