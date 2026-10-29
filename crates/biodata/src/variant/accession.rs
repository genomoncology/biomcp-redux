//! Accession grammar and display copied from the owning value module; no molecular parser.
use crate::{Error, text_value};
use std::{fmt, str::FromStr};
use winnow::{
    ModalResult, Parser,
    ascii::digit1,
    combinator::{alt, cut_err, opt, preceded},
    error::{ContextError, ErrMode},
};
/// A reference sequence accession: RefSeq or Ensembl with its version, or LRG.
/// Values enter through the checked parser; callers cannot construct unchecked fields.
/// ```compile_fail
/// let invalid = biodata::Accession::RefSeq {
///     prefix: biodata::RefSeqPrefix::Nm, number: 5228, version: 0,
/// };
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Accession(pub(crate) AccessionKind);

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum AccessionKind {
    /// A RefSeq accession such as `NC_000007.14`.
    RefSeq {
        /// The two letters and underscore.
        prefix: RefSeqPrefix,
        /// The number, written in six digits, or nine from 1000000 up.
        number: u32,
        /// The version after the dot.
        version: u16,
    },
    /// A human Ensembl accession such as `ENST00000288602.11`.
    Ensembl {
        /// The letter after `ENS`.
        feature: EnsemblFeature,
        /// The number, written in eleven digits.
        number: u64,
        /// The version after the dot.
        version: u16,
    },
    /// A Locus Reference Genomic record such as `LRG_199`, `LRG_199t1` or `LRG_199p1`.
    Lrg {
        /// The record number.
        number: u32,
        /// A transcript or protein of the record.
        part: Option<LrgPart>,
    },
}

crate::canonical_enum! {
    /// The RefSeq prefixes a variant description uses.
    RefSeqPrefix, AccessionInvalid {
    /// Published prefix.
    Nc => "NC_",
    /// Published prefix.
    Ng => "NG_",
    /// Published prefix.
    Nm => "NM_",
    /// Published prefix.
    Np => "NP_",
    /// Published prefix.
    Nr => "NR_",
    /// Published prefix.
    Nt => "NT_",
    /// Published prefix.
    Nw => "NW_",
    /// Published prefix.
    Xm => "XM_",
    /// Published prefix.
    Xp => "XP_",
    /// Published prefix.
    Xr => "XR_",
}}

crate::canonical_enum! {
    /// What an Ensembl accession names.
    EnsemblFeature, AccessionInvalid {
    /// Published prefix.
    Gene => "ENSG",
    /// Published prefix.
    Transcript => "ENST",
    /// Published prefix.
    Protein => "ENSP",
}}

/// A transcript or protein of an LRG record.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum LrgPart {
    /// `t1`
    Transcript(u16),
    /// `p1`
    Protein(u16),
}

/// An accession. Its prefix commits to the rest.
fn accession(input: &mut &str) -> ModalResult<Accession> {
    alt((refseq, ensembl, lrg)).parse_next(input)
}

fn refseq(input: &mut &str) -> ModalResult<Accession> {
    let prefix = RefSeqPrefix::parse(input)?;
    let (number, version) = cut_err((digits(refseq_width), version)).parse_next(input)?;
    Ok(Accession(AccessionKind::RefSeq {
        prefix,
        number,
        version,
    }))
}

fn ensembl(input: &mut &str) -> ModalResult<Accession> {
    let feature = EnsemblFeature::parse(input)?;
    let (number, version) = cut_err((digits(|_| 11), version)).parse_next(input)?;
    Ok(Accession(AccessionKind::Ensembl {
        feature,
        number,
        version,
    }))
}

fn lrg(input: &mut &str) -> ModalResult<Accession> {
    let number = preceded("LRG_", cut_err(decimal)).parse_next(input)?;
    let part = opt(alt((
        preceded('t', cut_err(decimal)).map(LrgPart::Transcript),
        preceded('p', cut_err(decimal)).map(LrgPart::Protein),
    )))
    .parse_next(input)?;
    Ok(Accession(AccessionKind::Lrg { number, part }))
}

fn version(input: &mut &str) -> ModalResult<u16> {
    preceded(
        '.',
        digit1.verify_map(|text: &str| {
            (!text.starts_with('0'))
                .then(|| text.parse().ok())
                .flatten()
        }),
    )
    .parse_next(input)
}

/// Digits read as a number, written in exactly the width the number renders at.
fn digits<'a, T: FromStr + Copy>(
    width: impl Fn(T) -> usize,
) -> impl Parser<&'a str, T, ErrMode<ContextError>> {
    digit1.verify_map(move |text: &str| text.parse().ok().filter(|n| text.len() == width(*n)))
}

fn refseq_width(number: u32) -> usize {
    if number < 1_000_000 { 6 } else { 9 }
}

impl fmt::Display for Accession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            AccessionKind::RefSeq {
                prefix,
                number,
                version,
            } => {
                let width = refseq_width(number);
                write!(f, "{prefix}{number:0width$}.{version}")
            }
            AccessionKind::Ensembl {
                feature,
                number,
                version,
            } => {
                write!(f, "{feature}{number:011}.{version}")
            }
            AccessionKind::Lrg { number, part } => {
                write!(f, "LRG_{number}")?;
                match part {
                    Some(LrgPart::Transcript(n)) => write!(f, "t{n}"),
                    Some(LrgPart::Protein(n)) => write!(f, "p{n}"),
                    None => Ok(()),
                }
            }
        }
    }
}

impl RefSeqPrefix {
    fn parse(input: &mut &str) -> ModalResult<Self> {
        winnow::token::take(3usize)
            .try_map(str::parse)
            .parse_next(input)
    }
}
impl EnsemblFeature {
    fn parse(input: &mut &str) -> ModalResult<Self> {
        winnow::token::take(4usize)
            .try_map(str::parse)
            .parse_next(input)
    }
}
fn decimal<T: FromStr>(input: &mut &str) -> ModalResult<T> {
    digit1
        .verify_map(|text: &str| text.parse().ok())
        .parse_next(input)
}
impl FromStr for Accession {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        if text.starts_with("LRG_") {
            return accession
                .parse(text)
                .map_err(|_| Error::AccessionLrgInvalid);
        }
        let prefix_len = if text
            .get(..3)
            .is_some_and(|p| p.parse::<RefSeqPrefix>().is_ok())
        {
            3
        } else if text
            .get(..4)
            .is_some_and(|p| p.parse::<EnsemblFeature>().is_ok())
        {
            4
        } else {
            return Err(Error::AccessionInvalid);
        };
        let tail = text.get(prefix_len..).ok_or(Error::AccessionInvalid)?;
        let (number, version) = tail.split_once('.').ok_or(Error::AccessionVersionMissing)?;
        let width_ok = if prefix_len == 3 {
            matches!(number.len(), 6 | 9)
        } else {
            number.len() == 11
        };
        if !width_ok || !number.bytes().all(|c| c.is_ascii_digit()) {
            return Err(Error::AccessionDigitsWidth);
        }
        if version.starts_with('0') {
            return Err(Error::AccessionVersionLeadingZero);
        }
        if version.is_empty() {
            return Err(Error::AccessionVersionMissing);
        }
        if !version.bytes().all(|c| c.is_ascii_digit()) || version.parse::<u16>().is_err() {
            return Err(Error::AccessionVersionOverflow);
        }
        accession
            .parse(text)
            .map_err(|_| Error::AccessionDigitsWidth)
    }
}
text_value!(
    Accession,
    "Accession",
    format!("^{}$", accession_pattern(true)),
    "A reference sequence accession: RefSeq or Ensembl with its version, or LRG."
);

/// An accession whose source states no version; never used for an allele location.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct AccessionBase(Box<str>);
impl AccessionBase {
    /// Borrow the canonical unversioned accession.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}
impl FromStr for AccessionBase {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        if text.contains('.') {
            return Err(Error::AccessionVersionPresent);
        }
        if text.starts_with("LRG_") {
            text.parse::<Accession>()?;
        } else {
            format!("{text}.1").parse::<Accession>()?;
        }
        Ok(Self(text.into()))
    }
}
impl fmt::Display for AccessionBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.0)
    }
}
text_value!(
    AccessionBase,
    "AccessionBase",
    format!("^{}$", accession_pattern(false)),
    "An accession whose source states no version; never used for an allele location."
);

#[cfg(feature = "schema")]
fn accession_pattern(versioned: bool) -> String {
    let version = if versioned {
        format!(r"\.{}", crate::schema_pattern::decimal(u16::MAX.into()))
    } else {
        String::new()
    };
    let number = crate::schema_pattern::source_decimal(u32::MAX.into());
    let part = crate::schema_pattern::source_decimal(u16::MAX.into());
    format!(
        r"(?:(?:N[CGMPRTW]|X[MPR])_(?:[0-9]{{6}}|(?!000)[0-9]{{9}}){version}|ENS[GTP][0-9]{{11}}{version}|LRG_{number}(?:[tp]{part})?)"
    )
}
