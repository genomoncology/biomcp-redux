use crate::{Refusal, mapped::Problems, raw::Field};
pub(super) use crate::mapped::refusal_serde::{deserialize, none, serialize};

// Only examined members establish refusals; optional containers remain fields.
pub(super) fn records<T>(
    problems: &mut Problems,
    refusals: &mut Refusal,
    field: &str,
    value: Option<Field<Vec<Field<T>>>>,
) -> Box<[T]> {
    problems.records(field, value, refusals)
}
