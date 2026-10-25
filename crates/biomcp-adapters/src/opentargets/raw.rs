//! Typed native Open Targets responses; no dataset release is invented.
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
/// GraphQL response with source errors kept distinct from empty data.
#[derive(Debug, Deserialize)]
pub struct Response<T> {
    /// Source result data.
    pub data: T,
    errors: Option<Vec<GraphError>>,
}
#[derive(Debug, Deserialize)]
struct GraphError {
    message: String,
}
impl<T: for<'de> Deserialize<'de>> Reply for Response<T> {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .errors
            .as_ref()
            .is_some_and(|e| !e.is_empty() || e.iter().any(|v| !v.message.is_empty()))
        {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
/// Source candidate collection.
#[derive(Debug, Deserialize)]
pub struct SearchData {
    /// Native candidate page.
    pub search: Search,
}
/// Candidate rows retain their native order.
#[derive(Debug, Deserialize)]
pub struct Search {
    /// Source total.
    pub total: u64,
    /// Source candidates.
    pub hits: Vec<Hit>,
}
/// One native candidate identity.
#[derive(Debug, Deserialize)]
pub struct Hit {
    /// Native ChEMBL identifier.
    pub id: String,
    /// Native source name.
    pub name: String,
    /// Native record entity.
    pub entity: String,
}
/// Exact-ID response.
#[derive(Debug, Deserialize)]
pub struct DrugData {
    /// Missing source record remains absent.
    pub drug: Option<Drug>,
}
/// Native source drug and its complete returned collections.
#[derive(Debug, Deserialize)]
pub struct Drug {
    /// Exact native ChEMBL identifier.
    pub id: String,
    /// Native source name.
    pub name: String,
    /// Complete native indication collection without invented server paging.
    pub indications: Rows<Indication>,
    /// Complete native mechanism collection without invented server paging.
    #[serde(rename = "mechanismsOfAction")]
    pub mechanisms_of_action: Rows<Mechanism>,
}
/// A native returned collection.
#[derive(Debug, Deserialize)]
pub struct Rows<T> {
    /// Native ordered rows.
    pub rows: Vec<T>,
}
/// Source indication and clinical stage, without regulatory interpretation.
#[derive(Debug, Deserialize)]
pub struct Indication {
    /// Native clinical stage spelling.
    #[serde(rename = "maxClinicalStage")]
    pub max_clinical_stage: String,
    /// Native disease identifier and label.
    pub disease: Disease,
}
/// A native disease reference without a name join.
#[derive(Debug, Deserialize)]
pub struct Disease {
    /// Native source identifier.
    pub id: String,
    /// Original source label.
    pub name: String,
}
/// Native mechanism and its complete target members.
#[derive(Debug, Deserialize)]
pub struct Mechanism {
    /// Complete original mechanism wording.
    #[serde(rename = "mechanismOfAction")]
    pub mechanism_of_action: String,
    /// Original action type.
    #[serde(rename = "actionType")]
    pub action_type: Option<String>,
    /// Complete native target member context.
    pub targets: Vec<Target>,
}
/// Native target gene identities and labels.
#[derive(Debug, Deserialize)]
pub struct Target {
    /// Native Ensembl gene ID.
    pub id: String,
    /// Original gene symbol.
    #[serde(rename = "approvedSymbol")]
    pub approved_symbol: String,
    /// Original gene name.
    #[serde(rename = "approvedName")]
    pub approved_name: String,
}
