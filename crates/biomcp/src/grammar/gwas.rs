//! Legacy aliases normalize only the checked gene selector.
use biomcp_types::{GwasGet, GwasGetCommand, Request};
use serde_json::Value;
pub(super) fn request(verb: &str, mut value: Value) -> Result<Request, ()> {
    match verb {
        "search" => {
            let fields = value.as_object_mut().ok_or(())?;
            if let Some(query) = fields.remove("query") {
                if fields.insert("gene".into(), query).is_some() {
                    return Err(());
                }
            }
            serde_json::from_value(value)
                .map(Request::SearchGwas)
                .map_err(|_| ())
        }
        "get" => serde_json::from_value::<GwasGetCommand>(value)
            .map_err(|_| ())
            .and_then(|q| GwasGet::try_from(q).map(Request::GetGwas).map_err(|_| ())),
        _ => Err(()),
    }
}
pub(super) fn aliases(command: clap::Command, schema: &Value) -> clap::Command {
    if schema.pointer("/properties/query").is_some() {
        command
            .mut_arg("gene", |arg| arg.short('g'))
            .mut_arg("limit", |arg| arg.short('l'))
    } else {
        command
    }
}
