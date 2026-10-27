use crate::{Error, UniProtAcc};
use std::{fmt, str::FromStr};
fn isoform(value: &str) -> bool {
    let (base, suffix) = value
        .split_once('-')
        .map_or((value, None), |(a, b)| (a, Some(b)));
    UniProtAcc::new(base).is_ok()
        && suffix.is_none_or(|v| {
            !v.is_empty() && !v.starts_with('0') && v.bytes().all(|b| b.is_ascii_digit())
        })
}
fn interpro(value: &str) -> bool {
    value
        .strip_prefix("IPR")
        .is_some_and(|v| v.len() == 6 && v.bytes().all(|b| b.is_ascii_digit()))
}
fn complex(value: &str) -> bool {
    value.strip_prefix("CPX-").is_some_and(|v| {
        !v.is_empty() && !v.starts_with('0') && v.bytes().all(|b| b.is_ascii_digit())
    })
}
fn residues(value: &str) -> bool {
    !value.is_empty()
        && value
            .bytes()
            .all(|b| b"ACDEFGHIKLMNPQRSTVWYUOX".contains(&b))
}
macro_rules! checked {
    ($name:ident, $error:ident, $check:ident, $pattern:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Box<str>);
        impl $name {
            /// Check the entire source spelling without adding information.
            pub fn new(value: &str) -> Result<Self, Error> {
                if $check(value) {
                    Ok(Self(value.into()))
                } else {
                    Err(Error::$error)
                }
            }
            /// Borrow the stated source spelling.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl FromStr for $name {
            type Err = Error;
            fn from_str(v: &str) -> Result<Self, Error> {
                Self::new(v)
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
    UniProtIsoform,
    UniProtIsoformInvalid,
    isoform,
    "^([OPQ][0-9][A-Z0-9]{3}[0-9]|[A-NR-Z][0-9]([A-Z][A-Z0-9]{2}[0-9]){1,2})(-[1-9][0-9]*)?$",
    "A stated UniProt accession with an optional positive isoform number; never imputes -1."
);
checked!(
    AminoAcids,
    ProteinResidueInvalid,
    residues,
    "^[ACDEFGHIKLMNPQRSTVWYUOX]+$",
    "Nonempty uppercase standard residues plus U, O and X."
);
checked!(
    InterProId,
    InterProIdInvalid,
    interpro,
    "^IPR[0-9]{6}$",
    "An integrated InterPro entry accession, distinct from a member signature."
);
checked!(
    ComplexPortalId,
    ComplexPortalIdInvalid,
    complex,
    "^CPX-[1-9][0-9]*$",
    "An exact Complex Portal accession."
);
