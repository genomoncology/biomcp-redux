use crate::{Text, UniProtAcc};
use serde::{Deserialize, Serialize};
/// A reviewed human protein listed by the authoritative gene record.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct Protein {
    /// Checked primary accession.
    pub accession: UniProtAcc,
    /// Recommended protein name.
    pub name: Text,
    /// Sequence length in amino acids.
    pub length: u32,
    /// Whole source function paragraphs, omitted when absent.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub function: Box<[Text]>,
    /// Source subcellular location names.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    pub subcellular_locations: Box<[Text]>,
}
