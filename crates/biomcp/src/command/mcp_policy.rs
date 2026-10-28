//! Transport authority is checked on parsed commands before fetch or save work.
use biomcp_query::Action;
use biomcp_types::{ErrorEnvelope, ErrorMessage, Request};

pub(super) fn refusal(
    action: &Action,
    globals: &crate::grammar::Globals,
    http: bool,
) -> Option<ErrorEnvelope> {
    if http {
        let command = match action {
            Action::Sync { .. } => Some(ErrorMessage::McpSync),
            Action::StoreClean => Some(ErrorMessage::McpStoreClean),
            _ => None,
        };
        if let Some(command) = command {
            return Some(ErrorEnvelope::mcp_unavailable(command));
        }
        if globals.fresh {
            return Some(ErrorEnvelope::arguments());
        }
    }
    match action {
        Action::Run { request, .. } if replaces(request) => Some(ErrorEnvelope::arguments()),
        _ => None,
    }
}
fn replaces(request: &Request) -> bool {
    match request {
        Request::ArticlePdf(query) => query.replace,
        Request::TrialDocument(query) => query.replace,
        Request::ArticleText(query) => query.replace(),
        Request::ArticleSupplement(query) => query.replace(),
        _ => false,
    }
}
