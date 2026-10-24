use crate::ArticleInput;
use super::gene::QueryInvalid;
use serde::{Deserialize, Serialize};
/// Get one to twenty article inputs in caller order.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature="schema", schemars(with="Fields"))]
pub struct ArticleGet(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature="schema", derive(schemars::JsonSchema))]
struct Fields {
    /// PubMed, PMC or DOI identifiers in input order.
    #[cfg_attr(feature="schema", schemars(length(min=1,max=20), extend("x-positional"=0)))]
    input: Box<[ArticleInput]>,
}
impl TryFrom<Fields> for ArticleGet {
    type Error=QueryInvalid;
    fn try_from(fields:Fields)->Result<Self,QueryInvalid> { Self::new(fields.input) }
}
impl ArticleGet {
    /// Check batch bounds.
    pub fn new(input:Box<[ArticleInput]>)->Result<Self,QueryInvalid> {
        if !(1..=20).contains(&input.len()) { return Err(QueryInvalid); }
        Ok(Self(Fields { input }))
    }
    /// Original selectors.
    pub fn input(&self)->&[ArticleInput] { &self.0.input }
}
