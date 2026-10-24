//! Checked source replies mapped into shared human gene values.
pub mod hgnc;
mod mapped;
pub mod mygene;
/// Keyed OncoKB gene annotations.
pub mod oncokb;
pub mod pubmed;
mod quotas;
mod raw;
pub use mapped::Mapped;

pub mod gnomad;
pub mod uniprot;

pub mod reactome;

pub mod civic;

pub mod clingen;

pub mod hpa;

pub mod europepmc;
pub mod pubtator;

pub mod jats;
pub mod pmc_oa;

pub mod icite;

/// Exact variant publication lookup.
pub mod litvar;
/// Semantic Scholar graph metadata under one shared optional-key quota.
pub mod pdf;
pub mod semantic_scholar;
mod xml_element;

pub mod registry;

/// Sentence resemblance in native provider order.
pub mod litsense;
pub mod sheet;

pub mod allele_registry;
/// ClinicalTrials.gov native search.
pub mod ctgov;

pub mod mutalyzer;
pub mod myvariant;
/// Explicit keyed NCI native trials.
pub mod nci;
pub mod variantvalidator;

/// GWAS Catalog v2 exact rsID associations.
pub mod gwas;
