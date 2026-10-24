use crate::{Error, macros::text_value};
use std::{fmt, str::FromStr};
/// Published variant identifier grammar, without interpreting the allele expression.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct VariantInput(Box<str>);
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
}
impl VariantInput {
    /// Check the identifier's spelling while preserving every supplied character.
    pub fn new(value: &str) -> Result<Self, Error> {
        if value.is_empty() || value.len() > 2048 || !value.bytes().all(|b| b.is_ascii_graphic()) {
            return Err(Error::VariantInputInvalid);
        }
        let input = Self(value.into());
        input.namespace()?;
        Ok(input)
    }
    /// Return the unchanged identifier.
    pub fn as_str(&self) -> &str {
        &self.0
    }
    /// Classify a published namespace, without parsing molecular meaning.
    pub fn namespace(&self) -> Result<VariantNamespace, Error> {
        let value = self.as_str();
        if value.strip_prefix("rs").is_some_and(positive) {
            return Ok(VariantNamespace::Rs);
        }
        if value.strip_prefix("CA").is_some_and(positive) {
            return Ok(VariantNamespace::Ca);
        }
        if positive(value) || value.strip_prefix("ClinVar:").is_some_and(positive) {
            return Ok(VariantNamespace::ClinVar);
        }
        let Some((reference, description)) = value.split_once(':') else {
            return Err(Error::VariantInputInvalid);
        };
        if description.len() < 3
            || !matches!(
                description.as_bytes().first(),
                Some(b'c' | b'g' | b'n' | b'p' | b'r' | b'm')
            )
            || description.as_bytes().get(1) != Some(&b'.')
        {
            return Err(Error::VariantInputInvalid);
        }
        if reference
            .strip_prefix("chr")
            .is_some_and(|s| matches!(s, "X" | "Y" | "M") || positive(s))
        {
            return Ok(VariantNamespace::Chromosome);
        }
        let valid = if let Some(tail) = reference.strip_prefix("LRG_") {
            let mut parts = tail.split(['t', 'p']);
            parts.next().is_some_and(positive) && parts.next().is_none_or(positive) && parts.next().is_none()
        } else {
            ["NC_", "NM_", "NP_", "NG_", "NR_", "NT_", "NW_", "ENST", "ENSP"]
                .iter().any(|prefix| reference.strip_prefix(prefix).is_some_and(accession))
        };
        if valid {
            Ok(VariantNamespace::Hgvs)
        } else {
            Err(Error::VariantInputInvalid)
        }
    }
}
fn accession(value: &str) -> bool {
    let Some((number, version)) = value.split_once('.') else { return false; };
    !number.is_empty() && number.bytes().all(|b| b.is_ascii_digit()) && positive(version)
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
text_value!(VariantInput, "VariantInput", r"^(rs[1-9][0-9]*|CA[1-9][0-9]*|(ClinVar:)?[1-9][0-9]*|((N[CGMPRTW]_[0-9]+|ENS[TP][0-9]+)\.[1-9][0-9]*|LRG_[1-9][0-9]*([tp][1-9][0-9]*)?|chr([1-9][0-9]*|X|Y|M)):[cgnprm]\.[\x21-\x7e]+)$");
