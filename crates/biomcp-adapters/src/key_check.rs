//! Interpret the authentication signal from each publisher's cheapest read.
use biomcp_fetch::{Fetch, RequestError, SecretName};
use biomcp_types::{KeyCheckStatus, Reason};
use serde_json::Value;
use std::time::Duration;

/// Whether the provider has a supported authenticated check.
pub const fn supported(name: SecretName) -> bool {
    matches!(
        name,
        SecretName::NcbiApiKey
            | SecretName::NciApiKey
            | SecretName::OpenfdaApiKey
            | SecretName::AlphaGenomeApiKey
            | SecretName::SemanticScholarApiKey
    )
}

/// Make one publisher request using the effective registered credential.
pub fn run(fetch: &Fetch, name: SecretName, budget: Duration) -> KeyCheckStatus {
    let declaration = crate::decl::decl(crate::decl::key_source(name));
    let leg = fetch.leg(declaration.spec, fetch.deadline(budget));
    if name == SecretName::AlphaGenomeApiKey {
        return match crate::alphagenome::key_check::check(&leg) {
            Ok(()) => KeyCheckStatus::Working,
            Err(RequestError::Source(error)) if error.reason == Reason::Rejected => {
                rejected("Authentication or permission denied")
            }
            Err(error) => failure(error),
        };
    }
    let crate::decl::Probe::Get(path, args) = declaration.probe else {
        return unavailable(Reason::Unsupported);
    };
    match leg.inspect_key(path, args, |status, bytes| http_reply(name, status, bytes)) {
        Ok(result) => result,
        Err(RequestError::Source(error)) if error.reason == Reason::BadReply => {
            // A provider may echo a bad key. The transport quarantines that body.
            match error.status.and_then(|status| {
                if name == SecretName::NcbiApiKey && status == 400 {
                    Some("API key invalid")
                } else {
                    auth_status(status)
                }
            }) {
                Some(reason) => rejected(reason),
                None => unavailable(error.reason),
            }
        }
        Err(error) => failure(error),
    }
}

fn http_reply(name: SecretName, status: u16, bytes: &[u8]) -> KeyCheckStatus {
    let body = serde_json::from_slice::<Value>(bytes).unwrap_or(Value::Null);
    if status != 429 && status < 500 {
        let detail = match name {
            SecretName::NcbiApiKey if body["error"] == "API key invalid" => Some("API key invalid"),
            SecretName::OpenfdaApiKey
                if body.pointer("/error/code").and_then(Value::as_str)
                    == Some("API_KEY_INVALID") =>
            {
                Some("API_KEY_INVALID")
            }
            _ => None,
        };
        if let Some(reason) = detail.or_else(|| auth_status(status)) {
            return rejected(reason);
        }
    }
    match status {
        200..=299 if valid(name, &body) => KeyCheckStatus::Working,
        429 => unavailable(Reason::RateLimited),
        500..=599 => unavailable(Reason::Unavailable),
        _ => unavailable(Reason::BadReply),
    }
}

fn auth_status(status: u16) -> Option<&'static str> {
    match status {
        401 => Some("Unauthorized"),
        403 => Some("Forbidden"),
        _ => None,
    }
}

fn valid(name: SecretName, body: &Value) -> bool {
    match name {
        SecretName::NcbiApiKey => body
            .pointer("/einforesult/dbinfo")
            .is_some_and(Value::is_array),
        SecretName::OpenfdaApiKey => body["results"].is_array(),
        SecretName::NciApiKey | SecretName::SemanticScholarApiKey => {
            body["total"].is_u64() && body["data"].is_array()
        }
        _ => false,
    }
}

fn rejected(reason: &str) -> KeyCheckStatus {
    KeyCheckStatus::Rejected {
        reason: reason.into(),
    }
}

fn unavailable(reason: Reason) -> KeyCheckStatus {
    KeyCheckStatus::CouldNotCheck {
        reason: reason.to_string(),
    }
}

fn failure(error: RequestError) -> KeyCheckStatus {
    match error {
        RequestError::Source(error) => unavailable(error.reason),
        _ => KeyCheckStatus::CouldNotCheck {
            reason: error.to_string(),
        },
    }
}
