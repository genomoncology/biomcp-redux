use super::{next, resolve, sections};
use crate::{Ctx, Outcome, status};
use biodata::Text;
use biomcp_adapters::{hgnc, mygene};
use biomcp_types::GeneResolution;
use biomcp_types::{
    GeneCards, GeneGet, GeneItem, Problem, ProblemCode, QueryBody, Source, SourceState, Status,
};

pub(crate) fn get(cx: &Ctx<'_>, query: &GeneGet) -> Outcome {
    let deadline = cx.fetch.deadline(cx.budget);
    let leg = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(cx.fresh);
    let mut problems = Vec::new();
    let mut hgnc_error = None;
    let mut local = None;
    let mut items: Vec<_> = query
        .input()
        .iter()
        .map(|input| resolve_item(&leg, input, &mut problems, &mut hgnc_error, &mut local))
        .collect();
    let hgnc = status::source(Source::Hgnc, leg.finish(), hgnc_error);
    if hgnc.state == SourceState::Stale {
        for item in &mut items {
            if item.status == Status::Complete {
                item.status = Status::Partial;
            }
        }
    }
    let mut sources = vec![hgnc];
    let genes = sections::prepare(query, &mut items);
    let ids: Vec<_> = genes
        .iter()
        .filter_map(|(_, gene)| gene.entrez_id)
        .collect();
    let mut jobs = sections::jobs(cx, deadline, query, &genes);
    if !ids.is_empty() {
        jobs.insert(
            0,
            Box::new(|| {
                let leg = cx.fetch.leg(&mygene::SPEC, deadline).fresh(cx.fresh);
                let batches = mygene::genes(&leg, &ids);
                Job::MyGene(batches, leg.finish())
            }),
        );
    }
    for job in crate::fan_out::run(jobs) {
        match job {
            Job::MyGene(batches, report) => mygene_join(
                &mut items,
                batches,
                report,
                &mut sources,
                &mut problems,
                &mut local,
            ),
            Job::Section(result) => {
                sections::join(&mut items, &result);
                sources.push(result.source);
                status::append(&mut problems, result.problems);
                local = local.or(result.local);
            }
        }
    }
    sections::offer_oncokb(query, &mut items, &sources);
    let state = status::status(items.iter().map(|item| item.status));
    let next = next::get(query, &items, &sources);
    Outcome {
        status: state,
        body: QueryBody::GeneCards(GeneCards {
            items: items.into(),
        }),
        page: None,
        sources,
        problems,
        next,
        local,
    }
}
fn join(
    item: &mut GeneItem,
    extras: &[biomcp_adapters::Mapped<mygene::GeneExtras>],
    ids: &[biodata::NcbiGeneId],
    problems: &mut Vec<Problem>,
) {
    let Some(GeneResolution::Resolved { gene, .. }) = &mut item.answer else {
        return;
    };
    let Some(id) = gene.entrez_id.filter(|id| ids.contains(id)) else {
        return;
    };
    let matching = extras.iter().find(|extra| extra.value.query_id == id);
    if let Some(extra) = matching {
        status::append(problems, extra.problems.clone());
    }
    if let Some(extra) = matching
        .filter(|extra| extra.value.hgnc_id == Some(gene.hgnc_id) && extra.value.entrez_id == id)
    {
        gene.summary = extra.value.summary.clone();
        gene.spans = extra.value.spans.clone();
        if !extra.problems.is_empty() {
            item.status = Status::Partial;
        }
    } else {
        item.status = Status::Partial;
        status::append(
            problems,
            [Problem {
                source: Source::MyGene,
                record: Text::new(&id.to_string()).ok(),
                field: "hgnc-id".into(),
                code: ProblemCode::ValueUnknown,
            }],
        );
    }
}

fn resolve_item(
    leg: &biomcp_fetch::Leg<'_>,
    input: &biomcp_types::GeneInput,
    problems: &mut Vec<Problem>,
    error: &mut Option<biomcp_fetch::RequestError>,
    local: &mut Option<biomcp_fetch::RequestError>,
) -> GeneItem {
    let (answer, state) = match resolve(leg, input) {
        Ok(mapped) => {
            status::append(problems, mapped.problems);
            let state = if matches!(mapped.value, GeneResolution::NotFound {}) {
                Status::Absent
            } else {
                Status::Complete
            };
            (Some(mapped.value), state)
        }
        Err(failure) => {
            *error = Some(failure);
            *local = local.or(status::local(failure));
            (None, Status::Failed)
        }
    };
    GeneItem {
        oncokb: None,
        input: input.clone(),
        status: state,
        answer,
        sections: None,
        more: Box::default(),
    }
}

pub(super) enum Job {
    MyGene(Vec<mygene::GeneBatch>, biomcp_fetch::LegReport),
    Section(sections::SectionResult),
}
fn mygene_join(
    items: &mut [GeneItem],
    batches: Vec<mygene::GeneBatch>,
    report: biomcp_fetch::LegReport,
    sources: &mut Vec<biomcp_types::SourceOutcome>,
    problems: &mut Vec<Problem>,
    local: &mut Option<biomcp_fetch::RequestError>,
) {
    let mut error = None;
    for batch in batches {
        status::append(problems, batch.problems);
        match batch.result {
            Ok(extras) => {
                for item in &mut *items {
                    join(item, &extras, &batch.ids, problems);
                }
            }
            Err(failure) => {
                error = Some(failure);
                *local = local.or(status::local(failure));
                for item in &mut *items {
                    if matches!(&item.answer, Some(GeneResolution::Resolved { gene, .. }) if gene.entrez_id.is_some_and(|id| batch.ids.contains(&id)))
                    {
                        item.status = Status::Partial;
                    }
                }
            }
        }
    }
    let source = status::source(Source::MyGene, report, error);
    if source.state == SourceState::Stale {
        for item in items {
            if matches!(&item.answer, Some(GeneResolution::Resolved { gene, .. }) if gene.entrez_id.is_some())
            {
                item.status = Status::Partial;
            }
        }
    }
    sources.push(source);
}
