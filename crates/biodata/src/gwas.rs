//! Checked GWAS Catalog identities and exact source probabilities.
use crate::{Text, text_value};
use std::{fmt, str::FromStr};

/// Exact Catalog study accession; case and leading zeros survive.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GwasStudyId(Text);
impl FromStr for GwasStudyId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digits = value
            .strip_prefix("GCST")
            .ok_or(crate::Error::GwasValueInvalid)?;
        if digits.len() < 6
            || !digits.bytes().all(|b| b.is_ascii_digit())
            || !digits.bytes().any(|b| b != b'0')
        {
            return Err(crate::Error::GwasValueInvalid);
        }
        Ok(Self(
            Text::new(value).map_err(|_| crate::Error::GwasValueInvalid)?,
        ))
    }
}
impl fmt::Display for GwasStudyId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
text_value!(
    GwasStudyId,
    "GwasStudyId",
    "^GCST[0-9]{6,}$",
    "Exact Catalog study accession; case and leading zeros survive."
);

/// A positive Catalog association ID qualified by resource kind.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct GwasAssociationId(std::num::NonZeroU64);
impl GwasAssociationId {
    /// Native numeric ID.
    pub fn get(self) -> u64 {
        self.0.get()
    }
}
impl FromStr for GwasAssociationId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digits = value
            .strip_prefix("association:")
            .ok_or(crate::Error::GwasValueInvalid)?;
        if digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(crate::Error::GwasValueInvalid);
        }
        Ok(Self(
            std::num::NonZeroU64::new(digits.parse().map_err(|_| crate::Error::GwasValueInvalid)?)
                .ok_or(crate::Error::GwasValueInvalid)?,
        ))
    }
}
impl fmt::Display for GwasAssociationId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "association:{}", self.get())
    }
}
text_value!(
    GwasAssociationId,
    "GwasAssociationId",
    "^association:[1-9][0-9]*$",
    "A positive Catalog association ID qualified by resource kind."
);

/// Native ontology identifier; namespaces are preserved without an EFO restriction.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GwasTraitId(Text);
impl FromStr for GwasTraitId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let (namespace, local) = value
            .split_once(['_', ':'])
            .ok_or(crate::Error::GwasValueInvalid)?;
        if namespace.is_empty()
            || local.is_empty()
            || !namespace.bytes().all(|b| b.is_ascii_alphanumeric())
            || !local
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err(crate::Error::GwasValueInvalid);
        }
        Ok(Self(
            Text::new(value).map_err(|_| crate::Error::GwasValueInvalid)?,
        ))
    }
}
impl fmt::Display for GwasTraitId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
text_value!(
    GwasTraitId,
    "GwasTraitId",
    "^[A-Za-z0-9]+[_:][A-Za-z0-9_-]+$",
    "Native ontology identifier; namespaces are preserved without an EFO restriction."
);

mod probability;
pub use probability::ScientificPValue;
