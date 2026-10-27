use std::{fmt, str::FromStr};

fn native(value: &str) -> bool {
    value
        .bytes()
        .next()
        .is_some_and(|b| b.is_ascii_alphanumeric())
        && value
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || matches!(b, b'_' | b'-' | b'.'))
}
fn blob(value: &str) -> bool {
    value.len() == 40
        && value
            .bytes()
            .all(|b| b.is_ascii_digit() || matches!(b, b'a'..=b'f'))
}
fn import_time(value: &str) -> bool {
    let Some((date, time)) = value.split_once(' ') else {
        return false;
    };
    let parts: Vec<_> = time.split(':').collect();
    date.len() == 10
        && date.parse::<crate::PartialDate>().is_ok()
        && parts.len() == 3
        && parts.iter().zip([23, 59, 59]).all(|(v, max)| {
            v.len() == 2
                && v.bytes().all(|b| b.is_ascii_digit())
                && v.parse::<u8>().is_ok_and(|v| v <= max)
        })
}
macro_rules! checked {
    ($name:ident,$check:ident,$pattern:literal,$doc:literal) => {
        #[doc=$doc]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Box<str>);
        impl FromStr for $name {
            type Err = crate::Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if $check(value) {
                    Ok(Self(value.into()))
                } else {
                    Err(crate::Error::StudyValueInvalid)
                }
            }
        }
        impl $name {
            /// Borrow the exact source spelling.
            pub fn as_str(&self) -> &str {
                &self.0
            }
        }
        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str(&self.0)
            }
        }
        crate::text_value!($name, stringify!($name), $pattern, $doc);
    };
}
checked!(
    StudyId,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Opaque case-sensitive cBioPortal study identity, without repair."
);
checked!(
    StudyProfileId,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Exact cBioPortal molecular profile identity."
);
checked!(
    StudySampleListId,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Exact cBioPortal sample-list identity."
);
checked!(
    CbioportalCancerTypeId,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Native cBioPortal cancer-type code, with no OncoTree equivalence."
);
checked!(
    StudyResourceId,
    native,
    "^[A-Za-z0-9][A-Za-z0-9_.-]*$",
    "Exact cBioPortal resource identity scoped to one study."
);
checked!(
    StudyBlobSha,
    blob,
    "^[0-9a-f]{40}$",
    "GitHub repository blob revision, not a data-content checksum."
);
checked!(
    StudyImportTime,
    import_time,
    "^[0-9]{4}-[0-9]{2}-[0-9]{2} ([01][0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$",
    "Native study import timestamp with unstated timezone; never an acquisition time."
);
