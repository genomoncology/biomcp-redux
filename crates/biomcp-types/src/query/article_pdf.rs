use crate::{ArticleInput, Text};
use serde::{Deserialize, Serialize};
/// Download one publisher PDF to an explicit file or existing folder.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct ArticlePdf {
    /// Article identity.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    pub input: ArticleInput,
    /// Destination file or existing folder.
    pub save: Text,
    /// Replace an existing file only when explicitly requested.
    #[serde(default)]
    pub replace: bool,
}

impl ArticlePdf {
    /// Portable filename for an explicit folder destination or suggested next action.
    pub fn filename(input: &ArticleInput) -> String {
        let name: String = input
            .to_string()
            .chars()
            .map(|c| {
                if c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_') {
                    c
                } else {
                    '_'
                }
            })
            .collect();
        format!("{name}.pdf")
    }
}

crate::query::validation::checked_query!(ArticlePdf);
