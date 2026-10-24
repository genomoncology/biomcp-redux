use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum OneOrMany<T> {
    One(T),
    Many(Vec<T>),
}
impl<T> OneOrMany<T> {
    pub(crate) fn values(self) -> Vec<T> {
        match self {
            Self::One(value) => vec![value],
            Self::Many(values) => values,
        }
    }
}

// Retain malformed optional fields as mapping problems without rejecting
// unrelated fields or keeping arbitrary provider data.
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(crate) enum Field<T> {
    Value(T),
    Invalid(serde::de::IgnoredAny),
}
impl<T> Field<T> {
    pub(crate) fn value(self) -> Option<T> {
        match self {
            Self::Value(value) => Some(value),
            Self::Invalid(_) => None,
        }
    }
}
