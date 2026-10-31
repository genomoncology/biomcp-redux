//! Trial downloads share the single destination check and atomic file writer.
use super::SaveTarget;
use biodata::Text;
use biomcp_query::{Ctx, Outcome};
use biomcp_types::{QueryBody, Section, TrialDocumentGet, TrialDocumentResult, TrialDocumentSaved};
pub(crate) fn document(cx: &Ctx<'_>, query: &TrialDocumentGet, allow: bool) -> Outcome {
    let target = allow
        .then(|| SaveTarget::new(query.save.as_str(), query.file.as_str(), query.replace).ok())
        .flatten();
    let download = biomcp_query::trial::document::download(cx, query, target.is_some());
    let mut out = download.out;
    if let Some((document, bytes)) = download.file {
        let saved = target.and_then(|t| t.write(&bytes).ok()).and_then(|s| {
            Some((
                Text::new(s.path.to_str()?).ok()?,
                s.bytes,
                Text::new(&s.sha256).ok()?,
            ))
        });
        out.body = match saved {
            Some((path, bytes, sha256)) => QueryBody::TrialDocument(TrialDocumentResult {
                file: Section::Data {
                    source: biomcp_types::Source::ClinicalTrialsGov,
                    data: TrialDocumentSaved {
                        path,
                        bytes,
                        sha256,
                        document,
                    },
                },
            }),
            None => QueryBody::Usage(biomcp_types::UsageError::ArticleSaveRefused(document.url)),
        };
    }
    out
}
