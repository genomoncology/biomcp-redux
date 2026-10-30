use crate::Text;
use serde::{Deserialize, Serialize};

/// An authority and its unchanged native identifier; null means the authority was not stated.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(extend("x-identifier" = true)))]
pub struct Identifier<T> {
    /// Native authority spelling; an explicit null carries an unknown authority.
    pub namespace: IdentifierNamespace,
    /// Native identifier value, with its original scalar type and precision.
    pub id: T,
}

impl<T> Identifier<T> {
    /// Pair a native value with a stated authority without parsing or rewriting the value.
    pub fn new(namespace: Text, id: T) -> Self {
        Self {
            namespace: namespace.into(),
            id,
        }
    }
}

impl<T: std::fmt::Display> std::fmt::Display for Identifier<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.namespace.as_deref() {
            Some(namespace) => write!(f, "{namespace}:{}", self.id),
            None => write!(f, "unknown:{}", self.id),
        }
    }
}

/// A source-stated authority or an explicit unknown authority, represented by null.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(untagged)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum IdentifierNamespace {
    /// The complete authority spelling supplied for this identifier.
    Stated(Text),
    /// The source supplied no authority; serialize null rather than omit the property.
    Unknown,
}
impl IdentifierNamespace {
    /// Borrow the native authority when one was stated.
    pub fn as_deref(&self) -> Option<&str> {
        match self {
            Self::Stated(value) => Some(value.as_str()),
            Self::Unknown => None,
        }
    }
}
impl From<Text> for IdentifierNamespace {
    fn from(value: Text) -> Self {
        Self::Stated(value)
    }
}
impl From<Option<Text>> for IdentifierNamespace {
    fn from(value: Option<Text>) -> Self {
        value.map_or(Self::Unknown, Self::Stated)
    }
}
