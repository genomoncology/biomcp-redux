//! One Monarch ranking for a set of HP IDs, without local reranking.
use super::{empty, problem, recovery, run};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::monarch;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGet, PHENOTYPE_MATCH_MAX, PhenotypeGroup, PhenotypeMatches, PhenotypeMetric,
    PhenotypeSearch, ReplyData, Request, Source, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &PhenotypeSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&monarch::SPEC, deadline).fresh(cx.fresh);
    let mut out = empty(body(q, None));
    match monarch::phenotype::matches::search(&leg, q) {
        Ok(ranked) => {
            out.sources
                .push(status::source(Source::Monarch, leg.finish(), None));
            accept(&mut out, q, ranked);
        }
        Err(error) => {
            out.sources
                .push(status::source(Source::Monarch, leg.finish(), Some(error)));
            out.local = status::local(error);
        }
    }
    problem(&mut out, Source::Monarch, None);
    recovery(&mut out, Request::SearchPhenotype(q.clone()));
    command.finish(deadline.remaining());
    out
}
/// Keep Monarch's rows, or refuse a set holding a term Monarch does not know.
fn accept(out: &mut Outcome, q: &PhenotypeSearch, ranked: monarch::phenotype::matches::Ranked) {
    out.include_refusals(ranked.refusals);
    out.problems.extend(ranked.problems);
    if !ranked.unknown.is_empty() {
        refuse(out, q, &ranked.unknown);
        return;
    }
    pivots(out, q.group(), &ranked.items);
    out.body = body(
        q,
        Some(PhenotypeMatches {
            items: ranked.items.into(),
            terms: ranked.terms.into(),
            method: ranked.method,
            source_limit: PHENOTYPE_MATCH_MAX,
            components: monarch::components().unwrap_or_default(),
            fetched: crate::disease::seer::fetched(ranked.metadata.fetched),
        }),
    );
    crate::status::search(out);
}
fn body(q: &PhenotypeSearch, body: Option<PhenotypeMatches>) -> ReplyData {
    PhenotypeMatches::into_reply(q.clone(), body)
}
/// Monarch would silently rank without an unknown term, so the set is refused.
fn refuse(out: &mut Outcome, q: &PhenotypeSearch, unknown: &[biodata::HpId]) {
    let step = |kind, args: Vec<String>| biomcp_types::Step {
        kind,
        args: args.into_iter().map(Into::into).collect(),
    };
    let mut next: Vec<_> = unknown
        .iter()
        .map(|id| {
            step(
                StepKind::Disambiguate,
                vec!["get".into(), "phenotype".into(), id.to_string()],
            )
        })
        .collect();
    let known: Vec<_> = q
        .terms()
        .unwrap_or_default()
        .into_iter()
        .filter(|term| !unknown.contains(term))
        .map(|term| term.to_string())
        .collect();
    if !known.is_empty() {
        let mut args = vec!["search".into(), "phenotype".into(), known.join(",")];
        if q.group() != PhenotypeGroup::Genes {
            args.extend(["--group".into(), q.group().as_str().into()]);
        }
        if q.metric() != PhenotypeMetric::AncestorInformationContent {
            args.extend(["--metric".into(), q.metric().as_str().into()]);
        }
        args.extend(["--limit".into(), q.limit().to_string()]);
        next.push(step(StepKind::Restart, args));
    }
    out.body = ReplyData::Usage(biomcp_types::UsageError::Arguments(
        biomcp_types::UsageArguments {
            argument: Some("text".into()),
            message: Some(biomcp_types::ErrorMessage::PhenotypeSetUnknown),
            valid_values: None,
            next,
        },
    ));
}
fn pivots(out: &mut Outcome, group: PhenotypeGroup, rows: &[biomcp_types::PhenotypeMatch]) {
    for row in rows {
        let id = row.id.to_string();
        let (request, source) = match group {
            PhenotypeGroup::Genes => (
                crate::disease::monarch::gene_query(&id, &mut out.problems).map(Request::GetGene),
                Source::Hgnc,
            ),
            PhenotypeGroup::Diseases => (
                biomcp_adapters::mydisease::supported_id(&id)
                    .then(|| {
                        crate::next::checked(
                            DiseaseGet::from_id(row.id.clone()),
                            Source::Monarch,
                            &mut out.problems,
                        )
                    })
                    .flatten()
                    .map(Request::GetDisease),
                Source::MyDisease,
            ),
        };
        if let Some(request) = request {
            let step = crate::next::Step::new(StepKind::Pivot, &[source], run(request));
            if !out.next.contains(&step) {
                out.next.push(step);
            }
        }
    }
}
