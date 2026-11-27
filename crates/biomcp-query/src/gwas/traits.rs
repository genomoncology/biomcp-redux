//! Resolve the selected trait only after the native association count is zero.
use crate::{Ctx, Outcome, next::Step, status};
use biomcp_adapters::gwas::{self, TraitMatch};
use biomcp_fetch::Deadline;
use biomcp_types::{
    GwasSearch, Reason, Request, Source, SourceNotice, SourceState, Status, StepKind,
};

pub(super) fn resolve(cx: &Ctx<'_>, q: &GwasSearch, deadline: Deadline<'_>, out: &mut Outcome) {
    if out.status() != Status::Complete || (q.trait_id().is_none() && q.trait_label().is_none()) {
        return;
    }
    // A page past the end or a p-value cut may be empty for a valid trait.
    let Some(source) = out
        .sources
        .iter_mut()
        .find(|source| source.source == Source::Gwas && source.count == Some(0))
    else {
        return;
    };
    let leg = cx.fetch.leg(&gwas::SPEC, deadline).fresh(cx.fresh);
    let id = q
        .trait_id()
        .map_or(Ok(()), |id| gwas::trait_exists(&leg, id));
    let unknown = id.err().and_then(status::reason) == Some(Reason::NotFound);
    let result = id.and_then(|()| {
        q.trait_label()
            .map(|label| gwas::trait_matches(&leg, label))
            .transpose()
    });
    let error = result.as_ref().err().copied();
    let incoming = status::source(Source::Gwas, leg.finish(), error);
    status::retain_source_state(source, &incoming);
    match result {
        Ok(Some(TraitMatch::Similar {
            exhaustive,
            candidates,
        })) => {
            let names = candidates
                .iter()
                .map(|row| format!("{} ({})", row.name, row.id))
                .collect::<Vec<_>>()
                .join(", ");
            source.notices.push(SourceNotice::RequestGuidance {
                data_date: None,
                message: if names.is_empty() {
                    "GWAS Catalog found no matching trait names.".into()
                } else {
                    format!("Similar GWAS Catalog traits include {names}.").into()
                },
                retry_after_seconds: None,
            });
            out.next.extend(candidates.into_iter().map(|row| {
                Step::run(
                    StepKind::Disambiguate,
                    &[Source::Gwas],
                    Request::SearchGwas(q.choose_trait(row.id)),
                    false,
                )
            }));
            let absent = exhaustive && incoming.state == SourceState::Ok;
            status::item(out, !absent, false, true);
        }
        Err(_) if unknown => status::item(out, false, false, false),
        Err(error) => {
            out.local = status::local(error);
            status::item(out, true, false, true);
        }
        Ok(_) => status::item(out, true, false, false),
    }
}
