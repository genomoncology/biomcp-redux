use super::{SPEC, efetch_book, map::publication_type};
use crate::mapped::{Problems, record};
use crate::xml_element::{Element, body_nodes};
use biodata::{PartialDate, Pmid, Text};
use biomcp_fetch::{Input, Read};
use biomcp_types::{Problem, Source};

use biomcp_fetch::{
    Arg, BadReply, Endpoint, Freshness, Leg, Reply, ReplyMetadata, RequestError, Secret,
    xml::XmlReply,
};
use biomcp_types::{AbstractPart, Article, Correction, CorrectionKind, CorrectionRelation};
use serde::Deserialize;
/// One citation and its separately requested indexing facts.
#[derive(Debug, Deserialize)]
pub struct FetchedArticle {
    /// Actual refused native indexing records.
    #[serde(with = "crate::mapped::refusal_serde")]
    pub refusals: crate::Refusal,
    /// Full article for the default card.
    pub article: Article,
    /// Facts from that same citation.
    pub indexing: biomcp_types::Indexing,
    /// Rejected optional fields from this exact citation.
    pub problems: Vec<Problem>,
}
/// Decoded PubMed citations from one EFetch response.
#[derive(Debug)]
pub struct FetchedArticles {
    /// Usable citations in source order.
    pub articles: Vec<FetchedArticle>,
    /// Provenance for the whole EFetch response.
    pub metadata: ReplyMetadata,
    /// A book citation could not be bound to a valid PMID.
    pub unbound_book: bool,
}
const FETCH: Endpoint = SPEC
    .post_form("/efetch.fcgi")
    .idempotent()
    .fresh(Freshness::Days(1));
/// Fetch a checked batch of at most two hundred PMID records and its provenance.
pub fn fetch_articles(
    leg: &Leg<'_>,
    ids: &[Pmid],
    email: Option<&Secret>,
) -> Result<FetchedArticles, RequestError> {
    if ids.len() > 200 || ids.is_empty() {
        return Err(biomcp_fetch::InputError::Method.into());
    }
    let licenses = crate::article_rights::pubmed::lookup(leg, ids);
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
    let (reply, metadata) = leg
        .call(
            &FETCH,
            Input::Params(&params),
            Read::<Records>::xml()
                .cache_if(&|reply| !reply.unbound_book)
                .filter_body(&|body| crate::article_rights::pubmed::filter(leg, body, &licenses)),
        )
        .map(|received| (received.value, received.meta))?;
    Ok(FetchedArticles {
        articles: reply.articles,
        metadata,
        unbound_book: reply.unbound_book,
    })
}
#[derive(Deserialize)]
struct Records {
    articles: Vec<FetchedArticle>,
    unbound_book: bool,
}
impl Reply for Records {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
impl XmlReply for Records {
    fn decode_xml(xml: &str) -> Result<Self, BadReply> {
        let root = Element::root(xml, "PubmedArticleSet")?;
        if root.child("ERROR")?.is_some() {
            return Err(BadReply);
        }
        let mut articles = Vec::new();
        let mut unbound_book = false;
        for (kind, node) in body_nodes(root.0)? {
            match kind.as_str() {
                "PubmedArticle" => articles.push(article(node)?),
                "PubmedBookArticle" => match efetch_book::chapter(node)? {
                    Some(chapter) => articles.push(chapter),
                    None => unbound_book = true,
                },
                _ => (),
            }
        }
        Ok(Self {
            articles,
            unbound_book,
        })
    }
}
fn article(root: Element<'_>) -> Result<FetchedArticle, BadReply> {
    let citation = root.child("MedlineCitation")?.ok_or(BadReply)?;
    let row = citation.child("Article")?.ok_or(BadReply)?;
    let pmid: Pmid = citation
        .value(&["PMID"])?
        .ok_or(BadReply)?
        .parse()
        .map_err(|_| BadReply)?;
    let mut problems = Problems {
        source: Source::PubMed,
        record: record(pmid),
        list: Vec::new(),
    };
    let title = problems.text("title", row.value(&["ArticleTitle"])?.as_deref());
    let journal = problems.text("journal", row.value(&["Journal", "Title"])?.as_deref());
    let date = pubdate(
        row.at(&["Journal", "JournalIssue", "PubDate"])?,
        "date",
        &mut problems,
    )?;
    let mut doi = None;
    let mut pmcid = None;
    if let Some(ids) = root.at(&["PubmedData", "ArticleIdList"])? {
        for id in ids.children("ArticleId")? {
            match id.attr("IdType")?.as_deref() {
                Some("doi") => doi = problems.optional("doi", Some(id.text()?.parse())),
                Some("pmc") => pmcid = problems.optional("pmcid", Some(id.text()?.parse())),
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
    let mut refusals = crate::Refusal::None;
    let indexing = super::indexing::parse(citation, row, &mut problems, &mut refusals)?;
    Ok(FetchedArticle {
        refusals,
        indexing,
        article: Article {
            book_title: None,
            bookshelf_url: None,
            text_withheld: crate::article_rights::pubmed::notice(&root, pmid)?,
            europepmc_id: None,
            pmid: Some(pmid),
            doi,
            pmcid,
            title,
            journal,
            date,
            electronic_date: electronic_date(row, date, &mut problems)?,
            types: types.into(),
            corrections: corrections(citation, &mut problems)?.into(),
            authors: authors(row, &mut problems)?.into(),
            abstract_parts: abstracts(row, &mut problems)?.into(),
        },
        problems: problems.list,
    })
}
fn electronic_date(
    row: Element<'_>,
    print: Option<PartialDate>,
    problems: &mut Problems,
) -> Result<Option<PartialDate>, BadReply> {
    for date in row.children("ArticleDate")? {
        if date.attr("DateType")?.as_deref() == Some("Electronic") {
            return Ok(pubdate(Some(date), "electronic_date", problems)?
                .filter(|electronic| Some(*electronic) != print));
        }
    }
    Ok(None)
}

pub(super) fn authors(row: Element<'_>, problems: &mut Problems) -> Result<Vec<Text>, BadReply> {
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
            if let Some(name) = problems.text("authors", Some(&name)) {
                out.push(name);
            }
        }
    }
    Ok(out)
}
fn abstracts(row: Element<'_>, problems: &mut Problems) -> Result<Vec<AbstractPart>, BadReply> {
    let mut out = Vec::new();
    if let Some(body) = row.child("Abstract")? {
        for part in body.children("AbstractText")? {
            let value = part.text()?;
            if problems.text("abstract_parts.text", Some(&value)).is_some() {
                out.push(AbstractPart {
                    label: problems.text("abstract_parts.label", part.attr("Label")?.as_deref()),
                    text: crate::jats::markdown(part.0)?,
                });
            }
        }
    }
    Ok(out)
}
fn corrections(
    citation: Element<'_>,
    problems: &mut Problems,
) -> Result<Vec<Correction>, BadReply> {
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
                pmid: problems.optional(
                    "corrections.pmid",
                    item.value(&["PMID"])?.map(|s| s.parse()),
                ),
            });
        }
    }
    Ok(out)
}
pub(super) fn pubdate(
    node: Option<Element<'_>>,
    field: &str,
    problems: &mut Problems,
) -> Result<Option<PartialDate>, BadReply> {
    let Some(node) = node else { return Ok(None) };
    // Do not infer date components from native MedlineDate prose.
    let Some(year) = node.value(&["Year"])? else {
        return Ok(problems.required(field, None));
    };
    let month = node.value(&["Month"])?;
    let day = node.value(&["Day"])?;
    let value = (|| {
        let year = year.parse().map_err(|_| BadReply)?;
        let month = month.as_deref().map(super::map::month).transpose()?;
        let day = day
            .as_deref()
            .map(str::parse)
            .transpose()
            .map_err(|_| BadReply)?;
        PartialDate::new(year, month, day).map_err(|_| BadReply)
    })();
    Ok(problems.optional(field, Some(value)))
}
