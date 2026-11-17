//! Retain only book citation fields, including within their XML containers.
use crate::xml_element::{Element, body_nodes};
use biomcp_fetch::BadReply;
use quick_xml::{Reader, escape::escape, events::Event};
use std::borrow::Cow;
use std::fmt::Write as _;

pub(super) fn book_citations(xml: &str) -> Result<Cow<'_, str>, BadReply> {
    if !xml.contains("<PubmedBookArticle") {
        return Ok(Cow::Borrowed(xml));
    }
    let root = Element::root(xml, "PubmedArticleSet")?;
    let mut filtered = String::from("<PubmedArticleSet>");
    for (kind, node) in body_nodes(root.0)? {
        match kind.as_str() {
            "PubmedArticle" | "ERROR" => filtered.push_str(node.0),
            "PubmedBookArticle" => {
                filtered.push_str("<PubmedBookArticle>");
                if let Some(doc) = node.child("BookDocument")? {
                    citation_fields(&mut filtered, doc)?;
                }
                filtered.push_str("</PubmedBookArticle>");
            }
            _ => (),
        }
    }
    filtered.push_str("</PubmedArticleSet>");
    Ok(Cow::Owned(filtered))
}

fn citation_fields(out: &mut String, doc: Element<'_>) -> Result<(), BadReply> {
    out.push_str("<BookDocument>");
    fields(out, doc, &["PMID", "ArticleTitle", "PublicationType"])?;
    if let Some(book) = doc.child("Book")? {
        out.push_str("<Book>");
        fields(out, book, &["BookTitle"])?;
        out.push_str("</Book>");
    }
    if let Some(date) = doc.child("ContributionDate")? {
        out.push_str("<ContributionDate>");
        fields(out, date, &["Year", "Month", "Day"])?;
        out.push_str("</ContributionDate>");
    }
    if let Some(authors) = doc.child("AuthorList")? {
        out.push_str("<AuthorList>");
        for author in authors.children("Author")? {
            out.push_str("<Author>");
            fields(out, author, &["LastName", "Initials", "CollectiveName"])?;
            out.push_str("</Author>");
        }
        out.push_str("</AuthorList>");
    }
    if let Some(ids) = doc.child("ArticleIdList")? {
        out.push_str("<ArticleIdList>");
        for id in ids.children("ArticleId")? {
            if id.attr("IdType")?.as_deref() == Some("bookaccession") {
                let id = citation_text(id, false)?.unwrap_or_default();
                let _ = write!(
                    out,
                    "<ArticleId IdType=\"bookaccession\">{}</ArticleId>",
                    escape(&id)
                );
            }
        }
        out.push_str("</ArticleIdList>");
    }
    out.push_str("</BookDocument>");
    Ok(())
}

fn fields(out: &mut String, parent: Element<'_>, names: &[&str]) -> Result<(), BadReply> {
    for name in names {
        for field in parent.children(name)? {
            let mixed = matches!(*name, "ArticleTitle" | "BookTitle" | "CollectiveName");
            if let Some(text) = citation_text(field, mixed)? {
                let _ = write!(out, "<{name}>{}</{name}>", escape(&text));
            }
        }
    }
    Ok(())
}

fn citation_text(field: Element<'_>, mixed: bool) -> Result<Option<String>, BadReply> {
    let mut reader = Reader::from_str(field.0);
    reader.read_event().map_err(|_| BadReply)?;
    loop {
        match reader.read_event().map_err(|_| BadReply)? {
            Event::Start(tag) | Event::Empty(tag) => {
                if !mixed || !matches!(tag.name().as_ref(), "b" | "i" | "sup" | "sub" | "u") {
                    return Ok(None);
                }
            }
            Event::Eof => return field.text().map(Some),
            _ => (),
        }
    }
}
