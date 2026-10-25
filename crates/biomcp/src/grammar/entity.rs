use biomcp_types::Request;
use serde_json::Value;
pub(super) fn request(verb: &str, entity: &str, value: Value) -> Result<Request, ()> {
    let result = match (verb, entity) {
        ("search", "pgx") => serde_json::from_value(value).map(Request::SearchPgx),
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
        ("get", "drug") => serde_json::from_value(value).map(Request::GetDrug),
        _ => return Err(()),
    };
    result.map_err(|_| ())
}
