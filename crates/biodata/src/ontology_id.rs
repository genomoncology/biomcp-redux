//! Publisher ontology identifiers preserve their exact source spelling.
use crate::Error;
use std::{fmt, str::FromStr};

macro_rules! ontology_id {
    ($name:ident, $prefix:literal, $width:expr, $pattern:literal, $description:literal) => {
        #[doc = $description]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Box<str>);
        impl FromStr for $name {
            type Err = Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                let digits = value.strip_prefix($prefix).ok_or(Error::CurieInvalid)?;
                if digits.is_empty()
                    || !digits.bytes().all(|v| v.is_ascii_digit())
                    || $width.is_some_and(|width| digits.len() != width)
                {
                    return Err(Error::CurieInvalid);
                }
                Ok(Self(value.into()))
            }
        }
        impl $name {
            /// Borrow the unchanged publisher identifier.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(self.as_str())
            }
        }
        crate::text_value!($name, stringify!($name), $pattern, $description);
    };
}
// https://github.com/monarch-initiative/mondo/blob/master/src/ontology/mondo-idranges.owl
ontology_id!(
    MondoId,
    "MONDO:",
    Some(7),
    "^MONDO:[0-9]{7}$",
    "A publisher MONDO identifier with seven digits."
);
// https://evs.nci.nih.gov/ftp1/GDC/About.html
ontology_id!(
    NcitId,
    "C",
    None::<usize>,
    "^C[0-9]+$",
    "A native NCI Thesaurus concept code."
);
