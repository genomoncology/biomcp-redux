use std::{fmt, str::FromStr};
/// A ClinPGx object accession, distinct from the protein allele namespace PaId.
/// The namespaces overlap in spelling; a source-given ClinPGx object is never converted to a protein allele.
/// ```compile_fail
/// let accession: biodata::ClinPgxAccession = "PA166134421".parse().unwrap();
/// let allele: biodata::PaId = accession;
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClinPgxAccession(Box<str>);
impl ClinPgxAccession {
    /// Exact source accession.
    pub fn as_str(&self) -> &str { &self.0 }
}
impl FromStr for ClinPgxAccession {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let digits = value.strip_prefix("PA").ok_or(crate::Error::ClinPgxAccession)?;
        if digits.is_empty() || digits.starts_with('0') || !digits.bytes().all(|b| b.is_ascii_digit()) {
            return Err(crate::Error::ClinPgxAccession);
        }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for ClinPgxAccession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.write_str(self.as_str()) }
}
crate::text_value!(ClinPgxAccession, "ClinPgxAccession", r"^PA[1-9][0-9]*$");

/// Exact JSON gene key; ordering preserves spelling and never merges differently cased source keys.
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PgxGeneKey(Box<str>);
impl FromStr for PgxGeneKey {
    type Err=crate::Error;
    fn from_str(value:&str)->Result<Self,Self::Err> {
        crate::GeneSymbol::new(value)?;
        Ok(Self(value.into()))
    }
}
impl PgxGeneKey {
    /// Exact original source spelling.
    pub fn as_str(&self)->&str { &self.0 }
}
impl fmt::Display for PgxGeneKey {
    fn fmt(&self,f:&mut fmt::Formatter<'_>)->fmt::Result {f.write_str(self.as_str())}
}
crate::text_value!(PgxGeneKey,"PgxGeneKey",r"^[A-Za-z][A-Za-z0-9._/@-]{0,254}$");
