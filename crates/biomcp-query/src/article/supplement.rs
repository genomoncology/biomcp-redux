//! Confined supplement download, local parsing and bounded row selection.
mod page;
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    ArticleInput, ArticleSupplement, ArticleSupplementResult, QueryBody, Reason, Request, Section,
    Source, Status, StepKind, SupplementContent, Text,
};
use biomcp_adapters::pmc_oa;
use biomcp_fetch::{Deadline, RequestError, SourceError};
use sha2::{Digest, Sha256};
/// Read only a file named by both JATS and its owning bucket record.
pub fn read(cx: &Ctx<'_>, query: &ArticleSupplement) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let mut out = Outcome {
        status: Status::Complete,
        body: QueryBody::ArticleSupplement(ArticleSupplementResult {
            pmcid: None,
            content: Section::NotApplicable,
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    };
    let Ok(selection) = super::protein_change::Selection::new(query.find(), query.variant()) else {
        out.body = QueryBody::TextPositionError;
        return out;
    };
    let result = execute(cx, query, &selection, &mut deadline, &mut out);
    command.finish(deadline.remaining());
    if let Err(error) = result {
        out.local = out.local.or(status::local(error));
        out.status = Status::Failed;
        if let QueryBody::ArticleSupplement(body) = &mut out.body {
            body.content = Section::Unavailable {
                source: Source::PmcOa,
                reason: match error {
                    RequestError::Source(e) => e.reason,
                    _ => Reason::Unavailable,
                },
            };
        }
        out.next.push((
            StepKind::Fresh,
            Action::Run {
                request: Request::ArticleSupplement(query.clone()),
                fresh: true,
            },
        ));
    }
    out
}
fn failure(reason: Reason) -> RequestError {
    SourceError {
        source: Source::PmcOa,
        reason,
        status: None,
        retry_after: None,
    }
    .into()
}
fn execute<'a>(
    cx: &Ctx<'a>,
    query: &ArticleSupplement,
    selection: &super::protein_change::Selection,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let Some(id) = super::text::resolve(cx, query.input(), deadline, out)? else {
        return Ok(());
    };
    let leg = cx.fetch.leg(&pmc_oa::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let pmid = match query.input() {
        ArticleInput::Pmid(v) => Some(*v),
        _ => None,
    };
    let result = (|| {
        let Some(document) = pmc_oa::read(&leg, id, pmid)? else {
            return Ok(Section::Unavailable {
                source: Source::PmcOa,
                reason: Reason::NotOpen,
            });
        };
        let Some(file) = document
            .supplements
            .iter()
            .find(|v| v.file.name == *query.file())
        else {
            out.body = QueryBody::TextPositionError;
            return Ok(Section::NotApplicable);
        };
        let mut content = SupplementContent {
            license: document.license,
            sha256: None,
            url: None,
            reason: None,
            sheets: None,
            rows: None,
            forms: selection.forms.clone(),
        };
        if !file.file.kind.readable() {
            content.url = Some(file.url.clone());
            content.reason = Some(Reason::Unsupported);
        } else {
            let local_end = std::time::Instant::now() + leg.remaining();
            let (bytes, sheets) = pmc_oa::download(&leg, file)?;
            content.sha256 = Some(
                Text::new(
                    &Sha256::digest(&bytes)
                        .iter()
                        .map(|v| format!("{v:02x}"))
                        .collect::<String>(),
                )
                .map_err(|_| failure(Reason::BadReply))?,
            );
            if !page::select(
                query,
                &sheets,
                &mut content,
                out,
                &selection.needles,
                local_end,
            )
            .map_err(failure)?
            {
                out.body = QueryBody::TextPositionError;
                return Ok(Section::NotApplicable);
            }
        }
        Ok(Section::Data {
            source: Source::PmcOa,
            data: content,
        })
    })();
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::PmcOa, leg.finish(), result.as_ref().err().copied());
    if source.state == biodata::SourceState::Stale {
        out.status = Status::Partial;
    }
    out.sources.push(source);
    let content = result?;
    if !matches!(out.body, QueryBody::TextPositionError) {
        out.body = QueryBody::ArticleSupplement(ArticleSupplementResult {
            pmcid: Some(id),
            content,
        });
    }
    Ok(())
}
