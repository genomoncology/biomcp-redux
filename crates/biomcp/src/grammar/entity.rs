use biomcp_types::Request;
use serde_json::Value;
pub(super) fn request(verb: &str, entity: &str, value: Value) -> Result<Request, ()> {
    let result = match (verb, entity) {
        ("cell-line", "assay-count") => {
            serde_json::from_value(value).map(Request::CellLineAssayCount)
        }
        ("search", "author") => serde_json::from_value(value).map(Request::SearchAuthor),
        ("get", "author") => serde_json::from_value(value).map(Request::GetAuthor),
        ("search", "cell-line") => serde_json::from_value(value).map(Request::SearchCellLine),
        ("get", "cell-line") => serde_json::from_value(value).map(Request::GetCellLine),
        ("protein", "domains") => serde_json::from_value(value).map(Request::ProteinDomains),
        ("protein", "complexes") => serde_json::from_value(value).map(Request::ProteinComplexes),
        ("protein", "complex-members") => {
            serde_json::from_value(value).map(Request::ProteinComplexMembers)
        }
        ("protein", "features") => serde_json::from_value(value).map(Request::ProteinFeatures),
        ("protein", "isoforms") => serde_json::from_value(value).map(Request::ProteinIsoforms),
        ("protein", "structures") => serde_json::from_value(value).map(Request::ProteinStructures),
        ("protein", "sequence") => serde_json::from_value(value).map(Request::ProteinSequence),
        ("search", "protein") => serde_json::from_value(value).map(Request::SearchProtein),
        ("get", "protein") => serde_json::from_value(value).map(Request::GetProtein),
        ("search", "phenotype") => serde_json::from_value(value).map(Request::SearchPhenotype),
        ("get", "phenotype") => serde_json::from_value(value).map(Request::GetPhenotype),
        ("search", "pathway") => serde_json::from_value(value).map(Request::SearchPathway),
        ("get", "pathway") => serde_json::from_value(value).map(Request::GetPathway),
        ("pathway", "genes") => serde_json::from_value(value).map(Request::PathwayGenes),
        ("pathway", "events") => serde_json::from_value(value).map(Request::PathwayEvents),
        ("search", "study") => serde_json::from_value(value).map(Request::SearchStudy),
        ("get", "study") => serde_json::from_value(value).map(Request::GetStudy),
        ("search", "pgx") => serde_json::from_value(value).map(Request::SearchPgx),
        ("get", "pgx") if value.get("annotation_id").is_some() => {
            let mut value = value;
            if let Some(object) = value.as_object_mut()
                && let Some(id) = object.remove("annotation_id")
            {
                object.insert("id".into(), id);
            }
            serde_json::from_value(value).map(Request::GetPgxAnnotation)
        }
        ("get", "pgx") => serde_json::from_value(value).map(Request::GetPgx),
        ("search", "diagnostic") => serde_json::from_value(value).map(Request::SearchDiagnostic),
        ("get", "diagnostic") => serde_json::from_value(value).map(Request::GetDiagnostic),
        ("search", "adverse-event") => {
            serde_json::from_value(value).map(Request::SearchAdverseEvent)
        }
        ("get", "adverse-event") => serde_json::from_value(value).map(Request::GetAdverseEvent),
        ("search", "disease") => serde_json::from_value(value).map(Request::SearchDisease),
        ("get", "disease") => serde_json::from_value(value).map(Request::GetDisease),
        ("search", "drug") => serde_json::from_value(value).map(Request::SearchDrug),
        ("get", "drug") if value.get("source").and_then(Value::as_str) == Some("who-pq") => {
            serde_json::from_value(value).map(Request::GetWhoMedicine)
        }
        ("get", "drug") => serde_json::from_value(value).map(Request::GetDrug),
        _ => return Err(()),
    };
    result.map_err(|_| ())
}
