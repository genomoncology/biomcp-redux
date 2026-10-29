use crate::Curie;
use std::{fmt, str::FromStr};
/// A cell identity failed its source spelling contract.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CellLineIdInvalid;
impl fmt::Display for CellLineIdInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("cell_line.id.invalid")
    }
}
impl std::error::Error for CellLineIdInvalid {}
macro_rules! checked {
    ($name:ident, $check:expr, $pattern:expr, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Box<str>);
        impl $name {
            /// Check the complete spelling without normalization.
            pub fn new(value: &str) -> Result<Self, CellLineIdInvalid> {
                if ($check)(value) {
                    Ok(Self(value.into()))
                } else {
                    Err(CellLineIdInvalid)
                }
            }
            /// Borrow the unchanged source identifier.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl FromStr for $name {
            type Err = CellLineIdInvalid;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                Self::new(value)
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
        crate::text_value!($name, stringify!($name), $pattern, $doc);
    };
}
checked!(
    CellosaurusAccession,
    |v: &str| v.strip_prefix("CVCL_").is_some_and(|s| s.len() == 4
        && s.bytes()
            .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())),
    "^CVCL_[A-Z0-9]{4}$",
    "A checked Cellosaurus accession; primary identity is verified by the adapter."
);
checked!(
    CellLineSourceId,
    |v: &str| Curie::qualified_parts(v).is_some(),
    format!(
        r"^[A-Za-z0-9_.-]+:[^{}\x00-\x1f\x7f-\x9f]+$",
        crate::schema_pattern::WHITESPACE
    ),
    "An exact database-qualified source ID, preserving the database and native accession independently."
);
impl CellLineSourceId {
    /// Source database spelling before the first colon.
    pub fn database(&self) -> &str {
        self.as_str().split_once(':').map_or("", |v| v.0)
    }
    /// Native accession, which can itself contain a colon.
    pub fn accession(&self) -> &str {
        self.as_str().split_once(':').map_or("", |v| v.1)
    }
}
checked!(
    ChemblCellId,
    |v: &str| v
        .strip_prefix("CHEMBL")
        .is_some_and(|s| !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit())),
    "^CHEMBL[0-9]+$",
    "A ChEMBL source-local cell ID, never a compound or target identity."
);
