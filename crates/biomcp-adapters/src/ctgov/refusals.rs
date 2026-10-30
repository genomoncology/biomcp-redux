use crate::{Refusal, mapped::Problems, raw::Field};
use serde::{Deserialize, Deserializer, Serializer};

pub(super) fn none() -> Refusal {
    Refusal::None
}

pub(super) fn serialize<S: Serializer>(value: &Refusal, serializer: S) -> Result<S::Ok, S::Error> {
    serializer.serialize_bool(*value == Refusal::Refused)
}

pub(super) fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<Refusal, D::Error> {
    Ok(if bool::deserialize(deserializer)? {
        Refusal::Refused
    } else {
        Refusal::None
    })
}

// Only examined native record members establish refusals. An invalid optional
// collection remains a field problem on its retained parent.
pub(super) fn records<T>(
    problems: &mut Problems,
    refusals: &mut Refusal,
    field: &str,
    value: Option<Field<Vec<Field<T>>>>,
) -> Box<[T]> {
    problems
        .field(field, value)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|row| match row {
            Field::Value(value) => Some(value),
            invalid @ Field::Invalid(_) => {
                *refusals = Refusal::Refused;
                problems.field(field, Some(invalid))
            }
        })
        .collect()
}
