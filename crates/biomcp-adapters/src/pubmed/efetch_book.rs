//! PubMed book chapters supply citation metadata separately from journal articles.
use super::efetch::{FetchedArticle, authors, pubdate};
use super::publication_type;
use crate::mapped::{Problems, record};
use crate::xml_element::Element;
use biodata::{Pmid, Text};
use biomcp_fetch::BadReply;
use biomcp_types::{Article, Source};

pub(super) fn chapter(root: Element<'_>) -> Result<Option<FetchedArticle>, BadReply> {
    let Some(document) = root.child("BookDocument")? else {
        return Ok(None);
    };
    let Some(id) = document.value(&["PMID"])? else {
        return Ok(None);
    };
    let Ok(pmid) = id.parse::<Pmid>() else {
        return Ok(None);
    };
    let mut problems = Problems {
        source: Source::PubMed,
        record: record(pmid),
        list: Vec::new(),
    };
    let title = problems.text("title", document.value(&["ArticleTitle"])?.as_deref());
    let book_title = problems.text(
        "book_title",
        document.value(&["Book", "BookTitle"])?.as_deref(),
    );
    let mut types = Vec::new();
    for item in document.children("PublicationType")? {
        if let Some(kind) = publication_type(&item.text()?)
            && !types.contains(&kind)
        {
            types.push(kind);
        }
    }
    let mut refusals = crate::Refusal::None;
    let indexing = super::indexing::parse(document, document, &mut problems, &mut refusals)?;
    Ok(Some(FetchedArticle {
        refusals,
        indexing,
        article: Article {
            text_withheld: Some(crate::article_rights::withheld(pmid)),
            europepmc_id: None,
            pmid: Some(pmid),
            doi: None,
            pmcid: None,
            title,
            book_title,
            bookshelf_url: bookshelf_url(document, &mut problems)?,
            journal: None,
            date: pubdate(document.child("ContributionDate")?, "date", &mut problems)?,
            electronic_date: None,
            types: types.into(),
            corrections: Box::default(),
            authors: authors(document, &mut problems)?.into(),
            abstract_parts: Box::default(),
        },
        problems: problems.list,
    }))
}

fn bookshelf_url(document: Element<'_>, problems: &mut Problems) -> Result<Option<Text>, BadReply> {
    let Some(ids) = document.child("ArticleIdList")? else {
        return Ok(None);
    };
    let mut accession = None;
    for id in ids.children("ArticleId")? {
        if id.attr("IdType")?.as_deref() == Some("bookaccession")
            && accession.replace(id.text()?).is_some()
        {
            return Ok(problems.required("bookshelf_url", None));
        }
    }
    Ok(problems.optional(
        "bookshelf_url",
        accession.map(|accession| {
            // The publisher's Bookshelf path accepts a checked NBK accession.
            if !accession.strip_prefix("NBK").is_some_and(|digits| {
                !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
            }) {
                return Err(BadReply);
            }
            Text::new(&format!("https://www.ncbi.nlm.nih.gov/books/{accession}/"))
                .map_err(|_| BadReply)
        }),
    ))
}
