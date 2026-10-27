//! Select article key identity from identifiers retained by the released search.
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

#[derive(Clone, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct ArticleKey {
    pub(super) article_key: String,
    field: Field,
}
#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum Field {
    Namespace,
    Item,
}
impl ArticleKey {
    pub(super) fn select(&self, value: &Value) -> Result<Value, String> {
        let row = super::super::assertions::select(value, &self.article_key)?;
        for (name, namespace) in [
            ("pmid", "pmid"),
            ("pmcid", "pmc"),
            ("doi", "doi"),
            ("semantic_scholar_id", "semantic_scholar"),
        ] {
            if let Some(raw) = row.get(name) {
                let id = raw
                    .as_str()
                    .ok_or("released article identifier requires a string")?;
                if !id.is_empty() {
                    return Ok(match self.field {
                        Field::Namespace => json!(namespace),
                        Field::Item => json!({"key":{"namespace":namespace,"id":id}}),
                    });
                }
            }
        }
        Err("released article has no retained identifier".into())
    }
}
