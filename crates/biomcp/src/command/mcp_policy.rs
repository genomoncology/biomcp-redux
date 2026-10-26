//! Transport authority is checked on parsed commands before fetch or save work.
use biomcp_query::Action;
use biomcp_types::Request;

pub(super) fn allows(action: &Action, globals: &crate::grammar::Globals, http: bool) -> bool {
    if http && (globals.fresh || matches!(action, Action::Sync { .. } | Action::StoreClean)) {
        return false;
    }
    match action {
        Action::Run { request, .. } => !replaces(request),
        _ => true,
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
