use super::{SPEC, map::publication_type};
use crate::xml_element::Element;
use biodata::{PartialDate, Pmid, Text};

use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, Leg, Reply, ReplyMetadata, RequestError, Secret,
    xml::XmlReply,
};
use biomcp_types::{
    AbstractPart, Article, ArticleInput, Correction, CorrectionKind, CorrectionRelation,
};
use serde::Deserialize;
/// One citation and its separately requested indexing facts.
#[derive(Debug, Deserialize)]
pub struct FetchedArticle {
    /// Full article for the default card.
    pub article: Article,
    /// Facts from that same citation.
    pub indexing: biomcp_types::Indexing,
}
const FETCH: Endpoint = SPEC
    .post_form("/efetch.fcgi")
    .idempotent()
    .fresh(Freshness::Days(1));
const RESOLVE: Endpoint = SPEC.get("/esearch.fcgi").fresh(Freshness::Days(1));
/// Resolve one namespace and retain distinct source matches.
pub fn resolve_ids(
    leg: &Leg<'_>,
    input: &ArticleInput,
    email: Option<&Secret>,
) -> Result<Vec<Pmid>, RequestError> {
    let term = match input {
        ArticleInput::EuropePmc(_) => return Ok(Vec::new()),
        ArticleInput::Pmid(id) => return Ok(vec![*id]),
        ArticleInput::Pmcid(id) => format!("{id}[pmc]"),
        ArticleInput::Doi(id) => format!("{id}[doi]"),
    };
    let mut params = vec![
        ("db", Arg::Text("pubmed")),
        ("term", Arg::Text(&term)),
        ("retmax", Arg::Num(9999)),
        ("retmode", Arg::Text("json")),
        ("tool", Arg::Text("biomcp")),
    ];
    if let Some(email) = email {
        params.push(("email", Arg::Contact(email)));
    }
    Ok(leg
        .json::<super::raw::Search>(&RESOLVE, &params)?
        .esearchresult
        .idlist)
}
/// Fetch a checked batch of at most two hundred PMID records and its provenance.
pub fn fetch_articles(
    leg: &Leg<'_>,
    ids: &[Pmid],
    email: Option<&Secret>,
) -> Result<(Vec<FetchedArticle>, ReplyMetadata), RequestError> {
    if ids.len() > 200 || ids.is_empty() {
        return Err(biomcp_fetch::InputError::Method.into());
    }
    let ids: Vec<_> = ids
        .iter()
        .map(|id| id.to_string().into_boxed_str())
        .collect();
    let mut params = vec![
        ("db", Arg::Text("pubmed")),
        ("id", Arg::Ids(&ids)),
        ("retmode", Arg::Text("xml")),
        ("tool", Arg::Text("biomcp")),
    ];
    if let Some(email) = email {
        params.push(("email", Arg::Contact(email)));
    }
    let (reply, metadata) = leg.xml_received::<Records>(&FETCH, &params)?;
    Ok((reply.articles, metadata))
}
#[derive(Deserialize)]
struct Records {
    articles: Vec<FetchedArticle>,
}
impl Reply for Records {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl XmlReply for Records {
    fn decode_xml(xml: &str) -> Result<Self, BadReply> {
        let mut reader = quick_xml::Reader::from_str(xml);
        let start = loop {
            let begin = reader.buffer_position() as usize;
            match reader.read_event().map_err(|_| BadReply)? {
                quick_xml::events::Event::Start(tag)
                    if tag.name().as_ref() == "PubmedArticleSet" =>
                {
                    break begin;
                }
                quick_xml::events::Event::Empty(tag)
                    if tag.name().as_ref() == "PubmedArticleSet" =>
                {
                    return Ok(Self {
                        articles: Vec::new(),
                    });
                }
                quick_xml::events::Event::Eof | quick_xml::events::Event::Start(_) => {
                    return Err(BadReply);
                }
                _ => {}
            }
        };
        if Element(&xml[start..]).child("ERROR")?.is_some() {
            return Err(BadReply);
        }
        let articles = Element(&xml[start..])
            .children("PubmedArticle")?
            .into_iter()
            .map(article)
            .collect::<Result<_, _>>()?;
        Ok(Self { articles })
    }
}
fn article(root: Element<'_>) -> Result<FetchedArticle, BadReply> {
    let citation = root.child("MedlineCitation")?.ok_or(BadReply)?;
    let row = citation.child("Article")?.ok_or(BadReply)?;
    let pmid = citation
        .value(&["PMID"])?
        .ok_or(BadReply)?
        .parse()
        .map_err(|_| BadReply)?;
    let title = row
        .value(&["ArticleTitle"])?
        .map(|s| crate::mapped::text(&s))
        .transpose()?
        .ok_or(BadReply)?;
    let journal = row
        .value(&["Journal", "Title"])?
        .map(|s| crate::mapped::text(&s))
        .transpose()?;
    let date = pubdate(row.at(&["Journal", "JournalIssue", "PubDate"])?)?;
    let mut doi = None;
    let mut pmcid = None;
    if let Some(ids) = root.at(&["PubmedData", "ArticleIdList"])? {
        for id in ids.children("ArticleId")? {
            match id.attr("IdType")?.as_deref() {
                Some("doi") => doi = Some(id.text()?.parse().map_err(|_| BadReply)?),
                Some("pmc") => pmcid = Some(id.text()?.parse().map_err(|_| BadReply)?),
                _ => {}
            }
        }
    }
    let mut types = Vec::new();
    if let Some(list) = row.child("PublicationTypeList")? {
        for item in list.children("PublicationType")? {
            if let Some(kind) = publication_type(&item.text()?)
                && !types.contains(&kind)
            {
                types.push(kind);
            }
        }
    }
    Ok(FetchedArticle {
        indexing: super::indexing::parse(citation, row)?,
        article: Article {
            europepmc_id: None,
            pmid: Some(pmid),
            doi,
            pmcid,
            title,
            journal,
            date,
            electronic_date: electronic_date(row, date)?,
            types: types.into(),
            corrections: corrections(citation)?.into(),
            authors: authors(row)?.into(),
            abstract_parts: abstracts(row)?.into(),
        },
    })
}
fn electronic_date(
    row: Element<'_>,
    print: Option<PartialDate>,
) -> Result<Option<PartialDate>, BadReply> {
    for date in row.children("ArticleDate")? {
        if date.attr("DateType")?.as_deref() == Some("Electronic") {
            return Ok(pubdate(Some(date))?.filter(|electronic| Some(*electronic) != print));
        }
    }
    Ok(None)
}

fn authors(row: Element<'_>) -> Result<Vec<Text>, BadReply> {
    let mut out = Vec::new();
    if let Some(list) = row.child("AuthorList")? {
        for author in list.children("Author")? {
            let name = if let Some(collective) = author.value(&["CollectiveName"])? {
                collective
            } else {
                [author.value(&["LastName"])?, author.value(&["Initials"])?]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<_>>()
                    .join(" ")
            };
            if !name.is_empty() {
                out.push(Text::new(&name).map_err(|_| BadReply)?);
            }
        }
    }
    Ok(out)
}
fn abstracts(row: Element<'_>) -> Result<Vec<AbstractPart>, BadReply> {
    let mut out = Vec::new();
    if let Some(body) = row.child("Abstract")? {
        for part in body.children("AbstractText")? {
            let value = part.text()?;
            if !value.trim().is_empty() {
                out.push(AbstractPart {
                    label: part
                        .attr("Label")?
                        .map(|s| crate::mapped::text(&s))
                        .transpose()?,
                    text: crate::jats::markdown(part.0)?,
                });
            }
        }
    }
    Ok(out)
}
fn corrections(citation: Element<'_>) -> Result<Vec<Correction>, BadReply> {
    let mut out = Vec::new();
    if let Some(list) = citation.child("CommentsCorrectionsList")? {
        for item in list.children("CommentsCorrections")? {
            let Some(kind) = item.attr("RefType")? else {
                continue;
            };
            let relation = if kind.ends_with("In") {
                CorrectionRelation::Notice
            } else if kind.ends_with("Of") {
                CorrectionRelation::Original
            } else {
                continue;
            };
            let kind = if kind.starts_with("Retraction") {
                CorrectionKind::Retraction
            } else if kind.starts_with("Erratum") {
                CorrectionKind::Erratum
            } else if kind.starts_with("ExpressionOfConcern") {
                CorrectionKind::ExpressionOfConcern
            } else {
                continue;
            };
            out.push(Correction {
                kind,
                relation,
                pmid: item
                    .value(&["PMID"])?
                    .map(|s| s.parse().map_err(|_| BadReply))
                    .transpose()?,
            });
        }
    }
    Ok(out)
}
fn pubdate(node: Option<Element<'_>>) -> Result<Option<PartialDate>, BadReply> {
    let Some(node) = node else { return Ok(None) };
    let Some(year) = node.value(&["Year"])?.and_then(|s| s.parse().ok()) else {
        return Ok(None);
    };
    let month = node.value(&["Month"])?.and_then(|s| {
        s.parse().ok().or_else(|| {
            [
                "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
            ]
            .iter()
            .position(|m| *m == s)
            .and_then(|n| u8::try_from(n + 1).ok())
        })
    });
    let day = if month.is_some() {
        node.value(&["Day"])?.and_then(|s| s.parse().ok())
    } else {
        None
    };
    Ok(PartialDate::new(year, month, day).ok())
}
