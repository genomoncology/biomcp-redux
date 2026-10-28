use crate::{Accession, Error, Sequence, Span0};
use serde::{Deserialize, Serialize};
/// An interbase interval on one versioned sequence accession.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SequenceLocation {
    /// Versioned source sequence, without an inferred genome build.
    #[serde(serialize_with = "serialize_sequence")]
    pub sequence: Accession,
    /// Zero-based half-open bounds.
    pub span: Span0,
}
/// Source-stated interbase replacement; equality never claims normalized equivalence.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct Allele(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    location: SequenceLocation,
    reference: Sequence,
    alternate: Sequence,
}
impl Allele {
    /// Check only shape and reference length; never trim, shift or read a genome.
    pub fn new(
        location: SequenceLocation,
        reference: Sequence,
        alternate: Sequence,
    ) -> Result<Self, Error> {
        checked_sequence(&location.sequence)?;
        if reference.is_empty() && alternate.is_empty() {
            return Err(Error::AlleleEmpty);
        }
        if reference.len() as u64 != u64::from(location.span.end() - location.span.start()) {
            return Err(Error::AlleleReferenceLength);
        }
        Ok(Self(Fields {
            location,
            reference,
            alternate,
        }))
    }
    /// Versioned sequence location.
    pub fn location(&self) -> &SequenceLocation {
        &self.0.location
    }
    /// Source reference bases.
    pub fn reference(&self) -> &Sequence {
        &self.0.reference
    }
    /// Source alternate bases, including a reference allele equal to the reference.
    pub fn alternate(&self) -> &Sequence {
        &self.0.alternate
    }
}

// Accession's copied enum permits public field construction. Reuse its parser at
// receiving boundaries so unchecked enum fields cannot escape as an allele.
fn checked_sequence(sequence: &Accession) -> Result<(), Error> {
    sequence.to_string().parse::<Accession>().map(|_| ())
}

fn serialize_sequence<S: serde::Serializer>(
    sequence: &Accession,
    serializer: S,
) -> Result<S::Ok, S::Error> {
    checked_sequence(sequence).map_err(serde::ser::Error::custom)?;
    sequence.serialize(serializer)
}
impl TryFrom<Fields> for Allele {
    type Error = Error;
    fn try_from(fields: Fields) -> Result<Self, Error> {
        Self::new(fields.location, fields.reference, fields.alternate)
    }
}
