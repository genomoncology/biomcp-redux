use crate::{Error, Text};
use serde::{Deserialize, Serialize};
/// Nonempty source text whose Markdown structure is preserved by text views.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(transparent)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(transparent, extend("x-markdown"=true)))]
pub struct Markdown(Text);
impl Markdown {
    /// Check the same nonempty boundary as ordinary text.
    pub fn new(value: &str) -> Result<Self, Error> { Text::new(value).map(Self) }
    /// Borrow the source Markdown.
    pub fn as_str(&self) -> &str { self.0.as_str() }
}
