use std::{fmt, str::FromStr};
/// An author identifier failed its source syntax or checksum.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AuthorIdInvalid {
    /// Invalid source identifier or qualification.
    Syntax,
    /// ORCID checksum mismatch.
    Checksum,
}
impl fmt::Display for AuthorIdInvalid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Syntax => "author.id.syntax",
            Self::Checksum => "author.id.checksum",
        })
    }
}
impl std::error::Error for AuthorIdInvalid {}
/// A checked source-local Semantic Scholar author identifier, never a person identity.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct SemanticScholarAuthorId(Box<str>);
impl FromStr for SemanticScholarAuthorId {
    type Err = AuthorIdInvalid;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if !v.starts_with('0') && !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()) {
            Ok(Self(v.into()))
        } else {
            Err(AuthorIdInvalid::Syntax)
        }
    }
}
impl fmt::Display for SemanticScholarAuthorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
crate::text_value!(
    SemanticScholarAuthorId,
    "SemanticScholarAuthorId",
    "^[1-9][0-9]*$",
    "A checked source-local Semantic Scholar author identifier, never a person identity."
);
/// A checksummed ORCID iD, identifying only the requested ORCID record.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrcidId(Box<str>);
impl FromStr for OrcidId {
    type Err = AuthorIdInvalid;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if v.len() != 19 {
            return Err(AuthorIdInvalid::Syntax);
        }
        let mut total = 0u32;
        for (i, b) in v.bytes().enumerate() {
            if [4, 9, 14].contains(&i) {
                if b != b'-' {
                    return Err(AuthorIdInvalid::Syntax);
                }
            } else if i == 18 {
                let check = (12 - total % 11) % 11;
                let expected = if check == 10 {
                    b'X'
                } else {
                    b'0' + check as u8
                };
                if b != expected {
                    return Err(AuthorIdInvalid::Checksum);
                }
            } else if b.is_ascii_digit() {
                total = (total + u32::from(b - b'0')) * 2;
            } else {
                return Err(AuthorIdInvalid::Syntax);
            }
        }
        Ok(Self(v.into()))
    }
}
impl fmt::Display for OrcidId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
crate::text_value!(
    OrcidId,
    "OrcidId",
    "^[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{3}[0-9X]$",
    "A checksummed ORCID iD, identifying only the requested ORCID record."
);
/// One provider record; this enum establishes no relationship between its variants.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum AuthorId {
    /// One Semantic Scholar record.
    SemanticScholar(SemanticScholarAuthorId),
    /// One ORCID record.
    Orcid(OrcidId),
}
impl FromStr for AuthorId {
    type Err = AuthorIdInvalid;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        match v.split_once(':') {
            Some(("semantic-scholar" | "semanticscholar", id)) => {
                Ok(Self::SemanticScholar(id.parse()?))
            }
            Some(("orcid", id)) => Ok(Self::Orcid(id.parse()?)),
            _ => Err(AuthorIdInvalid::Syntax),
        }
    }
}
impl fmt::Display for AuthorId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SemanticScholar(id) => write!(f, "semantic-scholar:{id}"),
            Self::Orcid(id) => write!(f, "orcid:{id}"),
        }
    }
}
crate::text_value!(
    AuthorId,
    "AuthorId",
    "^((semantic-scholar|semanticscholar):[1-9][0-9]*|orcid:[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{3}[0-9X])$",
    "One provider record; this enum establishes no relationship between its variants."
);

/// A checked native ORCID client application identifier.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct OrcidClientId(Box<str>);
impl FromStr for OrcidClientId {
    type Err = AuthorIdInvalid;
    fn from_str(v: &str) -> Result<Self, Self::Err> {
        if v.parse::<OrcidId>().is_ok()
            || v.strip_prefix("APP-").is_some_and(|s| {
                s.len() == 16
                    && s.bytes()
                        .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit())
            })
        {
            Ok(Self(v.into()))
        } else {
            Err(AuthorIdInvalid::Syntax)
        }
    }
}
impl fmt::Display for OrcidClientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
crate::text_value!(
    OrcidClientId,
    "OrcidClientId",
    "^(APP-[A-Z0-9]{16}|[0-9]{4}-[0-9]{4}-[0-9]{4}-[0-9]{3}[0-9X])$",
    "A checked native ORCID client application identifier."
);
