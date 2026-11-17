//! PubMed abstracts follow source terms unless the record has explicit restrictions.
use super::Licenses;
use super::pubmed_book::book_citations;
use crate::xml_element::{Element, descendants};
use biodata::Pmid;
use biomcp_fetch::{BadReply, Leg};
use biomcp_types::Use;

/// Look licenses up before the source call, so a text check never waits on a
/// permit its own call holds.
pub(crate) fn lookup(leg: &Leg<'_>, ids: &[Pmid]) -> Licenses {
    if leg.disabled_use(Use::PerRecord) {
        super::licenses(leg, ids)
    } else {
        Licenses::default()
    }
}
/// Read back the marker the filter left on this record.
pub(crate) fn notice(
    root: &Element<'_>,
    id: Pmid,
) -> Result<Option<biomcp_types::WithheldArticleText>, BadReply> {
    Ok(match root.child("BioMcpTextWithheld")? {
        Some(_) => Some(super::withheld(id)),
        None => root
            .child("BioMcpTextUnchecked")?
            .map(|_| super::unchecked(id)),
    })
}
pub(crate) fn filter(leg: &Leg<'_>, body: &[u8], licenses: &Licenses) -> Result<Vec<u8>, BadReply> {
    let xml = std::str::from_utf8(body).map_err(|_| BadReply)?;
    let books_filtered = book_citations(xml)?;
    if !leg.disabled_use(Use::PerRecord) {
        return Ok(books_filtered.into_owned().into_bytes());
    }
    let xml = books_filtered.as_ref();
    let articles = descendants(xml, "PubmedArticle")?;
    let mut filtered = xml.to_owned();
    for article in articles {
        let id = article
            .value(&["MedlineCitation", "PMID"])?
            .as_deref()
            .and_then(super::policy::parse_pmid);
        let permitted = super::permitted(id.and_then(|id| licenses.get(id)), true);
        let unchecked = id.is_some_and(|id| licenses.unchecked(id));
        if permitted && !unchecked {
            continue;
        }
        let mut replacement = article.0.to_owned();
        if !permitted {
            for name in ["Abstract", "OtherAbstract"] {
                for text in descendants(article.0, name)? {
                    replacement = replacement.replace(text.0, "");
                }
            }
        }
        // A refusal already on a cached record stands; repeat reads add nothing.
        if article.child("BioMcpTextWithheld")?.is_none() {
            let marker = if unchecked {
                "<BioMcpTextUnchecked/>"
            } else {
                "<BioMcpTextWithheld/>"
            };
            let closing = replacement.rfind("</PubmedArticle").ok_or(BadReply)?;
            replacement.insert_str(closing, marker);
        }
        filtered = filtered.replacen(article.0, &replacement, 1);
    }
    // Keep all citation records, including those with no readable abstract.
    Element::root(&filtered, "PubmedArticleSet")?;
    Ok(filtered.into_bytes())
}

#[cfg(test)]
mod tests {
    use super::book_citations;

    #[test]
    fn book_metadata_only_survives_the_retention_filter() {
        let book = concat!(
            "<PubmedBookArticle><BookDocument><PMID>21882399</PMID>",
            "<Book><BookTitle>GeneReviews</BookTitle><Abstract>restricted book text</Abstract></Book>",
            "<Abstract>restricted abstract</Abstract><Sections>restricted sections</Sections>",
            "<AuthorList><Abstract>restricted nested list text</Abstract>",
            "<Author><LastName>Adam</LastName><Initials>MP</Initials>",
            "<Abstract>restricted nested author text</Abstract></Author></AuthorList>",
            "<ArticleIdList><ArticleId IdType=\"bookaccession\"><Abstract>restricted accession text</Abstract>",
            "</ArticleId></ArticleIdList>",
            "</BookDocument><PubmedBookData>unneeded book data</PubmedBookData></PubmedBookArticle>",
        );
        let raw = format!(
            "<PubmedArticleSet><!--{book}-->{book}<PubmedArticle><MedlineCitation><Article><Abstract>journal policy input</Abstract></Article></MedlineCitation></PubmedArticle></PubmedArticleSet>"
        );
        let filtered = book_citations(&raw).unwrap();
        assert_eq!(
            filtered,
            concat!(
                "<PubmedArticleSet><PubmedBookArticle><BookDocument><PMID>21882399</PMID>",
                "<Book><BookTitle>GeneReviews</BookTitle></Book>",
                "<AuthorList><Author><LastName>Adam</LastName><Initials>MP</Initials></Author>",
                "</AuthorList><ArticleIdList><ArticleId IdType=\"bookaccession\"></ArticleId></ArticleIdList>",
                "</BookDocument></PubmedBookArticle>",
                "<PubmedArticle><MedlineCitation><Article><Abstract>journal policy input</Abstract>",
                "</Article></MedlineCitation></PubmedArticle></PubmedArticleSet>"
            )
        );
        assert_eq!(book_citations(&filtered).unwrap(), filtered);
    }
}
