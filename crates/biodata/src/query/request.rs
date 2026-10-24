use super::{ArticleSearch, GeneGet, GeneSearch};
use serde::{Deserialize, Serialize};
/// One checked query. Housekeeping actions are separate.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "command", rename_all = "snake_case")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub enum Request {
    /// Observe an exact allele in an explicitly selected cohort.
    VariantRecurrence(crate::VariantRecurrenceQuery),
    /// Send unchanged HGVS to explicit normalization services.
    VariantNormalize(crate::VariantNormalize),
