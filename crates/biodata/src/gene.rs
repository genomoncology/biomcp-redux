//! Checked human gene identity and source-stated annotations.
mod input;
pub use input::GeneInput;

mod annotations;
mod values;
pub use annotations::{Mane, ManeStatus, Summary};
pub use values::{GeneStatus, HgncStatus, LocusGroup, LocusType, MatchKind, SummarySource};

mod model;
pub use model::{Gene, GeneHeader, GeneResolution, Match};

mod reference;
pub use reference::GeneRef;
