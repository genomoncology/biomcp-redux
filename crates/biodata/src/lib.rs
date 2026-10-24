#![doc = include_str!("../README.md")]

mod article;
pub use article::{
    AbstractPart, Article, ArticleInput, ArticleMatchKind, Correction, CorrectionKind,
    CorrectionRelation,
};
pub use article::{ArticleHeader, Indexing, MeshHeading, MeshQualifier, PublicationType};
pub use id::{Doi, EuropePmcId, Pmcid, Pmid, SemanticScholarId};

mod text;

pub use text::Text;

mod coord;
mod date;
mod error;
mod id;
mod macros;

pub use coord::{Position1, Span0};
pub use date::PartialDate;
pub use error::Error;
pub use id::{Cytoband, EnsemblGeneId, GeneSymbol, HgncId, NcbiGeneId, OmimId, UniProtAcc};

mod gene;
pub use gene::{
    Gene, GeneHeader, GeneInput, GeneRef, GeneResolution, GeneStatus, HgncStatus, LocusGroup,
    LocusType, Mane, ManeStatus, Match, MatchKind, Summary, SummarySource,
};

pub use coord::{Build, Contig, GenomicInterval, Strand};

mod markdown;
pub use markdown::Markdown;

mod variant;
pub use variant::{Representation, Variant};

pub use coord::Chromosome;

pub use variant::{
    Accession, AccessionBase, Allele, Base, EnsemblFeature, LrgPart, RefSeqPrefix, Sequence,
    SequenceLocation,
};

pub use variant::{
    CaId, ClinVarAlleleId, ClinVarVariationId, CosmicId, Expression, ExpressionSyntax, HgvsText,
    PaId, RcvId, RsId, ScvId, VariantId, VcvId, VrsId,
};

mod trial;

pub use trial::{NctId, StudyType, TrialHeader, TrialPhase, TrialSex, TrialStatus};

mod trial_detail;
pub use trial_detail::{
    TrialAgeBound, TrialArm, TrialCard, TrialEligibility, TrialIntervention, TrialOutcome,
    TrialOutcomeKind,
};

pub use trial_detail::{TrialDate, TrialDateKind};

pub use trial_detail::{TrialArmKind, TrialInterventionKind};

mod trial_sites;
pub use trial_sites::{
    TrialContact, TrialContactRole, TrialCoordinates, TrialLocation, TrialOfficial,
    TrialOfficialRole, TrialSiteStatus,
};

mod trial_publications;

pub use trial_publications::{TrialDocument, TrialReference};

pub use variant::{
    AcmgAmp2015Code, AmpAscoCap2017LevelCode, AmpAscoCap2017TierCode, Attribution,
    CivicEvidenceLevelCode, ClingenCgcVicc2022Code, ClingenLowPenetrance2024Code, Coding, Concept,
    Curie, Direction, OncokbLevelCode, OncokbOncogenicityCode, Predicate, Proposition, Statement,
    Term, VariantRef,
};

pub use variant::{PopulationDataType, PopulationFrequency, PopulationSex};

mod number;
pub use number::{FiniteNumber, FiniteNumberError};

pub use variant::PredictorScore;
