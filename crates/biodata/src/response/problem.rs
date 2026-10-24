use crate::{Source, Text, source::canonical_enum};
use serde::{Deserialize, Serialize};

canonical_enum! {
    /// A stable code for a rejected field value.
    ProblemCode, ProblemCodeUnknown {
        /// The field value was not recognized.
        ValueUnknown => "value.unknown",
    }
}

/// A field that could not be accepted from a source record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Problem {
    /// The source that supplied the record.
    pub source: Source,
    /// The record's canonical identifier, when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub record: Option<Text>,
    /// The biodata field name.
    pub field: Box<str>,
    /// Why the field value was not accepted.
    pub code: ProblemCode,
}
