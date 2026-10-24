use crate::{Error, macros::text_value};
use std::{fmt, str::FromStr};
const AMINO: &[(&str, &str)] = &[
    ("A", "Ala"),
    ("R", "Arg"),
    ("N", "Asn"),
    ("D", "Asp"),
    ("C", "Cys"),
    ("E", "Glu"),
    ("Q", "Gln"),
    ("G", "Gly"),
    ("H", "His"),
    ("I", "Ile"),
    ("L", "Leu"),
    ("K", "Lys"),
    ("M", "Met"),
    ("F", "Phe"),
    ("P", "Pro"),
    ("S", "Ser"),
    ("T", "Thr"),
    ("W", "Trp"),
    ("Y", "Tyr"),
    ("V", "Val"),
    ("U", "Sec"),
    ("O", "Pyl"),
    ("*", "Ter"),
];
/// Explicit protein substitution with checked residue codes and a positive position.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ProteinChange {
    from: usize,
    position: u32,
    to: usize,
}
impl FromStr for ProteinChange {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let value = value.strip_prefix("p.").unwrap_or(value);
        let first = value
            .find(|c: char| c.is_ascii_digit())
            .ok_or(Error::ArticleIdInvalid)?;
        let end = value
            .rfind(|c: char| c.is_ascii_digit())
            .ok_or(Error::ArticleIdInvalid)?
            + 1;
        let residue = |v: &str| {
            AMINO
                .iter()
                .position(|(one, three)| v == *one || v == *three)
                .ok_or(Error::ArticleIdInvalid)
        };
        let digits = value.get(first..end).ok_or(Error::ArticleIdInvalid)?;
        if digits.starts_with('0') || !digits.bytes().all(|c| c.is_ascii_digit()) {
            return Err(Error::ArticleIdInvalid);
        }
        let position = digits.parse().map_err(|_| Error::ArticleIdInvalid)?;
        Ok(Self {
            from: residue(value.get(..first).ok_or(Error::ArticleIdInvalid)?)?,
            position,
            to: residue(value.get(end..).ok_or(Error::ArticleIdInvalid)?)?,
        })
    }
}
impl ProteinChange {
    /// Literal spellings searched by readers, in stable order.
    pub fn forms(&self) -> [String; 4] {
        let (from, from_three) = AMINO.get(self.from).copied().unwrap_or(("", ""));
        let (to, to_three) = AMINO.get(self.to).copied().unwrap_or(("", ""));
        let one = format!("{from}{}{to}", self.position);
        let three = format!("{from_three}{}{to_three}", self.position);
        [
            one.clone(),
            three.clone(),
            format!("p.{one}"),
            format!("p.{three}"),
        ]
    }
}
impl fmt::Display for ProteinChange {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.forms().first().map(String::as_str).unwrap_or(""))
    }
}
text_value!(
    ProteinChange,
    "ProteinChange",
    r"^(p\.)?([A-Z*]|[A-Z][a-z]{2})[1-9][0-9]*([A-Z*]|[A-Z][a-z]{2})$"
);
