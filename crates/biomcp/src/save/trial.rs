//! Trial downloads share the single destination check and atomic file writer.
use super::SaveTarget;
use biomcp_query::{Ctx, Outcome};
use biomcp_types::TrialDocumentGet;
pub(crate) fn document(
    cx: &Ctx<'_>,
    query: &TrialDocumentGet,
    path: &biomcp_types::Text,
    allow: bool,
) -> Outcome {
    let target = allow
        .then(|| SaveTarget::new(path.as_str(), query.file.as_str(), query.replace).ok())
        .flatten();
    let download = biomcp_query::trial::document::download(cx, query, target.is_some());
    let mut out = download.out;
    out.body = out.body.visit(super::reply::SaveReply {
        target,
        trial: download.file,
        ..Default::default()
    });
    out
}
