use crate::{Error, text_value};
use std::{fmt, str::FromStr};
/// One nucleotide: the four DNA bases and the IUPAC ambiguity codes.
/// It renders as its upper-case letter.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Base {
    /// IUPAC code A.
    A = b'A',
    /// IUPAC code C.
    C = b'C',
    /// IUPAC code G.
    G = b'G',
    /// IUPAC code T.
    T = b'T',
    /// IUPAC code N.
    N = b'N',
    /// IUPAC code R.
    R = b'R',
    /// IUPAC code Y.
    Y = b'Y',
    /// IUPAC code S.
    S = b'S',
    /// IUPAC code W.
    W = b'W',
    /// IUPAC code K.
    K = b'K',
    /// IUPAC code M.
    M = b'M',
    /// IUPAC code B.
    B = b'B',
    /// IUPAC code D.
    D = b'D',
    /// IUPAC code H.
    H = b'H',
    /// IUPAC code V.
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
        if value == "U" {
            return Err(Error::SequenceRna);
        }
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
text_value!(
    Base,
    "Base",
    "^[ACGTNRYSWKMBDHV]$",
    "One nucleotide: the four DNA bases and the IUPAC ambiguity codes. It renders as its upper-case letter."
);
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
            .map(|c| {
                Base::from_letter(c).ok_or(if c == 'U' {
                    Error::SequenceRna
                } else {
                    Error::SequenceInvalid
                })
            })
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
text_value!(
    Sequence,
    "Sequence",
    "^[ACGTNRYSWKMBDHV]*$",
    "A literal IUPAC DNA sequence; empty sequences express insertions or deletions."
);
