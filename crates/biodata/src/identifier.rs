use crate::Text;
use serde::{Deserialize, Serialize};

/// An authority and its unchanged native identifier; null means the authority was not stated.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct Identifier<T> {
    /// Native authority spelling; an explicit null carries an unknown authority.
    #[cfg_attr(feature = "schema", schemars(required))]
    pub namespace: Option<Text>,
    /// Native identifier value, with its original scalar type and precision.
    pub id: T,
}

impl<T> Identifier<T> {
    /// Pair a native value with a stated authority without parsing or rewriting the value.
    pub fn new(namespace: Text, id: T) -> Self {
        Self { namespace: Some(namespace), id }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.namespace {
            Some(namespace) => write!(f, "{namespace}:{}", self.id),
            None => write!(f, "unknown:{}", self.id),
        }
    }
}
