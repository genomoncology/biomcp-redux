//! Preserve native processing timestamps without inventing a missing timezone.
use serde::{Deserialize, Serialize};
mod updated;
mod value;
pub use updated::SourceUpdated;
pub use value::SourceTimestampValue;

/// Native source timestamp and the availability of its timezone.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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
        InValue => "in-value",
        /// The source supplied no timezone; none has been inferred.
        SourceGaveNone => "source-gave-none",
    }
}

impl From<crate::Text> for SourceTimestamp {
    fn from(value: crate::Text) -> Self {
        let zoned = value.as_str().split_once('T').is_some_and(|(_, time)| {
            time.ends_with('Z')
                || matches!(
                    time.as_bytes().get(time.len().saturating_sub(6)..),
                    Some([b'+' | b'-', h1, h2, b':', m1, m2])
                        if h1.is_ascii_digit() && h2.is_ascii_digit()
                            && m1.is_ascii_digit() && m2.is_ascii_digit()
                )
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
