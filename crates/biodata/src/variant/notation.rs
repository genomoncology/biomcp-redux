use crate::{Accession, Error, Text, macros::text_value};
use serde::{Deserialize, Serialize};
use std::{fmt, str::FromStr};
/// Reference-qualified notation with its original text, without a molecular parse.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HgvsText(Box<str>);
impl HgvsText {
    /// Borrow the unchanged published description.
    pub fn as_str(&self) -> &str { &self.0 }
    /// The checked versioned accession preceding the description.
    pub fn accession(&self) -> Result<Accession, Error> { self.0.split_once(':').ok_or(Error::NotationInvalid)?.0.parse() }
}
impl FromStr for HgvsText {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let (reference, body) = value.split_once(':').ok_or(Error::NotationInvalid)?;
        let _: Accession = reference.parse()?;
        if body.len() < 3 || !matches!(body.as_bytes().first(), Some(b'g' | b'c' | b'n' | b'r' | b'p' | b'm' | b'o')) || body.as_bytes().get(1) != Some(&b'.') || !body.bytes().all(|b| b.is_ascii() && !b.is_ascii_control()) { return Err(Error::NotationInvalid); }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for HgvsText { fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) } }
text_value!(HgvsText, "HgvsText", r"^((N[CGMPRTW]|X[MPR])_([0-9]{6}|[0-9]{9})\.[0-9]+|ENS[GTP][0-9]{11}\.[0-9]+|LRG_[0-9]+([tp][0-9]+)?):[gcnrpmo]\.[\x20-\x7e]+$");
crate::source::canonical_enum! {
    /// Published syntax label for a source-preserved expression.
    ExpressionSyntax, NotationInvalid {
        /// HGVS genomic description.
        HgvsG => "hgvs.g",
        /// HGVS coding description.
        HgvsC => "hgvs.c",
        /// HGVS noncoding transcript description.
        HgvsN => "hgvs.n",
        /// HGVS RNA description.
        HgvsR => "hgvs.r",
        /// HGVS protein description.
        HgvsP => "hgvs.p",
        /// HGVS mitochondrial description.
        HgvsM => "hgvs.m",
        /// HGVS circular genomic description.
        HgvsO => "hgvs.o",
        /// NCBI SPDI interbase description.
        Spdi => "spdi",
        /// gnomAD chromosome-position-reference-alternate identifier.
        Gnomad => "gnomad",
        /// Padded VCF chromosome-position-reference-alternate text.
        Vcf => "vcf",
        /// Source-preserved cytogenetic notation.
        Iscn => "iscn",
    }
}
/// Source expression checked for its declared published form, without normalization.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct Expression(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields { syntax: ExpressionSyntax, value: Text, #[serde(skip_serializing_if = "Option::is_none")] syntax_version: Option<Text> }
impl Expression {
    /// Retain source text after a form check. Unsupported forms are refused explicitly.
    pub fn new(syntax: ExpressionSyntax, value: Text, syntax_version: Option<Text>) -> Result<Self, Error> {
        let expected = match syntax { ExpressionSyntax::HgvsG => 'g', ExpressionSyntax::HgvsC => 'c', ExpressionSyntax::HgvsN => 'n', ExpressionSyntax::HgvsR => 'r', ExpressionSyntax::HgvsP => 'p', ExpressionSyntax::HgvsM => 'm', ExpressionSyntax::HgvsO => 'o', _ => return Err(Error::NotationInvalid) };
        let (_, body) = value.as_str().split_once(':').ok_or(Error::NotationInvalid)?;
        let qualified = value.as_str().parse::<HgvsText>().is_ok();
        let chromosome = syntax == ExpressionSyntax::HgvsG && value.as_str().strip_prefix("chr").and_then(|v| v.split_once(':')).is_some_and(|(c, _)| c.parse::<crate::Chromosome>().is_ok());
        if (!qualified && !chromosome) || !body.starts_with(&format!("{expected}.")) || body.len() < 3 { return Err(Error::NotationInvalid); }
        Ok(Self(Fields { syntax, value, syntax_version }))
    }
    /// Unchanged source notation.
    pub fn value(&self) -> &Text { &self.0.value }
    /// Declared notation syntax.
    pub fn syntax(&self) -> ExpressionSyntax { self.0.syntax }
    /// Construct from reference-qualified HGVS without interpreting its edit.
    pub fn hgvs(value: Text) -> Result<Self, Error> {
        let syntax = match value.as_str().split_once(':').and_then(|(_, b)| b.chars().next()) { Some('g') => ExpressionSyntax::HgvsG, Some('c') => ExpressionSyntax::HgvsC, Some('n') => ExpressionSyntax::HgvsN, Some('r') => ExpressionSyntax::HgvsR, Some('p') => ExpressionSyntax::HgvsP, Some('m') => ExpressionSyntax::HgvsM, Some('o') => ExpressionSyntax::HgvsO, _ => return Err(Error::NotationInvalid) };
        Self::new(syntax, value, None)
    }
}
impl TryFrom<Fields> for Expression { type Error = Error; fn try_from(f: Fields) -> Result<Self, Error> { Self::new(f.syntax, f.value, f.syntax_version) } }
