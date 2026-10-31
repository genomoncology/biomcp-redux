//! Pure typed follow-up requests; source names never become chosen drug identities.
use crate::next::run;
use crate::{Action, Outcome};
use biomcp_types::{
    PgxAnnotations, PgxGet, PgxPageInfo, PgxRecommendations, PgxSection, PgxSource, QueryBody,
    Request, SourceState, Status, StepKind,
};
fn gene_query(
    gene: &biodata::GeneSymbol,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<PgxGet> {
    crate::next::checked(
        PgxGet::new(gene.clone()),
        biomcp_types::Source::Cpic,
        problems,
    )
}
fn section(
    q: &PgxGet,
    s: PgxSection,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Option<Request> {
    crate::next::checked(
        q.with_section(s, PgxSource::Cpic),
        biomcp_types::Source::Cpic,
        problems,
    )
    .map(Request::GetPgx)
}
pub(super) fn steps(request: &Request, out: &mut Outcome) -> Vec<crate::next::Step> {
    steps_with(request, out, None)
}
pub(super) fn steps_with(
    request: &Request,
    out: &mut Outcome,
    native_consumed: Option<u32>,
) -> Vec<crate::next::Step> {
    let mut next = vec![];
    let association = matches!(
        &out.body,
        QueryBody::PgxGuidelines(_) | QueryBody::PgxAnnotations(_)
    );
    if out.status() == Status::Failed {
        if !diplotype(request) && out.sources.iter().any(|s| s.state != SourceState::Skipped) {
            next.push(crate::next::Step::new(
                StepKind::Fresh,
                &out.sources
                    .iter()
                    .map(|source| source.source)
                    .collect::<Vec<_>>(),
                Action::Run {
                    request: request.clone(),
                    fresh: true,
                },
            ));
        }
        return next;
    }
    let body = out.body.clone();
    match (request, &body) {
        (Request::SearchPgx(q), QueryBody::PgxPage(body)) => {
            let consumed = native_consumed.unwrap_or(body.page.returned);
            let offset = body.page.offset.saturating_add(consumed);
            if consumed > 0 && u64::from(offset) < body.page.total {
                next.push(crate::next::Step::new(
                    StepKind::Page,
                    &[biomcp_types::Source::Cpic],
                    run(Request::SearchPgx(q.at_offset(offset))),
                ));
            }
            if let Some(card) = gene_query(q.gene(), &mut out.problems) {
                next.push(crate::next::Step::new(
                    StepKind::Section,
                    &[biomcp_types::Source::Cpic],
                    run(Request::GetPgx(card.clone())),
                ));
                if let Some(r) = section(&card, PgxSection::Recommendations, &mut out.problems) {
                    next.push(crate::next::Step::new(
                        StepKind::Section,
                        &[biomcp_types::Source::Cpic],
                        run(r),
                    ));
                }
            }
        }
        (Request::GetPgx(q), QueryBody::PgxCard(Some(_))) => {
            card_steps(q, &mut next, &mut out.problems)
        }
        (Request::GetPgx(q), QueryBody::PgxAnnotations(body)) => {
            annotation_steps(q, body, &mut next, &mut out.problems)
        }
        (
            Request::GetPgx(q),
            QueryBody::PgxRecommendations(
                PgxRecommendations::View { page, .. } | PgxRecommendations::Table { page, .. },
            ),
        ) => page_step(q, page, &mut next, native_consumed, &mut out.problems),
        (Request::GetPgx(q), QueryBody::PgxGuidelines(body)) => {
            let mut seen = std::collections::BTreeSet::new();
            for gene in body.items.iter().flat_map(|r| r.genes.iter()) {
                if gene.symbol == *q.gene() || !seen.insert(gene.symbol.to_string()) {
                    continue;
                }
                if let Some(query) = crate::next::checked(
                    gene.symbol
                        .as_str()
                        .parse()
                        .map_err(|_| biomcp_types::QueryInvalid::new(&["input"]))
                        .and_then(|input| biomcp_types::GeneGet::new(vec![input].into())),
                    biomcp_types::Source::Hgnc,
                    &mut out.problems,
                ) {
                    next.push(crate::next::Step::new(
                        StepKind::Pivot,
                        &[biomcp_types::Source::Hgnc],
                        run(Request::GetGene(query)),
                    ));
                }
            }
            page_step(q, &body.page, &mut next, native_consumed, &mut out.problems);
        }
        (Request::GetPgx(q), QueryBody::PgxDiplotype(_)) => {
            if let Some(card) = gene_query(q.gene(), &mut out.problems)
                && let Some(r) = section(&card, PgxSection::Recommendations, &mut out.problems)
            {
                next.push(crate::next::Step::new(
                    StepKind::Section,
                    &[biomcp_types::Source::Cpic],
                    run(r),
                ));
            }
        }
        _ => {}
    }
    if out.status() == Status::Partial && !diplotype(request) {
        next.push(crate::next::Step::new(
            StepKind::Fresh,
            &out.sources
                .iter()
                .map(|source| source.source)
                .collect::<Vec<_>>(),
            Action::Run {
                request: request.clone(),
                fresh: true,
            },
        ));
    }
    if !association {
        next.truncate(5);
    }
    next
}
fn page_step(
    q: &PgxGet,
    p: &PgxPageInfo,
    next: &mut Vec<crate::next::Step>,
    consumed: Option<u32>,
    problems: &mut Vec<biomcp_types::Problem>,
) {
    let consumed = consumed.unwrap_or(p.returned);
    let offset = p.offset.saturating_add(consumed);
    if consumed > 0
        && u64::from(offset) < p.total
        && let Some(query) =
            crate::next::checked(q.at_section_offset(offset), q.source().source(), problems)
    {
        next.push(crate::next::Step::new(
            StepKind::Page,
            &[q.source().source()],
            run(Request::GetPgx(query)),
        ));
    }
}

fn diplotype(request: &Request) -> bool {
    matches!(request,Request::GetPgx(q) if q.section()==Some(PgxSection::Diplotype))
}

fn annotation_steps(
    q: &PgxGet,
    body: &PgxAnnotations,
    next: &mut Vec<crate::next::Step>,
    problems: &mut Vec<biomcp_types::Problem>,
) {
    match body {
        PgxAnnotations::Clinical { items, page, .. } => {
            for row in items {
                if let Some(query) = crate::next::checked(
                    biomcp_types::PgxAnnotationGet::new(row.accession_id.clone()),
                    biomcp_types::Source::ClinPgx,
                    problems,
                ) {
                    next.push(crate::next::Step::new(
                        StepKind::Disambiguate,
                        &[biomcp_types::Source::ClinPgx],
                        run(Request::GetPgxAnnotation(query)),
                    ));
                }
            }
            page_step(q, page, next, None, problems);
        }
        PgxAnnotations::Guideline { page, .. } | PgxAnnotations::Label { page, .. } => {
            page_step(q, page, next, None, problems)
        }
    }
}

fn card_steps(
    q: &PgxGet,
    next: &mut Vec<crate::next::Step>,
    problems: &mut Vec<biomcp_types::Problem>,
) {
    for s in [PgxSection::Recommendations, PgxSection::Guidelines] {
        if let Some(r) = section(q, s, problems) {
            next.push(crate::next::Step::new(
                StepKind::Section,
                &[biomcp_types::Source::Cpic],
                run(r),
            ));
        }
    }
    if let Some(query) = crate::next::checked(
        q.with_section(PgxSection::Annotations, PgxSource::Clinpgx),
        biomcp_types::Source::ClinPgx,
        problems,
    ) {
        next.push(crate::next::Step::new(
            StepKind::Section,
            &[biomcp_types::Source::ClinPgx],
            run(Request::GetPgx(query)),
        ));
    }
}
