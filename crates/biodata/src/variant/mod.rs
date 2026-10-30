//! Source-qualified variant identity, without local molecular interpretation.
mod model;
pub use model::*;

mod accession;
pub use accession::{Accession, AccessionBase, EnsemblFeature, LrgPart, RefSeqPrefix};
mod sequence;
pub use sequence::{Base, Sequence};
mod allele;
pub use allele::{Allele, SequenceLocation};

mod id;
pub use id::{
    CaId, ClinVarAlleleId, ClinVarVariationId, CosmicId, PaId, RcvId, RsId, ScvId, VariantId,
    VcvId, VrsId,
};
mod notation;
pub use notation::{Expression, ExpressionSyntax, HgvsText};

mod predictor;
pub use predictor::*;
mod statement;
pub use statement::*;

mod population;
pub use population::{GenotypeFilter, PopulationDataType, PopulationFrequency, PopulationSex};

pub(crate) use accession::AccessionKind;

mod coding_change;
pub use coding_change::CodingChange;
