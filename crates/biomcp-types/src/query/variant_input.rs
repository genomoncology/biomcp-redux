use crate::{Error, text_value};
use std::{fmt, str::FromStr};
/// Published variant identifier grammar, without interpreting the allele expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantInput(crate::Text);
/// Namespace selected solely from the published identifier form.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VariantNamespace {
    /// dbSNP reference SNP identifier.
    Rs,
    /// ClinGen canonical allele identifier.
    Ca,
    /// ClinVar numeric Variation ID.
    ClinVar,
    /// Reference-qualified opaque HGVS description.
    Hgvs,
    /// Chromosome HGVS description requiring an explicit build.
    Chromosome,
    /// Identifier vocabulary is recognized but this get slice has no source route.
    Unsupported,
}
impl VariantInput {
    #[cfg(feature = "schema")]
    pub(crate) const CHROMOSOME_PATTERN: &'static str =
        r"^chr([1-9]|1[0-9]|2[0-2]|X|Y|M|MT):g\.[\x21-\x7e]*[0-9][\x21-\x7e]*$";

    /// Maximum bytes in one published identifier.
    pub const MAX_BYTES: usize = 2048;
    /// Check the identifier's spelling while preserving every supplied character.
    pub fn new(value: &str) -> Result<Self, Error> {
        if value.is_empty()
            || value.len() > Self::MAX_BYTES
            || !value.bytes().all(|b| b.is_ascii_graphic())
        {
            return Err(Error::VariantInputInvalid);
        }
        let input = Self(crate::Text::new(value).map_err(|_| Error::VariantInputInvalid)?);
        input.namespace()?;
        Ok(input)
    }
    /// Return the unchanged identifier.
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
    /// Borrow the already checked input text without reparsing or trimming it.
    pub fn text(&self) -> &crate::Text {
        &self.0
    }
    /// Classify a published namespace, without parsing molecular meaning.
    pub fn namespace(&self) -> Result<VariantNamespace, Error> {
        let value = self.as_str();
        if let Ok(id) = value.parse::<crate::VariantId>() {
            return match id {
                crate::VariantId::Rs(_) => Ok(VariantNamespace::Rs),
                crate::VariantId::Ca(_) => Ok(VariantNamespace::Ca),
                crate::VariantId::ClinVarVariation(_) => Ok(VariantNamespace::ClinVar),
                _ => Ok(VariantNamespace::Unsupported),
            };
        }
        if positive(value) && value.parse::<u64>().is_ok() {
            return Ok(VariantNamespace::ClinVar);
        }
        if value.parse::<crate::HgvsText>().is_ok() {
            return Ok(VariantNamespace::Hgvs);
        }
        if crate::Expression::new(crate::ExpressionSyntax::HgvsG, self.0.clone(), None).is_ok() {
            return Ok(VariantNamespace::Chromosome);
        }
        Err(Error::VariantInputInvalid)
    }
}
fn positive(value: &str) -> bool {
    !value.is_empty() && !value.starts_with('0') && value.bytes().all(|b| b.is_ascii_digit())
}
impl FromStr for VariantInput {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::new(value)
    }
}
impl fmt::Display for VariantInput {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    VariantInput,
    "VariantInput",
    format!(
        "^(?=[\\x21-\\x7e]{{1,{}}}$)({}|{}|^{}$|{})",
        Self::MAX_BYTES,
        crate::VariantId::pattern(),
        crate::HgvsText::pattern(),
        biodata::schema_pattern::decimal(u64::MAX),
        Self::CHROMOSOME_PATTERN
    )
);
