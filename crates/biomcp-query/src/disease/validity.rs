//! Page independent ClinGen and GenCC disease validity assertions.
use super::page::{empty, page_info, run, slice};
use crate::{Action, Bundle, Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::{CurationMatches, clingen, gencc, received_date};
use biomcp_fetch::{Deadline, Leg, RequestError, SourceSpec};
use biomcp_types::{
    DiseaseCuration, DiseaseCurationCoverage, DiseaseGet, DiseaseId, DiseaseValidity, ReplyBody,
    Request, Source, SourceOutcome, SourceState, StepKind,
};

struct Publisher {
    bundle: Bundle,
    spec: &'static SourceSpec,
    read: fn(&Leg<'_>, &DiseaseId) -> Result<CurationMatches, RequestError>,
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
    let jobs = PUBLISHERS
        .iter()
        .filter(|publisher| selected.contains(&publisher.bundle.source()))
        .map(|publisher| {
            Box::new(move || publisher.fetch(cx, query.id(), deadline))
                as Box<dyn FnOnce() -> ReadResult + Send>
        })
        .collect();

    let mut rows = Vec::new();
    let mut coverage = Vec::new();
    let mut out = empty(body(query, Vec::new(), Vec::new()));
    for (bundle, mut source, result) in crate::fan_out::run(jobs) {
        match result {
            Ok(result) => {
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
                if source.state == SourceState::Syncing {
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
    let offset = query.section_offset().saturating_add(returned);
    if returned > 0 && u64::from(offset) < total {
        let available: Vec<_> = selected
            .iter()
            .copied()
            .filter(|source| !cx.fetch.disabled(*source))
            .collect();
        out.next.push(crate::next::Step::new(
            StepKind::Page,
            &available,
            run(Request::GetDisease(query.at_section_offset(offset))),
        ));
    }
    out.page = Some(page_info(query.section_offset(), returned, total));
    out.body = body(query, items, coverage);
    status::search(&mut out);
    command.finish(deadline.remaining());
    out
}

impl Publisher {
    fn fetch(&self, cx: &Ctx<'_>, id: &DiseaseId, deadline: Deadline<'_>) -> ReadResult {
        let leg = cx.fetch.leg(self.spec, deadline).fresh(cx.fresh);
        let result = (self.read)(&leg, id);
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
