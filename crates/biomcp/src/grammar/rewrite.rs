//! Translate checked query fields at the public command boundary.
#![deny(clippy::wildcard_enum_match_arm)]
use biomcp_types::{QueryInvalid, Request, Rewrite, decode_query};
use serde_json::Value;

pub(super) fn request(
    via: Rewrite,
    verb: &str,
    entity: &str,
    mut value: Value,
    matches: &clap::ArgMatches,
) -> Result<Request, QueryInvalid> {
    match via {
        Rewrite::Direct | Rewrite::GetPhenotype => Request::decode(verb, entity, value),
        Rewrite::GetPgx => {
            if value.get("annotation_id").is_some() {
                if let Some(fields) = value.as_object_mut()
                    && let Some(id) = fields.remove("annotation_id")
                {
                    fields.insert("id".into(), id);
                }
                decode_query(value).map(Request::GetPgxAnnotation)
                    .map_err(|cause| cause.rename("id", "annotation_id"))
            } else {
                decode_query(value).map(Request::GetPgx)
            }
        }
        Rewrite::GetDrug => {
            if value.get("source").and_then(Value::as_str) == Some("who-pq") {
                decode_query(value).map(Request::GetWhoMedicine)
            } else {
                decode_query(value).map(Request::GetDrug)
            }
        }
        Rewrite::GetGwas => {
            let fields = value.as_object_mut().ok_or_else(QueryInvalid::default)?;
            let id = fields.get("id").and_then(Value::as_str).ok_or_else(QueryInvalid::default)?;
            let kind = if id.starts_with("association:") { "association" } else { "study" };
            fields.insert("kind".into(), kind.into());
            decode_query::<biomcp_types::GwasGet>(value).map(Request::from)
        }
        Rewrite::SearchGwas => {
            let fields = value.as_object_mut().ok_or_else(QueryInvalid::default)?;
            let positional = fields.contains_key("query");
            if let Some(query) = fields.remove("query")
                && fields.insert("gene".into(), query).is_some()
            {
                return Err(QueryInvalid::new(&["gene", "query"]).with_message(
                    biomcp_types::ErrorMessage::GwasSearchArguments,
                ));
            }
            decode_query(value).map(Request::SearchGwas).map_err(|cause| {
                if positional { cause.rename("gene", "query") } else { cause }
            })
        }
        Rewrite::SearchCellLine | Rewrite::SearchAuthor => {
            if let Some(query) = matches.get_one::<String>("query_option") {
                value.as_object_mut().ok_or_else(QueryInvalid::default)?
                    .insert("query".into(), query.clone().into());
            }
            if via == Rewrite::SearchAuthor {
                decode_query(value).map(Request::SearchAuthor)
            } else {
                decode_query(value).map(Request::SearchCellLine)
            }
        }
    }
}

pub(crate) fn query_value(request: &Request) -> Result<Value, serde_json::Error> {
    let mut value = request.query_value()?;
    match request.rewrite() {
        Rewrite::Direct | Rewrite::GetDrug | Rewrite::GetGwas | Rewrite::SearchGwas
        | Rewrite::SearchCellLine | Rewrite::SearchAuthor => {}
        Rewrite::GetPgx => {
            if let Request::GetPgx(query) = request
                && query.section_limit() == query.default_section_limit()
                && let Some(fields) = value.as_object_mut()
            {
                fields.remove("section_limit");
            }
            if let Request::GetPgxAnnotation(_) = request
                && let Some(fields) = value.as_object_mut()
                && let Some(id) = fields.remove("id")
            {
                fields.insert("annotation_id".into(), id);
            }
        }
        Rewrite::GetPhenotype => {
            if let Request::GetPhenotype(query) = request
                && let Some(fields) = value.as_object_mut()
            {
                if query.source() == query.default_source() { fields.remove("source"); }
                if query.section_limit() == query.default_section_limit() { fields.remove("section_limit"); }
            }
        }
    }
    Ok(value)
}
