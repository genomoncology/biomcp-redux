use crate::{Error, text_value};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU32,
    str::FromStr,
};

/// A positive NCBI Gene identifier in canonical decimal text.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct NcbiGeneId(NonZeroU32);
impl NcbiGeneId {
    /// Check a namespace-local positive number.
    pub fn new(value: u32) -> Result<Self, Error> {
        NonZeroU32::new(value)
            .map(Self)
            .ok_or(Error::NcbiGeneIdInvalid)
    }
    /// Return the namespace-local number.
    pub fn get(self) -> u32 {
        self.0.get()
    }
}
impl FromStr for NcbiGeneId {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        if value.starts_with('0') || !value.bytes().all(|ch| ch.is_ascii_digit()) {
            return Err(Error::NcbiGeneIdInvalid);
        }
        Self::new(value.parse().map_err(|_| Error::NcbiGeneIdInvalid)?)
    }
}
impl fmt::Display for NcbiGeneId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.get().fmt(f)
    }
}
text_value!(
    NcbiGeneId,
    "NcbiGeneId",
    "^[1-9][0-9]*$",
    "A positive NCBI Gene identifier in canonical decimal text."
);

/// Source spelling of a gene symbol, with ASCII case-insensitive identity.
#[derive(Clone, Debug)]
pub struct GeneSymbol(Box<str>);
impl GeneSymbol {
    /// Maximum accepted symbol length in bytes.
    pub const MAX_LEN: usize = 255;
    /// Check the approved-symbol alphabet without inferring HGNC membership.
    pub fn new(value: &str) -> Result<Self, Error> {
        if value.is_empty()
            || value.len() > Self::MAX_LEN
            || !value
                .as_bytes()
                .first()
                .is_some_and(u8::is_ascii_alphabetic)
            || !value
                .bytes()
                .all(|ch| ch.is_ascii_alphanumeric() || b"-._/@".contains(&ch))
        {
            return Err(Error::GeneSymbolInvalid);
        }
        Ok(Self(value.into()))
    }
    /// Borrow the preserved source spelling.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl PartialEq for GeneSymbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq_ignore_ascii_case(&other.0)
    }
}
impl Eq for GeneSymbol {}
impl Hash for GeneSymbol {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for ch in self.0.bytes() {
            ch.to_ascii_lowercase().hash(state);
        }
    }
}
impl FromStr for GeneSymbol {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        Self::new(value)
    }
}
impl fmt::Display for GeneSymbol {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}
text_value!(
    GeneSymbol,
    "GeneSymbol",
    format!(r"^[A-Za-z][A-Za-z0-9._/@-]{{0,{}}}$", Self::MAX_LEN - 1),
    "Source spelling of a gene symbol, with ASCII case-insensitive identity."
);

crate::checked_id!(
    OmimId,
    OmimIdInvalid,
    |value: &str| value.len() == 6 && value.bytes().all(|ch| ch.is_ascii_digit()),
    "^[0-9]{6}$",
    " A six-digit OMIM identifier, preserving all six digits."
);
crate::checked_id!(
    UniProtAcc,
    UniProtAccInvalid,
    uniprot,
    "^([OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9]([A-Z][A-Z0-9]{2}[0-9]){1,2})$",
    " A whole UniProt base accession, without an isoform suffix."
);
crate::checked_id!(
    Cytoband,
    CytobandInvalid,
    cytoband,
    "^([1-9]|1[0-9]|2[0-2]|X|Y)[pq][1-9][0-9](\\.[0-9]+)?$",
    " A single nuclear cytogenetic band, without a range or location descriptor."
);
fn uniprot(value: &str) -> bool {
    let bytes = value.as_bytes();
    let Some((&first, rest)) = bytes.split_first() else {
        return false;
    };
    if !rest.first().is_some_and(u8::is_ascii_digit) {
        return false;
    }
    if matches!(first, b'O' | b'P' | b'Q') {
        return bytes.len() == 6
            && bytes.get(2..5).is_some_and(|part| {
                part.iter().all(u8::is_ascii_alphanumeric)
                    && part.iter().all(|ch| !ch.is_ascii_lowercase())
            })
            && bytes.last().is_some_and(u8::is_ascii_digit);
    }
    first.is_ascii_uppercase()
        && !matches!(first, b'O' | b'P' | b'Q')
        && matches!(bytes.len(), 6 | 10)
        && bytes.get(2..).is_some_and(|tail| {
            tail.chunks_exact(4).all(|part| match part {
                [a, b, c, d] => {
                    a.is_ascii_uppercase()
                        && [b, c]
                            .iter()
                            .all(|ch| ch.is_ascii_uppercase() || ch.is_ascii_digit())
                        && d.is_ascii_digit()
                }
                _ => false,
            })
        })
}
fn cytoband(value: &str) -> bool {
    let Some((chromosome, band)) = value.split_once(['p', 'q']) else {
        return false;
    };
    let chromosome_ok = matches!(chromosome, "X" | "Y")
        || chromosome
            .parse::<u8>()
            .is_ok_and(|number| (1..=22).contains(&number) && number.to_string() == chromosome);
    let (major, minor) = band
        .split_once('.')
        .map_or((band, None), |(major, minor)| (major, Some(minor)));
    chromosome_ok
        && major.len() == 2
        && !major.starts_with('0')
        && major.bytes().all(|ch| ch.is_ascii_digit())
        && minor
            .is_none_or(|minor| !minor.is_empty() && minor.bytes().all(|ch| ch.is_ascii_digit()))
}
