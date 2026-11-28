//! Page independent ClinGen and GenCC disease validity assertions.
use super::page::{empty, page_info, run, slice};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::{CurationLookup, CurationMatches, clingen, gencc, received_date};
use biomcp_fetch::{Deadline, Leg, RequestError, SourceSpec};
use biomcp_types::{
    DiseaseCuration, DiseaseCurationCoverage, DiseaseGet, DiseaseId, DiseaseValidity, Reason,
    ReplyBody, Request, Source, SourceNotice, SourceOutcome, SourceState, SourceStatus, StepKind,
};

struct Publisher {
    bundle: Bundle,
    spec: &'static SourceSpec,
    read: fn(&Leg<'_>, CurationLookup<'_>) -> Result<CurationMatches, RequestError>,
}
type ReadResult = (Bundle, SourceOutcome, Result<CurationMatches, RequestError>);

static PUBLISHERS: [Publisher; 2] = [
    Publisher {
        bundle: Bundle::CLIN_GEN,
        spec: &clingen::SPEC,
        read: clingen::by_disease,
    },
    Publisher {
        bundle: Bundle::GENCC,
        spec: &gencc::SPEC,
        read: gencc::by_disease,
    },
];

pub(super) fn get(cx: &Ctx<'_>, query: &DiseaseGet, selected: &[Source]) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let mut out = empty(body(query, Vec::new(), Vec::new()));
    let resolved = resolve(cx, query, selected, deadline, &mut out);
    // A checked MONDO mapping takes precedence so that both identifiers page the same rows.
    let lookup = query.source_title_prefix().map_or(
        CurationLookup::Exact(resolved.as_ref().unwrap_or_else(|| query.id())),
        CurationLookup::TitlePrefix,
    );
    let jobs = PUBLISHERS
        .iter()
        .filter(|publisher| selected.contains(&publisher.bundle.source()))
        .map(|publisher| {
            Box::new(move || publisher.fetch(cx, lookup, deadline))
                as Box<dyn FnOnce() -> ReadResult + Send>
        })
        .collect();

    let mut rows = Vec::new();
    let mut coverage = Vec::new();
    for (bundle, mut source, result) in crate::fan_out::run(jobs) {
        match result {
            Ok(result) => {
                source.notices.push(guidance(
                    "Coverage excludes curations for descendant diseases.".into(),
                ));
                let matched = result.rows.value.len() as u64;
                let before = rows.len();
                rows.extend(
                    result
                        .rows
                        .value
                        .into_iter()
                        .filter(|row| matches(query, row)),
                );
                source.count = Some((rows.len() - before) as u64);
                out.include_refusals(result.rows.refusals);
                out.problems.extend(result.rows.problems);
                coverage.push(DiseaseCurationCoverage {
                    source: source.source,
                    export_rows: result.snapshot.rows,
                    matched,
                    retrieved: received_date(result.snapshot.retrieved_at),
                });
                if !result.snapshot.fresh {
                    refresh(&mut out, bundle);
                }
            }
            Err(error) => {
                out.local = out.local.or(status::local(error));
                if crate::next::wants_sync(&source) {
                    refresh(&mut out, bundle);
                }
            }
        }
        out.sources.push(source);
    }
    let (items, total) = slice(rows, query);
    for source in &mut out.sources {
        if source.count.is_some() {
            source.retrieved_count = Some(
                items
                    .iter()
                    .filter(|row| source.source == source_of(row))
                    .count() as u32,
            );
        }
    }
    let returned = items.len() as u32;
    out.page = Some(page_info(query.section_offset(), returned, total));
    out.next
        .extend(next_page(cx, query, selected, returned, total));
    out.body = body(query, items, coverage);
    status::search_for(&mut out, selected);
    command.finish(deadline.remaining());
    out
}

fn next_page(
    cx: &Ctx<'_>,
    query: &DiseaseGet,
    selected: &[Source],
    returned: u32,
    total: u64,
) -> Option<crate::next::Step> {
    let offset = query.section_offset().saturating_add(returned);
    if returned == 0 || u64::from(offset) >= total {
        return None;
    }
    let available: Vec<_> = selected
        .iter()
        .copied()
        .filter(|source| !cx.fetch.disabled(*source))
        .collect();
    Some(crate::next::Step::new(
        StepKind::Page,
        &available,
        run(Request::GetDisease(query.at_section_offset(offset))),
    ))
}

fn resolve(
    cx: &Ctx<'_>,
    query: &DiseaseGet,
    selected: &[Source],
    deadline: Deadline<'_>,
    out: &mut Outcome,
) -> Option<DiseaseId> {
    let DiseaseId::Concept(id) = query.id() else {
        return None;
    };
    if !gencc::has_submitted_disease_namespace(id.as_str())
        || selected.iter().all(|source| cx.fetch.disabled(*source))
    {
        return None;
    }
    let resolved = super::mydisease::mondo(cx, query, id, deadline, out);
    let source = out.sources.last_mut()?;
    if source.reason == Some(Reason::NotFound) {
        source.set_status(SourceStatus::Ok);
    }
    if resolved.is_none() {
        source.notices.push(guidance(format!(
            "No MONDO mapping was available for {id}. Coverage is limited to the requested identifier."
        )));
    }
    out.dropped |= source.state != SourceState::Ok;
    resolved.map(DiseaseId::Concept)
}

fn guidance(message: String) -> SourceNotice {
    SourceNotice::RequestGuidance {
        data_date: None,
        message: message.into(),
        retry_after_seconds: None,
    }
}

impl Publisher {
    fn fetch(
        &self,
        cx: &Ctx<'_>,
        lookup: CurationLookup<'_>,
        deadline: Deadline<'_>,
    ) -> ReadResult {
        let leg = cx.fetch.leg(self.spec, deadline).fresh(cx.fresh);
        let result = (self.read)(&leg, lookup);
        let source = status::source(
            self.bundle.source(),
            leg.finish(),
            result.as_ref().err().copied(),
        );
        (self.bundle, source, result)
    }
}

fn refresh(out: &mut Outcome, bundle: Bundle) {
    out.next.push(crate::next::Step::new(
        StepKind::Sync,
        &[bundle.source()],
        Action::Sync { bundle },
    ));
}

fn matches(query: &DiseaseGet, row: &DiseaseCuration) -> bool {
    let (classification, submitter) = match row {
        DiseaseCuration::ClinGen { assertion: row, .. } => {
            (row.classification.as_ref(), row.expert_panel.as_ref())
        }
        DiseaseCuration::Gencc { assertion: row, .. } => {
            (row.classification.as_ref(), row.submitter.as_ref())
        }
    };
    same(query.classification(), classification) && same(query.submitter(), submitter)
}

fn same(requested: Option<&Text>, stated: Option<&Text>) -> bool {
    requested.is_none_or(|requested| {
        stated.is_some_and(|stated| requested.as_str().eq_ignore_ascii_case(stated.as_str()))
    })
}

fn source_of(row: &DiseaseCuration) -> Source {
    match row {
        DiseaseCuration::ClinGen { .. } => Source::ClinGen,
        DiseaseCuration::Gencc { .. } => Source::Gencc,
    }
}

fn body(
    query: &DiseaseGet,
    items: Vec<DiseaseCuration>,
    coverage: Vec<DiseaseCurationCoverage>,
) -> biomcp_types::ReplyData {
    DiseaseValidity {
        items: items.into(),
        coverage: coverage.into(),
    }
    .with_query(query.clone())
}
