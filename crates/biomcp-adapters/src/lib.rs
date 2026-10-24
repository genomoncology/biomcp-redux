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
pub mod semantic_scholar;
mod xml_element;

pub mod registry;
pub mod sheet;
