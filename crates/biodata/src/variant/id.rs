use crate::{Error, macros::text_value};
use std::{fmt, str::FromStr};
/// A published variant identifier; the namespace never comes from biological meaning.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum VariantId {
    /// dbSNP RefSNP identifier.
    Rs(u64),
    /// ClinGen canonical nucleotide allele.
    Ca(u64),
    /// ClinVar Variation ID.
    ClinVarVariation(u64),
    /// ClinVar Allele ID.
    ClinVarAllele(u64),
    /// ClinVar versioned or unversioned VCV, RCV or SCV accession.
    ClinVarAccession(Box<str>),
    /// A source-supplied VRS allele digest.
    Vrs(Box<str>),
}
impl FromStr for VariantId {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        for (prefix, make) in [
            ("rs", Self::Rs as fn(u64) -> Self),
            
            
            ("clinvar:", Self::ClinVarVariation),
            ("clinvar.allele:", Self::ClinVarAllele),
        ] {
            if let Some(number) = value.strip_prefix(prefix) {
                let n = number.parse::<u64>().map_err(|_| Error::VariantIdInvalid)?;
                return if n > 0 && n.to_string() == number {
                    Ok(make(n))
                } else {
                    Err(Error::VariantIdInvalid)
                };
            }
        }
        for (prefix, make) in [("CA", Self::Ca as fn(u64) -> Self)] {
            if let Some(number) = value.strip_prefix(prefix) {
                let n = number.parse::<u64>().map_err(|_| Error::VariantIdInvalid)?;
                let id = make(n);
                return if n > 0 && id.to_string() == value { Ok(id) } else { Err(Error::VariantIdInvalid) };
            }
        }
        if let Some(digest) = value.strip_prefix("ga4gh:VA.") {
            if digest.len() == 32
                && digest
                    .bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
            {
                return Ok(Self::Vrs(value.into()));
            }
        }
        let (accession, version) = value
            .split_once('.')
            .map_or((value, None), |(a, v)| (a, Some(v)));
        let valid = ["VCV", "RCV", "SCV"].iter().any(|prefix| {
            accession.strip_prefix(prefix).is_some_and(|n| {
                n.len() == 9 && n.bytes().all(|b| b.is_ascii_digit()) && n != "000000000"
            })
        });
        if valid
            && version.is_none_or(|v| {
                !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()) && !v.starts_with('0')
            })
        {
            return Ok(Self::ClinVarAccession(value.into()));
        }
        Err(Error::VariantIdInvalid)
    }
}
impl fmt::Display for VariantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rs(n) => write!(f, "rs{n}"),
            Self::Ca(n) => write!(f, "CA{n:06}"),
            Self::ClinVarVariation(n) => write!(f, "clinvar:{n}"),
            Self::ClinVarAllele(n) => write!(f, "clinvar.allele:{n}"),
            Self::ClinVarAccession(s) | Self::Vrs(s) => f.write_str(s),
        }
    }
}
text_value!(
    VariantId,
    "VariantId",
    r"^(((rs|clinvar:|clinvar\.allele:)[1-9][0-9]*|CA[0-9]{6,})|(VCV|RCV|SCV)[0-9]{9}(\.[1-9][0-9]*)?|ga4gh:VA\.[A-Za-z0-9_-]{32})$"
);
