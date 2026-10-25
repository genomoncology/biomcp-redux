use std::{fmt, str::FromStr};
/// A ClinPGx object accession, distinct from the protein allele namespace PaId.
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
