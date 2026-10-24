//! Source-qualified variant identity, without local molecular interpretation.
mod model;
pub use model::*;

mod accession;
pub use accession::{Accession, EnsemblFeature, LrgPart, RefSeqPrefix};
mod sequence;
pub use sequence::{Base, Sequence};
mod allele;
pub use allele::{Allele, SequenceLocation};

mod id;
pub use id::VariantId;
mod notation;
pub use notation::{HgvsText, Expression, ExpressionSyntax};
