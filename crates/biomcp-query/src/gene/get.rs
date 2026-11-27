use super::{next, resolve, sections};
use crate::{Ctx, Outcome, status};
use biodata::{Gene, HgncId, Text};
use biomcp_adapters::{clinvar::GENE_COUNTS, hgnc, mygene};
use biomcp_fetch::Deadline;
use biomcp_types::ReplyBody;
use biomcp_types::{
    GeneCards, GeneGet, GeneItem, Incomplete, Problem, ProblemCode, Source, SourceNotice,
    SourceState, Status,
};
use biomcp_types::{GeneIdentity, GeneResolution};

pub(crate) fn get(cx: &Ctx<'_>, query: &GeneGet) -> Outcome {
    // ClinVar gene counts declare a longer budget than the default; other legs keep their caps.
    let requested = sections::requested(query);
    let budget = if requested.contains(&biomcp_types::GeneSection::ClinVar) {
        cx.fetch.source_budget(&GENE_COUNTS, cx.budget)
    } else {
        cx.budget
    };
    // HGNC, then the sections side by side, all on one command clock (1038).
    let command = cx.fetch.command_deadline(budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(cx.fresh);
    let mut problems = Vec::new();
    let mut hgnc_error = None;
    let mut local = None;
    let mut items: Vec<_> = query
        .input()
        .iter()
        .map(|input| resolve_item(&leg, input, &mut problems, &mut hgnc_error, &mut local))
        .collect();
    let genes = sections::prepare(query, &mut items);
    let mappings = if requested.contains(&biomcp_types::GeneSection::Pathways) {
        sections::pathway_mappings(&leg, &genes)
    } else {
        Vec::new()
    };
    for error in mappings
        .iter()
        .filter_map(|(_, keys)| keys.as_ref().err().copied())
    {
        hgnc_error = hgnc_error.or(Some(error));
        local = local.or_else(|| status::local(error));
    }
    let hgnc = status::source(Source::Hgnc, leg.finish(), hgnc_error);
    if hgnc.state == SourceState::Stale {
        for item in &mut items {
            if item.status == Status::Complete {
                item.status = crate::status::item_status(true, false, true);
            }
        }
    }
    let mut sources = vec![hgnc];
    let mut jobs = sections::jobs(cx, deadline, query, &genes, &mappings);
    if !query.section_only()
        && let Some(job) = mygene_job(cx, deadline, &genes)
    {
        jobs.insert(0, job);
    }
    let mut incomplete = Vec::new();
    for job in crate::fan_out::run(jobs) {
        match job {
            Job::MyGene(batches, report) => incomplete.extend(mygene_join(
                &mut items,
                batches,
                report,
                &mut sources,
                &mut problems,
                &mut local,
            )),
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
    let next = next::get(query, &items, &sources, &mut problems);
    if query.section_only() {
        retain_identities(&mut items);
    }
    let mut out = crate::status::finish(
        [state],
        crate::status::Parts {
            dropped: false,
            next,
            body: (GeneCards {
                items: items.into(),
            })
            .with_query(query.clone()),
            page: None,
            sources,
            problems,
            local,
        },
    );
    out.incomplete = incomplete;
    out
}
fn mygene_job<'a>(
    cx: &'a Ctx<'a>,
    deadline: Deadline<'a>,
    genes: &[(HgncId, Gene)],
) -> Option<SourceJob<'a>> {
    let ids: Vec<_> = genes
        .iter()
        .filter_map(|(_, gene)| gene.entrez_id)
        .collect();
    if ids.is_empty() {
        return None;
    }
    Some(Box::new(move || {
        let leg = cx.fetch.leg(&mygene::SPEC, deadline).fresh(cx.fresh);
        let batches = mygene::genes(&leg, &ids);
        Job::MyGene(batches, leg.finish())
    }))
}

fn retain_identities(items: &mut [GeneItem]) {
    for item in items {
        if let Some(GeneResolution::Resolved { gene, r#match }) = &item.answer {
            item.answer = Some(GeneResolution::ResolvedIdentity {
                gene: GeneIdentity {
                    hgnc_id: gene.hgnc_id,
                    symbol: gene.symbol.clone(),
                },
                r#match: r#match.clone(),
            });
        }
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
        if extra.refusals == biomcp_adapters::Refusal::Refused {
            item.status = crate::status::item_status(true, false, true);
        }
    } else {
        item.status = crate::status::item_status(true, false, true);
        status::append(
            problems,
            [Problem {
                source: Source::MyGene,
                record: Text::new(&id.to_string()).ok(),
                field: "hgnc_id".into(),
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
                crate::status::item_status(false, false, false)
            } else {
                crate::status::item_status(
                    true,
                    false,
                    mapped.refusals == biomcp_adapters::Refusal::Refused,
                )
            };
            (Some(mapped.value), state)
        }
        Err(failure) => {
            *error = Some(failure);
            *local = local.or(status::local(failure));
            (None, crate::status::item_status(false, true, false))
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

pub(super) type SourceJob<'a> = Box<dyn FnOnce() -> Job + Send + 'a>;

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
) -> Vec<Incomplete> {
    let mut error = None;
    let mut missing = Vec::new();
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
                        item.status = crate::status::item_status(true, false, true);
                    }
                }
                missing.push((batch.ids, status::reason(failure)));
            }
        }
    }
    let mut source = status::source(Source::MyGene, report, error);
    if source.state == SourceState::Stale {
        for item in items {
            if matches!(&item.answer, Some(GeneResolution::Resolved { gene, .. }) if gene.entrez_id.is_some())
            {
                item.status = crate::status::item_status(true, false, true);
            }
        }
    }
    let mut incomplete = Vec::new();
    for (ids, reason) in missing {
        let at = Text::new(&format!(
            "/sources/{}/notices/{}",
            sources.len(),
            source.notices.len()
        ))
        .unwrap_or_else(|_| biodata::text!("/sources"));
        let ids = ids
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(", ");
        source.notices.push(SourceNotice::RequestGuidance {
            data_date: None,
            message: format!(
                "MyGene did not supply gene summaries or genomic spans for NCBI Gene IDs {ids}."
            )
            .into(),
            retry_after_seconds: None,
        });
        incomplete.push(Incomplete {
            at,
            source: Source::MyGene,
            reason,
            code: None,
        });
    }
    sources.push(source);
    incomplete
}
