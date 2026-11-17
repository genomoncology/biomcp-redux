//! g:Profiler g:GOSt over-representation analysis of a human gene list.
//!
//! Only openly licensed term sources are requested: Reactome, Gene Ontology and WikiPathways.
//! KEGG, TRANSFAC, CORUM, HPA, HP and miRTarBase carry their own terms and are never asked.
mod profile;
pub use profile::{SPEC, TEST, enrichment};

/// Complete source policy and command declaration.
pub(crate) mod decl;
