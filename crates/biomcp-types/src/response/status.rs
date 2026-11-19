use serde::{Deserialize, Serialize};
biodata::wire_enum! {
    /// Overall command outcome.
    Status {
        /// Every source row answered or was skipped on purpose, and nothing is marked missing.
        Complete => "complete",
        /// Answered, but a source row failed, is stale or is syncing, or requested data is missing;
        /// `incomplete` says where.
        Partial => "partial",
        /// Every identity was confirmed absent.
        Absent => "absent",
        /// No requested identity could be answered.
        Failed => "failed",
    }
}
/// Outcome for one input.
pub type ItemStatus = Status;

/// One thing a `partial` reply is missing, pointing at the detail already in the reply.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Incomplete {
    /// JSON pointer into this reply, such as `/sources/1` or a record's field.
    pub at: crate::Text,
    /// The source whose data is missing.
    pub source: crate::Source,
    /// Why, when a typed source reason is known.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<crate::Reason>,
    /// The refused record's own problem code, when a problem is the cause.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<crate::ProblemCode>,
}
