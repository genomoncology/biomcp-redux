use crate::text_value;
use crate::{EnsemblGeneId, Error, HgncId, NcbiGeneId, Text};
use std::{fmt, str::FromStr};

/// An input classified before source resolution; aliases remain free text.
#[derive(Clone, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum GeneInput {
    /// An explicitly namespaced HGNC identifier.
    HgncId(HgncId),
    /// An explicitly namespaced Ensembl gene identifier.
    EnsemblId(EnsemblGeneId),
    /// A decimal NCBI Gene identifier.
    EntrezId(NcbiGeneId),
    /// A symbol or alias for source resolution.
    Text(Text),
}
impl GeneInput {
    /// Trim input and classify explicit identifiers before alias text.
    pub fn new(value: &str) -> Result<Self, Error> {
        let value = value.trim();
        if value
            .get(..5)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("HGNC:"))
        {
            return value.parse().map(Self::HgncId);
        }
        if value
            .get(..4)
            .is_some_and(|prefix| prefix.eq_ignore_ascii_case("ENSG"))
        {
            return value.parse().map(Self::EnsemblId);
        }
        if !value.is_empty() && value.bytes().all(|ch| ch.is_ascii_digit()) {
            return value.parse().map(Self::EntrezId);
        }
        Text::new(value).map(Self::Text)
    }
}
impl FromStr for GeneInput {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::new(value)
    }
}

impl fmt::Display for GeneInput {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::HgncId(value) => value.fmt(formatter),
            Self::EnsemblId(value) => value.fmt(formatter),
            Self::EntrezId(value) => value.fmt(formatter),
            Self::Text(value) => value.fmt(formatter),
        }
    }
}
text_value!(
    GeneInput,
    "GeneInput",
    r"\S",
    "An input classified before source resolution; aliases remain free text."
);
