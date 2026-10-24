use crate::{Error, macros::text_value};
use std::{fmt, str::FromStr};
/// One nucleotide: the four DNA bases and the IUPAC ambiguity codes.
/// It renders as its upper-case letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
#[allow(missing_docs, reason = "each variant is its IUPAC letter")]
pub enum Base {
    A = b'A',
    C = b'C',
    G = b'G',
    T = b'T',
    N = b'N',
    R = b'R',
    Y = b'Y',
    S = b'S',
    W = b'W',
    K = b'K',
    M = b'M',
    B = b'B',
    D = b'D',
    H = b'H',
    V = b'V',
}

impl Base {
    pub(crate) fn from_letter(letter: char) -> Option<Self> {
        use Base::*;
        Some(match letter {
            'A' => A,
            'C' => C,
            'G' => G,
            'T' => T,
            'N' => N,
            'R' => R,
            'Y' => Y,
            'S' => S,
            'W' => W,
            'K' => K,
            'M' => M,
            'B' => B,
            'D' => D,
            'H' => H,
            'V' => V,
            _ => return None,
        })
    }
}

impl fmt::Display for Base {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", char::from(*self as u8))
    }
}

impl FromStr for Base {
    type Err = Error;
    fn from_str(value: &str) -> Result<Self, Error> {
        let mut letters = value.chars();
        let base = letters
            .next()
            .and_then(Self::from_letter)
            .ok_or(Error::SequenceInvalid)?;
        if letters.next().is_some() {
            return Err(Error::SequenceInvalid);
        }
        Ok(base)
    }
}
text_value!(Base, "Base", "^[ACGTNRY SWKMBDHV]$".replace(' ', ""));
/// A literal IUPAC DNA sequence; empty sequences express insertions or deletions.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sequence(Box<[Base]>);
impl Sequence {
    /// Preserve all source bases without normalization or case conversion.
    pub fn new(bases: impl Into<Box<[Base]>>) -> Self {
        Self(bases.into())
    }
    /// Source bases in order.
    pub fn as_slice(&self) -> &[Base] {
        &self.0
    }
    /// Number of bases, including zero.
    pub fn len(&self) -> usize {
        self.0.len()
    }
    /// Whether this side of an allele is empty.
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}
impl FromStr for Sequence {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        text.chars()
            .map(|c| Base::from_letter(c).ok_or(Error::SequenceInvalid))
            .collect::<Result<Vec<_>, _>>()
            .map(|v| Self(v.into()))
    }
}
impl fmt::Display for Sequence {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for b in &self.0 {
            b.fmt(f)?;
        }
        Ok(())
    }
}
text_value!(Sequence, "Sequence", "^[ACGTNRYSWKMBDHV]*$");
