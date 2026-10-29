//! Primary chromosome and assembly accessions copied from the owning reference module.
use crate::{Accession, Build, Error, RefSeqPrefix, text_value};
use std::{fmt, str::FromStr};
const VERSIONS: [[u16; 24]; 2] = [
    [
        10, 11, 11, 11, 9, 11, 13, 10, 11, 10, 9, 11, 10, 8, 9, 9, 10, 9, 9, 10, 8, 10, 10, 9,
    ],
    [
        11, 12, 12, 12, 10, 12, 14, 11, 12, 11, 10, 12, 11, 9, 10, 10, 11, 10, 10, 11, 9, 11, 11,
        10,
    ],
];
/// A primary chromosome in the caller's reference build.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Chromosome(u8);
impl Chromosome {
    /// Construct a primary chromosome from its numeric identifier (1–25).
    pub fn new(number: u8) -> Option<Self> {
        (1..=25).contains(&number).then_some(Self(number))
    }

    /// Return its chromosome accession in this build.
    pub fn accession(self, build: Build) -> Accession {
        let (number, version) = if self.0 == 25 {
            (12920, 1)
        } else {
            let row = if build == Build::GRCh37 {
                VERSIONS.first()
            } else {
                VERSIONS.last()
            };
            (
                u32::from(self.0),
                row.and_then(|v| v.get(usize::from(self.0 - 1)))
                    .copied()
                    .unwrap_or(0),
            )
        };
        Accession(crate::variant::AccessionKind::RefSeq {
            prefix: RefSeqPrefix::Nc,
            number,
            version,
        })
    }
    /// Chromosome name without the optional `chr` prefix.
    pub fn name(self) -> String {
        match self.0 {
            23 => "X".into(),
            24 => "Y".into(),
            25 => "MT".into(),
            n => n.to_string(),
        }
    }
    /// Read only a canonical primary chromosome spelling.
    pub fn parse(text: &str) -> Result<Self, Error> {
        let number = match text {
            "X" => 23,
            "Y" => 24,
            "MT" => 25,
            _ => text.parse().map_err(|_| Error::ChromosomeInvalid)?,
        };
        let value = Self::new(number).ok_or(Error::ChromosomeInvalid)?;
        if value.name() == text {
            Ok(value)
        } else {
            Err(Error::ChromosomeInvalid)
        }
    }
    /// Numeric primary chromosome identifier (1–25).
    pub const fn number(self) -> u8 {
        self.0
    }
    /// Resolve an exact primary chromosome accession on the selected build.
    pub fn from_accession(ac: Accession, build: Build) -> Result<Self, Error> {
        (1..=25)
            .map(Self)
            .find(|c| c.accession(build) == ac)
            .ok_or(Error::ChromosomeInvalid)
    }
}

impl FromStr for Chromosome {
    type Err = Error;
    fn from_str(text: &str) -> Result<Self, Error> {
        Self::parse(text)
    }
}
impl fmt::Display for Chromosome {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.name())
    }
}
text_value!(
    Chromosome,
    "Chromosome",
    r"^([1-9]|1[0-9]|2[0-2]|X|Y|MT)$",
    "A primary chromosome in the caller's reference build."
);
