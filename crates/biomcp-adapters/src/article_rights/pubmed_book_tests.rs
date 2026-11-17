//! Check that PubMed chapter XML retention excludes noncitation text before storage.
use super::pubmed_book::book_citations;

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
        concat!(
            "<PubmedArticleSet><!--{}-->{}<PubmedArticle><MedlineCitation><Article>",
            "<Abstract>journal policy input</Abstract></Article></MedlineCitation>",
            "</PubmedArticle></PubmedArticleSet>"
        ),
        book, book
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
