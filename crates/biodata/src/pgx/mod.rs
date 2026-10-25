//! Native pharmacogenomics records without I/O, clinical propositions or patient inference.
mod accession;
mod cpic;
mod annotation;
mod detail;
pub use accession::{ClinPgxAccession, PgxGeneKey};
pub use cpic::*;
pub use annotation::*;
pub use detail::*;
