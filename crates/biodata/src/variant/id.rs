use crate::{Error, text_value};
use std::{
    fmt,
    hash::{Hash, Hasher},
    num::NonZeroU64,
    str::FromStr,
};
macro_rules! numeric_id {
    ($(#[doc = $doc:literal])* $name:ident, $error:ident, $prefix:literal, $padding:literal) => {
        $(#[doc = $doc])*
        #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
        pub struct $name(NonZeroU64);
        impl $name {
            /// Check a positive namespace-local number.
            pub fn new(value: u64) -> Result<Self, Error> {
                NonZeroU64::new(value).map(Self).ok_or(Error::$error)
            }
            /// Return the namespace-local number.
            pub fn get(self) -> u64 { self.0.get() }
        }
        impl FromStr for $name {
            type Err = Error;
            fn from_str(text: &str) -> Result<Self, Error> {
                let number = text.strip_prefix($prefix).ok_or(Error::$error)?;
                let id = Self::new(number.parse().map_err(|_| Error::$error)?)?;
                if id.to_string() != text { return Err(Error::$error); }
                Ok(id)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                write!(f, "{}{:0width$}", $prefix, self.get(), width = $padding)
            }
        }
        text_value!($name, stringify!($name), format!("^{}{}$", $prefix, crate::schema_pattern::padded_decimal(u64::MAX, $padding)), concat!($($doc),*));
    };
}
// Each level has its own checked value, even where the grammar is the same.
numeric_id!(/// A dbSNP site identifier, never an allele join key.
    RsId, RsIdInvalid, "rs", 0);
numeric_id!(/// A ClinGen canonical nucleotide allele identifier.
    CaId, CaIdInvalid, "CA", 6);
// Registry API documents separate CA nucleotide and PA protein namespaces:
// https://reg.genome.network/doc/AlleleRegistry_1.01.xx_api_v1.pdf
numeric_id!(/// A ClinGen canonical protein allele identifier.
    PaId, PaIdInvalid, "PA", 0);
numeric_id!(/// A ClinVar variation record, which may represent a compound.
    ClinVarVariationId, ClinVarVariationIdInvalid, "clinvar:", 0);
numeric_id!(/// An allele identifier inside a ClinVar record, not a variant identifier.
    ClinVarAlleleId, ClinVarAlleleIdInvalid, "", 0);
macro_rules! accession_id {
    ($(#[doc = $doc:literal])* $name:ident, $error:ident, $prefix:literal) => {
        $(#[doc = $doc])*
        #[derive(Clone, Debug)]
        pub struct $name(Box<str>);
        impl $name {
            /// Borrow the complete source accession, including its optional version.
            pub fn as_str(&self) -> &str { &self.0 }
            /// Borrow the record identity without its optional version.
            pub fn accession(&self) -> &str { self.0.split_once('.').map_or(&self.0, |(id, _)| id) }
        }
        impl PartialEq for $name {
            fn eq(&self, other: &Self) -> bool { self.accession() == other.accession() }
        }
        impl Eq for $name {}
        impl Hash for $name {
            fn hash<H: Hasher>(&self, state: &mut H) { self.accession().hash(state); }
        }
        impl FromStr for $name {
            type Err = Error;
            fn from_str(text: &str) -> Result<Self, Error> {
                let (accession, version) = text.split_once('.').map_or((text, None), |(a,v)| (a,Some(v)));
                let valid = accession.strip_prefix($prefix).is_some_and(|n| {
                    n.len() == 9 && n.bytes().all(|b| b.is_ascii_digit()) && n != "000000000"
                });
                if !valid || !version.is_none_or(positive) { return Err(Error::$error); }
                Ok(Self(text.into()))
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(&self.0) }
        }
        text_value!($name, stringify!($name), concat!("^", $prefix, "(?!000000000)[0-9]{9}(\\.[1-9][0-9]*)?$"), concat!($($doc),*));
    };
}
accession_id!(/// A ClinVar variation accession, possibly a haplotype or compound.
    VcvId, VcvIdInvalid, "VCV");
accession_id!(/// A ClinVar variant-and-condition aggregate accession.
    RcvId, RcvIdInvalid, "RCV");
accession_id!(/// A ClinVar submission accession.
    ScvId, ScvIdInvalid, "SCV");
fn positive(text: &str) -> bool {
    !text.is_empty() && !text.starts_with('0') && text.bytes().all(|b| b.is_ascii_digit())
}
/// A COSMIC genomic, legacy transcript or noncoding variant identifier; its kind is retained.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct CosmicId(Box<str>);
impl FromStr for CosmicId {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        if !["COSV", "COSM", "COSN"]
            .iter()
            .any(|p| text.strip_prefix(p).is_some_and(positive))
        {
            return Err(Error::CosmicIdInvalid);
        }
        Ok(Self(text.into()))
    }
}
impl fmt::Display for CosmicId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    CosmicId,
    "CosmicId",
    "^COS[VMN][1-9][0-9]*$",
    "A COSMIC genomic, legacy transcript or noncoding variant identifier; its kind is retained."
);
/// A source-supplied VRS allele identifier; construction never computes a digest.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct VrsId(Box<str>);
impl FromStr for VrsId {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        if !text.strip_prefix("ga4gh:VA.").is_some_and(|d| {
            d.len() == 32
                && d.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_'))
        }) {
            return Err(Error::VrsIdInvalid);
        }
        Ok(Self(text.into()))
    }
}
impl fmt::Display for VrsId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    VrsId,
    "VrsId",
    r"^ga4gh:VA\.[A-Za-z0-9_-]{32}$",
    "A source-supplied VRS allele identifier; construction never computes a digest."
);
/// Published variant-level identifiers, with each identifier's level retained.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum VariantId {
    /// dbSNP site, possibly containing multiple alternate alleles.
    Rs(RsId),
    /// Canonical nucleotide allele.
    Ca(CaId),
    /// Canonical protein allele.
    Pa(PaId),
    /// ClinVar variation record, possibly a compound.
    Vcv(VcvId),
    /// Numeric ClinVar variation record identifier.
    ClinVarVariation(ClinVarVariationId),
    /// COSMIC identifier with its stated kind.
    Cosmic(CosmicId),
    /// Source-supplied VRS allele digest.
    Vrs(VrsId),
}
impl FromStr for VariantId {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        if text.starts_with("rs") {
            return text.parse().map(Self::Rs);
        }
        if text.starts_with("CA") {
            return text.parse().map(Self::Ca);
        }
        if text.starts_with("PA") {
            return text.parse().map(Self::Pa);
        }
        if text.starts_with("VCV") {
            return text.parse().map(Self::Vcv);
        }
        if text.starts_with("clinvar:") {
            return text.parse().map(Self::ClinVarVariation);
        }
        if text.starts_with("COS") {
            return text.parse().map(Self::Cosmic);
        }
        if text.starts_with("ga4gh:VA.") {
            return text.parse().map(Self::Vrs);
        }
        Err(Error::VariantIdInvalid)
    }
}
impl fmt::Display for VariantId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rs(v) => v.fmt(f),
            Self::Ca(v) => v.fmt(f),
            Self::Pa(v) => v.fmt(f),
            Self::Vcv(v) => v.fmt(f),
            Self::ClinVarVariation(v) => v.fmt(f),
            Self::Cosmic(v) => v.fmt(f),
            Self::Vrs(v) => v.fmt(f),
        }
    }
}
text_value!(
    VariantId,
    "VariantId",
    format!(
        "({}|{}|{}|{}|{}|{}|{})",
        RsId::pattern(),
        CaId::pattern(),
        PaId::pattern(),
        VcvId::pattern(),
        ClinVarVariationId::pattern(),
        CosmicId::pattern(),
        VrsId::pattern()
    ),
    "Published variant-level identifiers, with each identifier's level retained."
);
