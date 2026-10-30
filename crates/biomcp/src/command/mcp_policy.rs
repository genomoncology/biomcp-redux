//! Transport authority is checked on parsed commands before fetch or save work.
use biomcp_query::Action;
use biomcp_types::{ErrorEnvelope, ErrorMessage, Request};

pub(super) fn refusal(parsed: &crate::grammar::Parsed, http: bool) -> Option<ErrorEnvelope> {
    let globals = &parsed.globals;
    let action = parsed.action.as_ref();
    if globals.hosted
        && (parsed.key.is_some()
            || globals.fresh
            || matches!(
                action,
                Some(Action::Health | Action::Sync { .. } | Action::StoreClean)
            )
            || matches!(action, Some(Action::Run { request, .. }) if saves(request)))
    {
        return Some(ErrorEnvelope::mcp_unavailable(ErrorMessage::McpHosted));
    }
    if parsed.key.is_some() {
        return Some(ErrorEnvelope::mcp_unavailable(ErrorMessage::McpKey));
    }
    let action = action?;
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

fn saves(request: &Request) -> bool {
    match request {
        Request::ArticlePdf(_) | Request::TrialDocument(_) => true,
        Request::ArticleText(query) => query.save().is_some(),
        Request::ArticleSupplement(query) => query.save().is_some(),
        _ => false,
    }
}
