use super::{PageInfo, SourceOutcome, Status, Step};
use crate::Problem;
use serde::Serialize;
/// A response body supplies its schema identity and query type.
pub trait ResponseBody: Serialize {
    /// Versioned schema identity.
    const SCHEMA: &'static str;
    /// Query echoed by this response.
    type Query: Serialize;
}
/// One versioned command response; schema identity comes only from its body type.
/// JSON readers ignore unknown fields and unknown growable enum values within a major.
#[derive(Clone, Debug, Serialize)]
#[serde(bound = "T: ResponseBody")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(
    feature = "schema",
    schemars(bound = "T: ResponseBody + schemars::JsonSchema, T::Query: schemars::JsonSchema")
)]
pub struct Envelope<T: ResponseBody> {
    #[cfg_attr(feature = "schema", schemars(extend("const" = T::SCHEMA)))]
    schema: &'static str,
    /// Overall outcome.
    pub status: Status,
    /// Bounded local cache failure, independent of provider outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub local_error: Option<super::CacheProblem>,
    /// Parsed query.
    pub query: T::Query,
    /// Answer when available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<T>,
    /// Search page metadata.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<PageInfo>,
    /// Sources actually asked.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sources: Vec<SourceOutcome>,
    /// Rejected source fields.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub problems: Vec<Problem>,
    /// Available next commands.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next: Vec<Step>,
}
impl<T: ResponseBody> Envelope<T> {
    /// Construct a response with its body's schema identity.
    pub fn new(status: Status, query: T::Query, result: Option<T>) -> Self {
        Self {
            schema: T::SCHEMA,
            status,
            local_error: None,
            query,
            result,
            page: None,
            sources: Vec::new(),
            problems: Vec::new(),
            next: Vec::new(),
        }
    }
}
