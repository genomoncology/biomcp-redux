//! CLI grammar shapes derived from the standalone checked requests.
use super::{GwasAssociationSection, GwasGet, GwasSectionPage, GwasStudySection};
use crate::{QueryInvalid, Text};
use serde::{Deserialize, Serialize};
/// Flat source identity spelling; resource pairing remains checked by GwasGet.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct GwasGetCommand {
    /// Exact GCST study accession or association:ID; never an inferred resource.
    #[cfg_attr(feature = "schema", schemars(extend("x-positional" = 0)))]
    pub id: Text,
    /// Study: associations or ancestries. Association: loci.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "schema", schemars(with = "Text", extend("x-positional" = 1)))]
    pub section: Option<Text>,
    /// Independent bounded relation window.
    #[serde(flatten)]
    pub page: GwasSectionPage,
}
impl TryFrom<GwasGetCommand> for GwasGet {
    type Error = QueryInvalid;
    fn try_from(q: GwasGetCommand) -> Result<Self, Self::Error> {
        if q.id.as_str().starts_with("association:") {
            Ok(Self::Association {
                id: q.id.as_str().parse().map_err(|_| QueryInvalid)?,
                section: q
                    .section
                    .map(|s| s.as_str().parse::<GwasAssociationSection>())
                    .transpose()
                    .map_err(|_| QueryInvalid)?,
                page: q.page,
            })
        } else {
            Ok(Self::Study {
                id: q.id.as_str().parse().map_err(|_| QueryInvalid)?,
                section: q
                    .section
                    .map(|s| s.as_str().parse::<GwasStudySection>())
                    .transpose()
                    .map_err(|_| QueryInvalid)?,
                page: q.page,
            })
        }
    }
}
/// Search command grammar omits the fixed source order and deferred threshold.
#[cfg(feature = "schema")]
#[derive(Debug)]
pub struct GwasSearchCommand;
#[cfg(feature = "schema")]
impl schemars::JsonSchema for GwasSearchCommand {
    fn schema_name() -> std::borrow::Cow<'static, str> {
        "GwasSearchCommand".into()
    }
    fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {
        let mut schema = super::GwasSearch::json_schema(generator);
        if let Some(fields) = schema.get_mut("properties").and_then(|v| v.as_object_mut()) {
            fields.remove("order");
            fields.remove("p_value");
            if let Some(gene) = fields.get("gene").cloned() {
                let mut query = gene;
                if let Some(object) = query.as_object_mut() {
                    object.insert("x-positional".into(), 0.into());
                }
                fields.insert("query".into(), query);
            }
        }
        schema
    }
}
