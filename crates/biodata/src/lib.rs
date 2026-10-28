#![doc = include_str!("../README.md")]

pub use id::{Doi, EuropePmcId, Pmcid, Pmid, SemanticScholarId};

mod source_text;
pub use source_text::SourceText;
mod ontology_id;
mod text;
pub use ontology_id::{MondoId, NcitId};

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
    Gene, GeneLocation, GeneLocationKind, GeneRef, GeneStatus, HgncStatus, LocusGroup, LocusType,
    Mane, ManeStatus, MatchKind, Summary, SummarySource,
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

pub use trial::{NctId, StudyType, TrialPhase, TrialSex, TrialStatus};

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

mod license;
pub use license::License;

mod disease;
pub use disease::{
    DiseaseAssociation, DiseaseAssociationQualifier, DiseaseAssociationReference, DiseaseEndpoint,
    DiseaseRelation, DiseaseStatistic, DiseaseSynonym, StatisticEstimate, StatisticSelector,
};

mod drug;
pub use drug::{
    AuthorizationDate, AuthorizationEvent, DrugDiseaseReference, DrugId, DrugIdentity,
    DrugIdentityLevel, DrugIdentityRelation, DrugIndicationClaim, DrugInteractionClaim,
    DrugLabelReference, DrugLabelSection, DrugNativeReference, DrugNativeTarget, DrugProduct,
    DrugStage, DrugTargetRelation,
};

mod adverse_event;
pub use adverse_event::{
    FaersDrug, FaersIdentifiers, FaersPatient, FaersReaction, FaersReport, FaersReporter,
    FaersSeriousness, MaudeDevice, MaudeNarrative, MaudeReport, ReportId, ReportIdInvalid,
};

mod pgx;
pub use pgx::*;
mod diagnostic;
pub use diagnostic::*;

mod gwas;
pub use gwas::{GwasAssociationId, GwasStudyId, GwasTraitId, GwasValueInvalid, ScientificPValue};
mod phenotype;
pub use phenotype::*;
mod pathway;
pub use pathway::{
    Pathway, PathwayEventRef, PathwayId, PathwayIdInvalid, PathwayLiteratureRef, PathwayMember,
    PathwayParticipant, PathwayRef, PathwaySourceRef, PathwaySpecies, PathwaySummation, ReactomeId,
    ReactomeIdInvalid, WikiPathwaysId,
};

mod protein;
pub use protein::*;
mod study;
pub use study::*;

mod cell_line;
pub use cell_line::*;

mod author;
pub use author::*;

/// Omit an optional collection when the source supplied no values.
pub fn absent_or_empty<T>(value: &Option<Box<[T]>>) -> bool {
    value.as_ref().is_none_or(|items| items.is_empty())
}
