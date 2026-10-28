use biomcp_types::Request;
use serde_json::Value;
pub(super) fn request(
    verb: &str,
    entity: &str,
    value: Value,
) -> Result<Request, biomcp_types::QueryInvalid> {
    match (verb, entity) {
        ("author", "papers") => super::validation::decode(value).map(Request::AuthorPapers),
        ("cell-line", "assay-count") => {
            super::validation::decode(value).map(Request::CellLineAssayCount)
        }
        ("search", "author") => super::validation::decode(value).map(Request::SearchAuthor),
        ("get", "author") => super::validation::decode(value).map(Request::GetAuthor),
        ("search", "cell-line") => super::validation::decode(value).map(Request::SearchCellLine),
        ("get", "cell-line") => super::validation::decode(value).map(Request::GetCellLine),
        ("protein", "domains") => super::validation::decode(value).map(Request::ProteinDomains),
        ("protein", "complexes") => super::validation::decode(value).map(Request::ProteinComplexes),
        ("protein", "complex-members") => {
            super::validation::decode(value).map(Request::ProteinComplexMembers)
        }
        ("protein", "features") => super::validation::decode(value).map(Request::ProteinFeatures),
        ("protein", "isoforms") => super::validation::decode(value).map(Request::ProteinIsoforms),
        ("protein", "structures") => {
            super::validation::decode(value).map(Request::ProteinStructures)
        }
        ("protein", "sequence") => super::validation::decode(value).map(Request::ProteinSequence),
        ("search", "protein") => super::validation::decode(value).map(Request::SearchProtein),
        ("get", "protein") => super::validation::decode(value).map(Request::GetProtein),
        ("search", "phenotype") => super::validation::decode(value).map(Request::SearchPhenotype),
        ("get", "phenotype") => super::validation::decode(value).map(Request::GetPhenotype),
        ("search", "organization") => {
            super::validation::decode(value).map(Request::SearchOrganization)
        }
        ("search", "pathway") => super::validation::decode(value).map(Request::SearchPathway),
        ("get", "pathway") => super::validation::decode(value).map(Request::GetPathway),
        ("pathway", "genes") => super::validation::decode(value).map(Request::PathwayGenes),
        ("pathway", "events") => super::validation::decode(value).map(Request::PathwayEvents),
        ("search", "study") => super::validation::decode(value).map(Request::SearchStudy),
        ("get", "study") => super::validation::decode(value).map(Request::GetStudy),
        ("search", "pgx") => super::validation::decode(value).map(Request::SearchPgx),
        ("get", "pgx") if value.get("annotation_id").is_some() => {
            let mut value = value;
            if let Some(object) = value.as_object_mut()
                && let Some(id) = object.remove("annotation_id")
            {
                object.insert("id".into(), id);
            }
            super::validation::decode(value)
                .map(Request::GetPgxAnnotation)
                .map_err(|cause| cause.rename("id", "annotation_id"))
        }
        ("get", "pgx") => super::validation::decode(value).map(Request::GetPgx),
        ("search", "diagnostic") => super::validation::decode(value).map(Request::SearchDiagnostic),
        ("get", "diagnostic") => super::validation::decode(value).map(Request::GetDiagnostic),
        ("search", "adverse-event") => {
            super::validation::decode(value).map(Request::SearchAdverseEvent)
        }
        ("get", "adverse-event") => super::validation::decode(value).map(Request::GetAdverseEvent),
        ("search", "disease") => super::validation::decode(value).map(Request::SearchDisease),
        ("get", "disease") => super::validation::decode(value).map(Request::GetDisease),
        ("search", "drug") => super::validation::decode(value).map(Request::SearchDrug),
        ("get", "drug") if value.get("source").and_then(Value::as_str) == Some("who-pq") => {
            super::validation::decode(value).map(Request::GetWhoMedicine)
        }
        ("get", "drug") => super::validation::decode(value).map(Request::GetDrug),
        _ => Err(biomcp_types::QueryInvalid::default()),
    }
}
