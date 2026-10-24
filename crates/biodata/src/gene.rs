//! Checked human gene identity and source-stated annotations.
mod input;
pub use input::GeneInput;

mod values;
mod annotations;
pub use values::{GeneStatus, LocusGroup, MatchKind, SummarySource, HgncStatus, LocusType};
pub use annotations::{ManeSelect, Summary};

mod model;
pub use model::{Gene, GeneHeader, GeneResolution, Match};
