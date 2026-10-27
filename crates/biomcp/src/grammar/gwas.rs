//! Legacy aliases normalize only the checked gene selector.
use biomcp_types::Request;
use serde_json::Value;
pub(super) fn request(verb: &str, mut value: Value) -> Result<Request, biomcp_types::QueryInvalid> {
    match verb {
        "search" => {
            let fields = value
                .as_object_mut()
                .ok_or_else(biomcp_types::QueryInvalid::default)?;
            let positional = fields.contains_key("query");
            if let Some(query) = fields.remove("query")
                && fields.insert("gene".into(), query).is_some()
            {
                return Err(biomcp_types::QueryInvalid::new(&["gene", "query"]));
            }
            super::validation::decode(value)
                .map(Request::SearchGwas)
                .map_err(|cause| {
                    if positional {
                        cause.rename("gene", "query")
                    } else {
                        cause
                    }
                })
        }
        "get" => {
            let fields = value
                .as_object_mut()
                .ok_or_else(biomcp_types::QueryInvalid::default)?;
            let id = fields
                .get("id")
                .and_then(Value::as_str)
                .ok_or_else(biomcp_types::QueryInvalid::default)?;
            let kind = if id.starts_with("association:") {
                "association"
            } else {
                "study"
            };
            fields.insert("kind".into(), kind.into());
            super::validation::decode::<biomcp_types::GwasGet>(value).map(Request::from)
        }
        _ => Err(biomcp_types::QueryInvalid::new(&["gene", "query"])),
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

/// Recognize an invalid threshold for the checked usage explanation.
pub(crate) fn invalid_threshold(args: &[std::ffi::OsString]) -> bool {
    let words: Vec<_> = args.iter().filter_map(|arg| arg.to_str()).collect();
    if !words.windows(2).any(|pair| pair == ["search", "gwas"]) {
        return false;
    }
    let mut words = words.into_iter();
    while let Some(word) = words.next() {
        if word == "--" {
            break;
        }
        let threshold = if word == "--p-value" {
            words.next()
        } else {
            word.strip_prefix("--p-value=")
        };
        if let Some(value) = threshold {
            return value.parse::<biodata::ScientificPValue>().is_err();
        }
    }
    false
}
