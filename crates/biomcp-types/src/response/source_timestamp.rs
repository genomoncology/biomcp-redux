//! Preserve native processing timestamps without inventing a missing timezone.
use serde::{Deserialize, Serialize};

/// Native source timestamp and the availability of its timezone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct SourceTimestamp {
    /// Exact source-supplied timestamp.
    pub value: crate::Text,
    /// Whether the native timestamp includes a timezone.
    pub timezone: SourceTimestampTimezone,
}

biodata::wire_enum! {
    /// Availability of timezone information in a native timestamp.
    SourceTimestampTimezone {
        /// The value retains the timezone supplied by the source.
        InValue => "in_value",
        /// The source supplied no timezone; none has been inferred.
        SourceGaveNone => "source_gave_none",
    }
}

impl From<crate::Text> for SourceTimestamp {
    fn from(value: crate::Text) -> Self {
        let zoned = value.as_str().split_once('T').is_some_and(|(_, time)| {
            time.ends_with('Z')
                || time
                    .char_indices()
                    .any(|(index, ch)| index > 0 && matches!(ch, '+' | '-'))
        });
        Self {
            value,
            timezone: if zoned {
                SourceTimestampTimezone::InValue
            } else {
                SourceTimestampTimezone::SourceGaveNone
            },
        }
    }
}
