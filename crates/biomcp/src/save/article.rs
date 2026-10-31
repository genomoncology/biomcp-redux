//! Article payloads reuse the single binary-owned destination and atomic write boundary.
use super::SaveTarget;
use biodata::Text;
use biomcp_query::{Ctx, Outcome, article};
use biomcp_types::{ArticleSaved, QueryBody, Section};

pub(crate) fn text(cx: &Ctx<'_>, query: &biomcp_types::ArticleText, allow: bool) -> Outcome {
    let filename = format!("{}.md", query.input());
    let target = target(query.save(), &filename, query.replace(), allow);
    let download = article::text::download(cx, query, target.is_some());
    finish(download, target)
}
pub(crate) fn supplement(
    cx: &Ctx<'_>,
    query: &biomcp_types::ArticleSupplement,
    allow: bool,
) -> Outcome {
    let target = target(query.save(), query.file().as_str(), query.replace(), allow);
    let download = article::supplement::download(cx, query, target.is_some());
    finish(download, target)
}
fn target(path: Option<&Text>, filename: &str, replace: bool, allow: bool) -> Option<SaveTarget> {
    allow
        .then(|| SaveTarget::new(path?.as_str(), filename, replace).ok())
        .flatten()
}
fn finish(download: article::text::Download, target: Option<SaveTarget>) -> Outcome {
    let mut out = download.out;
    let url = match &out.body {
        QueryBody::ArticleText(body) => match &body.content {
            Section::Data { data, .. } => data.url.clone(),
            _ => None,
        },
        QueryBody::ArticleSupplement(body) => match &body.content {
            Section::Data { data, .. } => data.url.clone(),
            _ => None,
        },
        _ => None,
    };
    if let Some(file) = download.file {
        let saved = target.and_then(|target| target.write(&file.bytes).ok());
        let receipt = saved.and_then(|saved| {
            Some(ArticleSaved {
                path: Text::new(saved.path.to_str()?).ok()?,
                bytes: saved.bytes,
                sha256: Text::new(&saved.sha256).ok()?,
                url: file.url.clone(),
                parts: file.parts,
            })
        });
        match (receipt, &mut out.body) {
            (Some(saved), QueryBody::ArticleText(body)) => {
                if let Section::Data { data, .. } = &mut body.content {
                    data.saved = Some(saved);
                }
            }
            (Some(saved), QueryBody::ArticleSupplement(body)) => {
                if let Section::Data { data, .. } = &mut body.content {
                    data.saved = Some(saved);
                }
            }
            _ => out.body = QueryBody::ArticleSaveRefused(file.url),
        }
    } else if let Some(url) = url {
        out.body = QueryBody::ArticleSaveRefused(url);
    }
    out
}
