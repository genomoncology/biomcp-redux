use crate::{Action, Ctx, Outcome, status};
use biodata::Pmcid;
use biomcp_adapters::pmc_oa;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::ArticleInput;
use biomcp_types::{
    ArticleResolution, ArticleSupplement, ArticleText, FullText, Reason, Request, Section, Source,
    Status, StepKind, Supplements,
};
type Identity = (Pmcid, Option<biodata::Pmid>);
type Content = Result<Option<(FullText, Supplements)>, Reason>;
/// Load one document for both sections, preserving a single source request sequence.
pub(super) fn run<'a>(
    cx: &Ctx<'a>,
    deadline: &mut Deadline<'a>,
    out: &mut Outcome,
    fulltext: bool,
    supplements: bool,
) {
    let biomcp_types::QueryBody::ArticleCards(cards) = &mut out.body else {
        return;
    };
    let leg = cx.fetch.leg(&pmc_oa::SPEC, *deadline).fresh(cx.fresh);
    let started = leg.remaining();
    let mut error = None;
    let mut loaded: Vec<(Identity, Content)> = Vec::new();
    for item in &mut cards.items {
        let Some(ArticleResolution::Resolved { card, .. }) = &item.answer else {
            continue;
        };
        let Some(sections) = &mut item.sections else {
            continue;
        };
        if card.europepmc_id.is_some() {
            continue;
        }
        let Some(id) = card.pmcid else {
            continue;
        };
        let identity = (id, card.pmid);
        if !loaded.iter().any(|(found, _)| *found == identity) {
            let result = pmc_oa::read(&leg, id, card.pmid).map(|mapped| {
                out.problems.extend(mapped.problems);
                mapped.value
            });
            if let Err(e) = result {
                error.get_or_insert(e);
                out.local = out.local.or(status::local(e));
            }
            if let Ok(Some(doc)) = &result {
                out.problems.extend(doc.problems.clone());
            }
            loaded.push((identity, content(result)));
        }
        let Some((_, data)) = loaded.iter().find(|(found, _)| *found == identity) else {
            continue;
        };
        if data.is_err() || matches!(data, Ok(None)) {
            item.status = Status::Partial;
        }
        if fulltext && sections.fulltext.is_none() {
            sections.fulltext = Some(section(data, |(text, _)| text.clone()));
            if matches!(data,Ok(Some((text,_))) if !text.parts.is_empty()) && out.next.len() < 5 {
                next(id, 1, &mut out.next);
            }
        }
        if supplements && sections.supplements.is_none() {
            sections.supplements = Some(section(data, |(_, files)| files.clone()));
            if let Ok(Some((_, files))) = data {
                supplement_next(id, files, &mut out.next);
            }
        }
    }
    *deadline = deadline.after_stage(started.saturating_sub(leg.remaining()));
    let source = status::source(Source::PmcOa, leg.finish(), error);
    if !loaded.is_empty() {
        if source.state == biomcp_types::SourceState::Stale {
            for item in &mut cards.items {
                if item.sections.as_ref().is_some_and(|s| {
                    matches!(s.fulltext, Some(Section::Data { .. }))
                        || matches!(s.supplements, Some(Section::Data { .. }))
                }) {
                    item.status = Status::Partial;
                }
            }
        }
        out.sources.push(source);
    }
    let state = status::status(cards.items.iter().map(|i| i.status));
    status::get(out, [state].into_iter());
}
fn content(result: Result<Option<pmc_oa::Document>, RequestError>) -> Content {
    result
        .map(|doc| {
            doc.map(|doc| {
                let text = outline(&doc);
                let files = Supplements {
                    license: doc.license,
                    files: doc.supplements.into_iter().map(|v| v.file).collect(),
                };
                (text, files)
            })
        })
        .map_err(|e| crate::status::reason(e).unwrap_or(biomcp_types::Reason::Unavailable))
}
fn section<T>(content: &Content, map: impl FnOnce(&(FullText, Supplements)) -> T) -> Section<T> {
    match content {
        Ok(Some(data)) => Section::Data {
            source: Source::PmcOa,
            data: map(data),
        },
        Ok(None) => Section::Unavailable {
            source: Source::PmcOa,
            reason: Reason::NotOpen,
        },
        Err(reason) => Section::unavailable(Source::PmcOa, *reason),
    }
}
fn supplement_next(id: Pmcid, files: &Supplements, out: &mut Vec<(StepKind, Action)>) {
    if let Some(file) = files.files.iter().find(|v| v.kind.readable())
        && out.len() < 5
        && let Ok(query) = ArticleSupplement::new(ArticleInput::Pmcid(id), file.name.clone())
    {
        out.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleSupplement(query),
                fresh: false,
            },
        ));
    }
}
pub(crate) fn outline(document: &pmc_oa::Document) -> FullText {
    FullText {
        license: document.license,
        processing_version: document.version,
        manuscript: document.manuscript,
        parts: document.body.parts.iter().map(|p| p.meta.clone()).collect(),
        figures: document.body.figures.clone().into(),
    }
}
pub(crate) fn next(id: Pmcid, part: u32, out: &mut Vec<(StepKind, Action)>) {
    next_input(ArticleInput::Pmcid(id), part, out);
}
pub(crate) fn next_input(input: ArticleInput, part: u32, out: &mut Vec<(StepKind, Action)>) {
    if let Ok(query) = ArticleText::part(input, part) {
        out.push((
            StepKind::Page,
            Action::Run {
                request: Request::ArticleText(query),
                fresh: false,
            },
        ));
    }
}
