use std::{fmt, str::FromStr};

fn digits(value: &str, width: usize) -> bool {
    value.len() == width && value.bytes().all(|b| b.is_ascii_digit())
}
fn gtr(value: &str) -> bool {
    value
        .strip_prefix("GTR")
        .and_then(|v| v.split_once('.'))
        .is_some_and(|(id, version)| {
            digits(id, 9)
                && !version.starts_with('0')
                && !version.is_empty()
                && version.bytes().all(|b| b.is_ascii_digit())
        })
}
fn who(value: &str) -> bool {
    let mut parts = value.split('-');
    parts.next().is_some_and(|v| digits(v, 4) || digits(v, 5))
        && parts.next().is_some_and(|v| digits(v, 3) || digits(v, 5))
        && parts.next().is_some_and(|v| digits(v, 2))
        && parts.next().is_none()
}
fn pma(value: &str) -> bool {
    value.strip_prefix('P').is_some_and(|v| digits(v, 6))
}
fn supplement(value: &str) -> bool {
    value
        .strip_prefix('S')
        .is_some_and(|v| digits(v, 3) && v != "000")
}
fn native_supplement(value: &str) -> bool {
    value.is_empty() || supplement(value)
}
fn drug(value: &str) -> bool {
    value
        .strip_prefix("NDA")
        .or_else(|| value.strip_prefix("BLA"))
        .is_some_and(|v| digits(v, 6))
}
fn device(value: &str) -> bool {
    if let Some((base, sub)) = value.split_once('/') {
        return pma(base) && supplement(sub);
    }
    pma(value)
        || value
            .strip_prefix('K')
            .or_else(|| value.strip_prefix("DEN"))
            .or_else(|| value.strip_prefix('H'))
            .is_some_and(|v| digits(v, 6))
}
macro_rules! id {
    ($name:ident, $check:ident, $pattern:literal, $doc:literal) => {
        #[doc = $doc]
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        pub struct $name(Box<str>);
        impl FromStr for $name {
            type Err = crate::Error;
            fn from_str(value: &str) -> Result<Self, Self::Err> {
                if $check(value) {
                    Ok(Self(value.into()))
                } else {
                    Err(crate::Error::DiagnosticIdInvalid)
                }
            }
        }
        impl $name {
            /// Borrow the complete published identity.
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
id!(
    GtrTestId,
    gtr,
    "^GTR[0-9]{9}\\.[1-9][0-9]*$",
    "Complete native GTR accession and positive version."
);
id!(
    WhoProductId,
    who,
    "^[0-9]{4,5}-([0-9]{3}|[0-9]{5})-[0-9]{2}$",
    "WHO product listing identity, independent of product codes."
);
id!(
    PmaNumber,
    pma,
    "^P[0-9]{6}$",
    "Native FDA PMA base application number."
);
id!(
    PmaSupplement,
    supplement,
    "^S(00[1-9]|0[1-9][0-9]|[1-9][0-9]{2})$",
    "Explicit native PMA supplement selector; no invented base alias."
);
id!(
    FdaDrugApplication,
    drug,
    "^(NDA|BLA)[0-9]{6}$",
    "Cell-owned drug application preserving native NDA/BLA prefix and leading zeros."
);
id!(
    DiagnosticDeviceId,
    device,
    "^(P[0-9]{6}(/S(00[1-9]|0[1-9][0-9]|[1-9][0-9]{2}))?|K[0-9]{6}|DEN[0-9]{6}|H[0-9]{6})$",
    "Source device identifier; DEN and H identifiers imply no executable get."
);

id!(
    PmaSupplementCell,
    native_supplement,
    "^($|S(00[1-9]|0[1-9][0-9]|[1-9][0-9]{2}))$",
    "Native PMA supplement cell; empty denotes base and never becomes S000."
);
