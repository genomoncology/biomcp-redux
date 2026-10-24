//! Data-only presentation hints; paths refer exclusively to response JSON.
pub(super) struct Hint {
    pub(super) schema: &'static str,
    pub(super) title: &'static str,
    pub(super) columns: &'static [&'static str],
}
const HINTS: &[Hint] = &[
    Hint { schema: "biomcp.gene_cards/1", title: "/answer/gene/symbol", columns: &[] },
    Hint { schema: "biomcp.article_cards/1", title: "/answer/card/title", columns: &[] },
    Hint { schema: "biomcp.gene_page/1", title: "", columns: &["symbol", "name", "hgnc_id", "location", "status"] },
    Hint { schema: "biomcp.article_page/1", title: "", columns: &["pmid", "header/title", "header/journal", "header/date", "found_by", "cited_by"] },
    Hint { schema: "biomcp.article_edges/1", title: "", columns: &["pmid", "header/title", "header/journal", "cited_by"] },
];
pub(super) fn find(schema: &str) -> Option<&'static Hint> {
    HINTS.iter().find(|hint| hint.schema == schema)
}
