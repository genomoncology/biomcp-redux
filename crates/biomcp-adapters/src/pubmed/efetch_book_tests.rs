//! Check Bookshelf link admission for missing, malformed and conflicting chapter accessions.
use super::efetch_book::chapter;
use crate::xml_element::Element;

#[test]
fn absent_invalid_and_conflicting_accessions_do_not_create_links() {
    for ids in [
        "",
        r#"<ArticleId IdType="bookaccession">NBK62111/other</ArticleId>"#,
        r#"<ArticleId IdType="bookaccession">NBK62111</ArticleId><ArticleId IdType="bookaccession">NBK1109</ArticleId>"#,
    ] {
        let xml = format!(
            concat!(
                "<PubmedBookArticle><BookDocument><PMID>21882399</PMID>",
                "<ArticleTitle>Kabuki Syndrome</ArticleTitle><ArticleIdList>{}</ArticleIdList>",
                "</BookDocument></PubmedBookArticle>"
            ),
            ids
        );
        assert!(
            chapter(Element(&xml))
                .unwrap()
                .expect("checked PMID")
                .article
                .bookshelf_url
                .is_none()
        );
    }
}
