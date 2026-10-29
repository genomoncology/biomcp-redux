mod gtr;
mod id;
mod regulatory;
pub use gtr::*;
pub use id::*;
pub use regulatory::*;
use serde::{Deserialize, Serialize};
/// Source-owned inventory or regulatory records; none is a clinical Statement.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", content = "record", rename_all = "kebab-case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum DiagnosticRecord {
    /// Original GTR test-version row, including independently retained conflicts.
    GtrTestVersion(Box<GtrTestVersion>),
    /// One native GTR membership or method row.
    GtrTestRelation(GtrTestRelation),
    /// Complete selected WHO listing.
    WhoIvdProduct(WhoIvdProduct),
    /// Exact FDA regulatory submission.
    DeviceSubmission(DeviceSubmission),
    /// Complete scoped companion table row.
    CompanionDiagnosticRelation(CompanionDiagnosticRelation),
}
