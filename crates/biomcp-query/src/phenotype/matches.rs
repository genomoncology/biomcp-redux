//! One Monarch ranking for a set of HP IDs, without local reranking.
use super::{empty, problem, recovery, run};
use crate::{Ctx, Outcome, status};
use biomcp_adapters::monarch;
use biomcp_types::ReplyBody;
use biomcp_types::{
    DiseaseGet, PHENOTYPE_MATCH_MAX, PhenotypeGet, PhenotypeGroup, PhenotypeMatches,
    PhenotypeSearch, ReplyData, Request, Source, StepKind,
};
pub(super) fn search(cx: &Ctx<'_>, q: &PhenotypeSearch) -> Outcome {
    let command = cx.fetch.command_deadline(cx.budget);
    let deadline = command.deadline();
    let leg = cx.fetch.leg(&monarch::SPEC, deadline).fresh(cx.fresh);
    let reply = monarch::phenotype::matches::search(&leg, q);
    let error = reply.as_ref().err().copied();
    let mut out = empty(body(q, None));
    out.sources
        .push(status::source(Source::Monarch, leg.finish(), error));
    out.local = error.and_then(status::local);
    if let Ok(ranked) = reply {
        out.include_refusals(ranked.refusals);
        out.problems.extend(ranked.problems);
        if ranked.unknown.is_empty() {
            pivots(&mut out, q.group(), &ranked.items);
            out.body = body(
                q,
                Some(PhenotypeMatches {
                    items: ranked.items.into(),
                    terms: ranked.terms.into(),
                    method: ranked.method,
                    source_limit: PHENOTYPE_MATCH_MAX,
                    components: monarch::components().unwrap_or_default(),
                    fetched: ranked
                        .metadata
                        .fetched
                        .and_then(|value| u64::try_from(value).ok()),
                }),
            );
            crate::status::search(&mut out);
        } else {
            refuse(&mut out, q, &ranked.unknown);
        }
    }
    problem(&mut out, Source::Monarch, None);
    recovery(&mut out, Request::SearchPhenotype(q.clone()));
    command.finish(deadline.remaining());
    out
}
fn body(q: &PhenotypeSearch, body: Option<PhenotypeMatches>) -> ReplyData {
    PhenotypeMatches::into_reply(q.clone(), body)
}
/// Monarch would silently rank without an unknown term, so the set is refused.
fn refuse(out: &mut Outcome, q: &PhenotypeSearch, unknown: &[biodata::HpId]) {
    out.body = ReplyData::Usage(biomcp_types::UsageError::Arguments(
        biomcp_types::UsageArguments {
            argument: Some("text".into()),
            message: Some(biomcp_types::ErrorMessage::PhenotypeSetUnknown),
            valid_values: None,
            next: Vec::new(),
        },
    ));
    for id in unknown {
        let get = PhenotypeGet::new(id.clone());
        out.next.push(crate::next::Step::new(
            StepKind::Disambiguate,
            &[Source::Hpo],
            run(Request::GetPhenotype(get)),
        ));
    }
    let known: Vec<_> = q
        .terms()
        .unwrap_or_default()
        .into_iter()
        .filter(|term| !unknown.contains(term))
        .collect();
    if !known.is_empty()
        && let Some(search) =
            crate::next::checked(q.with_terms(&known), Source::Monarch, &mut out.problems)
    {
        out.next.push(crate::next::Step::new(
            StepKind::Restart,
            &[Source::Monarch],
            run(Request::SearchPhenotype(search)),
        ));
    }
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
