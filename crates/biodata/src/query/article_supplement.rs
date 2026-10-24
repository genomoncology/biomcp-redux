use super::gene::{QueryInvalid, present};
use crate::{ArticleInput, ProteinChange, Text, Error, macros::text_value};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
/// Inclusive original one-based row range, at most one page.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SheetRows(pub u32, pub u32);
impl FromStr for SheetRows {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self, Error> {
        let (a,b) = s.split_once('-').ok_or(Error::ArticleIdInvalid)?;
        let a: u32 = a.parse().map_err(|_| Error::ArticleIdInvalid)?;
        let b: u32 = b.parse().map_err(|_| Error::ArticleIdInvalid)?;
        if a == 0 || b < a || b - a >= 200 { return Err(Error::ArticleIdInvalid); }
        Ok(Self(a,b))
    }
}
impl fmt::Display for SheetRows {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { write!(f,"{}-{}",self.0,self.1) }
}
text_value!(SheetRows, "SheetRows", r"^[1-9][0-9]*-[1-9][0-9]*$");
/// Read a checked supplement file or page its original rows.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct ArticleSupplement(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// PubMed or PMC identity.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=0)))]
    input: ArticleInput,
    /// Exact file name from the supplements section.
    #[cfg_attr(feature="schema", schemars(extend("x-positional"=1)))]
    file: Text,
    /// Inclusive original row range.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="SheetRows"))]
    rows: Option<SheetRows>,
    /// Sheet name for row selection.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    sheet: Option<Text>,
    /// Literal case-insensitive needle.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="Text"))]
    find: Option<Text>,
    /// Explicit protein substitution.
    #[serde(default, deserialize_with="present", skip_serializing_if="Option::is_none")]
    #[cfg_attr(feature="schema", schemars(with="ProteinChange"))]
    variant: Option<ProteinChange>,
    /// Matching-row or text-fragment offset.
    #[serde(default)]
    offset: u32,
}
impl TryFrom<Fields> for ArticleSupplement {
    type Error = QueryInvalid;
    fn try_from(v: Fields) -> Result<Self, QueryInvalid> {
        if matches!(v.input,ArticleInput::Doi(_)) || [v.rows.is_some(),v.find.is_some(),v.variant.is_some()].into_iter().filter(|v| *v).count() > 1
            || v.file.as_str().contains(['/', '\\']) || v.file.as_str().starts_with('.')
            || v.sheet.is_some() && v.rows.is_none() { return Err(QueryInvalid); }
        Ok(Self(v))
    }
}
impl ArticleSupplement {
    /// Build the initial file action.
    pub fn new(input: ArticleInput, file: Text) -> Result<Self, QueryInvalid> {
        Fields { input,file,rows:None,sheet:None,find:None,variant:None,offset:0 }.try_into()
    }
    /// Original identity.
    pub fn input(&self) -> &ArticleInput { &self.0.input }
    /// Exact name.
    pub fn file(&self) -> &Text { &self.0.file }
    /// Row range.
    pub fn rows(&self) -> Option<&SheetRows> { self.0.rows.as_ref() }
    /// Optional sheet.
    pub fn sheet(&self) -> Option<&Text> { self.0.sheet.as_ref() }
    /// Literal needles; no shape detection occurs.
    pub fn needles(&self) -> Vec<String> { self.forms().map(|v|v.to_vec()).unwrap_or_else(||self.0.find.as_ref().map(|v|vec![v.as_str().to_owned()]).unwrap_or_default()) }
    /// Spellings of an explicit variant.
    pub fn forms(&self) -> Option<[String;4]> { self.0.variant.as_ref().map(ProteinChange::forms) }
    /// Result offset.
    pub fn offset(&self) -> u32 { self.0.offset }
    /// Continue a bounded page.
    pub fn page(&self, offset: u32) -> Self { let mut v=self.clone();v.0.offset=offset;v }
    /// Select the first bounded rows of a sheet.
    pub fn first_rows(&self, sheet: Text, last: u32) -> Self { let mut v=self.clone();v.0.sheet=Some(sheet);v.0.rows=Some(SheetRows(1,last.min(20).max(1)));v.0.offset=0;v }
}
