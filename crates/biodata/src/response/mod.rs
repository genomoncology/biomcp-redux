mod error;
mod problem;

pub use problem::{Problem, ProblemCode};

pub use error::{ErrorCode, ErrorEnvelope, ErrorStatus};

mod envelope;
mod gene;
mod page;
mod section;
mod source;
mod status;
mod step;
pub use envelope::{Envelope, ResponseBody};
pub use gene::{GeneCards, GeneItem, GenePage, QueryBody};
pub use page::PageInfo;
pub use section::{Section, SourceRecord};
pub use source::{SourceOutcome, SourceState};
pub use status::{ItemStatus, Status};
pub use step::{Step, StepKind};

pub use gene::{SyncQuery, SyncResult};
