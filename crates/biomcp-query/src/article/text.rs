//! Bounded body selection under one command budget, including PMID resolution.
use crate::{Action, Ctx, Outcome, status};
use biodata::{
    ArticleInput, ArticleText, ArticleTextResult, PageInfo, Pmcid, QueryBody, Reason, Request,
    Section, Source, Status, StepKind, Text, TextContent, TextHit,
};
use biomcp_adapters::{jats, pmc_oa, pubmed};
use biomcp_fetch::{Deadline, RequestError};
/// Read a bounded part or find page through the same transport and command budget.
pub fn read(cx: &Ctx<'_>, query: &ArticleText) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let mut deadline = command.deadline();
    let mut out = Outcome {
        status: Status::Complete,
        body: QueryBody::ArticleText(ArticleTextResult {
            pmcid: None,
            content: Section::NotApplicable,
        }),
        page: None,
        sources: Vec::new(),
        problems: Vec::new(),
        next: Vec::new(),
        local: None,
    };
    let result = execute(cx, query, &mut deadline, &mut out);
    command.finish(deadline.remaining());
    if let Err(error) = result {
        out.local = out.local.or(status::local(error));
        out.status = Status::Failed;
        if let QueryBody::ArticleText(body) = &mut out.body {
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
                request: Request::ArticleText(query.clone()),
                fresh: true,
            },
        ));
    }
    out
}
fn resolve<'a>(
    cx: &Ctx<'a>,
    query: &ArticleText,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<Option<Pmcid>, RequestError> {
    match query.input() {
        ArticleInput::Pmcid(id) => Ok(Some(*id)),
        ArticleInput::Pmid(id) => {
            let leg = cx.fetch.leg(&pubmed::SPEC, *deadline).fresh(cx.fresh);
            let started = leg.remaining();
            let result = pubmed::summaries(&leg, &[*id], cx.ncbi_email.as_ref());
            *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
            let source = status::source(Source::PubMed, leg.finish(), result.as_ref().err().copied());
            if source.state == biodata::SourceState::Stale { out.status = Status::Partial; }
            out.sources.push(source);
            let mapped = result?;
            if !mapped.problems.is_empty() { out.status = Status::Partial; }
            status::append(&mut out.problems, mapped.problems);
            let header = mapped.value.into_iter().find(|h| h.pmid == *id).ok_or(biomcp_fetch::SourceError {
                source: Source::PubMed, reason: Reason::BadReply, status: None, retry_after: None,
            })?;
            Ok(header.pmcid)
        }
        ArticleInput::Doi(_) => Ok(None),
    }
}
fn execute<'a>(
    cx: &Ctx<'a>,
    query: &ArticleText,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
) -> Result<(), RequestError> {
    let Some(id) = resolve(cx, query, deadline, out)? else {
        return Ok(());
    };
    let leg = cx.fetch.leg(&pmc_oa::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let pmid = match query.input() {
        ArticleInput::Pmid(id) => Some(*id),
        _ => None,
    };
    let result = pmc_oa::read(&leg, id, pmid);
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::PmcOa, leg.finish(), result.as_ref().err().copied());
    if source.state == biodata::SourceState::Stale {
        out.status = Status::Partial;
    }
    out.sources.push(source);
    let content = match result? {
        None => Section::Unavailable {
            source: Source::PmcOa,
            reason: Reason::NotOpen,
        },
        Some(document) => {
            let Some(content) = select(query, id, &document, out) else {
                out.body = QueryBody::TextPositionError;
                return Ok(());
            };
            Section::Data {
                source: Source::PmcOa,
                data: content,
            }
        }
    };
    out.body = QueryBody::ArticleText(ArticleTextResult {
        pmcid: Some(id),
        content,
    });
    Ok(())
}
fn select(
    query: &ArticleText,
    id: Pmcid,
    document: &pmc_oa::Document,
    out: &mut Outcome,
) -> Option<TextContent> {
    let mut content = TextContent {
        license: document.license,
        processing_version: document.version,
        manuscript: document.manuscript,
        scope: Text::new("PMC body sections and tables").ok()?,
        part: None,
        text: None,
        hits: None,
    };
    if let Some(number) = query.selected_part() {
        let part = document
            .body
            .parts
            .iter()
            .find(|p| p.meta.number == number)?;
        content.part = Some(number);
        content.text = Text::new(
            &part
                .rows
                .iter()
                .map(|r| r.text.as_str())
                .collect::<Vec<_>>()
                .join("\n"),
        )
        .ok();
        if part.continuation {
            super::sections::fulltext::next(id, number.checked_add(1)?, &mut out.next);
        }
    } else {
        content.hits = Some(find(query, document, out)?);
    }
    Some(content)
}

fn find(
    query: &ArticleText,
    document: &pmc_oa::Document,
    out: &mut Outcome,
) -> Option<Box<[TextHit]>> {
    let needle = query.find()?.as_str().to_lowercase();
    let matching: Vec<_> = document
        .body
        .rows
        .iter()
        .map(|r| r.to_lowercase().contains(&needle))
        .collect();
    let hits: Vec<_> = document
        .body
        .parts
        .iter()
        .flat_map(|p| {
            p.rows
                .iter()
                .filter(|row| matching.get(row.source_row).copied().unwrap_or(false))
                .map(|row| (p.meta.number, &row.text))
        })
        .collect();
    let offset = usize::try_from(query.offset()).ok()?;
    if offset > hits.len() {
        return None;
    }
    let mut page = Vec::new();
    let mut size = 0;
    for (part, row) in hits.iter().skip(offset).take(25) {
        let bytes = jats::escaped_bytes(row) + 64;
        if !page.is_empty() && size + bytes > jats::TEXT_BYTES {
            break;
        }
        size += bytes;
        page.push(TextHit {
            part: *part,
            text: Text::new(row).ok()?,
        });
    }
    let returned = u32::try_from(page.len()).ok()?;
    let total = u64::try_from(hits.len()).ok()?;
    let following = query.offset().checked_add(returned)?;
    if returned > 0 && u64::from(following) < total {
        out.next.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleText(query.page(following)),
                fresh: false,
            },
        ));
    }
    out.page = Some(PageInfo {
        offset: query.offset(),
        returned,
        total,
        pool: None,
        truncated: None,
    });
    Some(page.into())
}
