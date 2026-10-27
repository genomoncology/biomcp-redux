//! Resolve the source's own candidate-to-CA cross-reference before the checked GRCh38 join.
//! No expression edit, assembly conversion, padding or candidate selection occurs here.
use crate::{Ctx, status};
use biomcp_adapters::allele_registry;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{Problem, Reason, Source, SourceOutcome, VariantGet, VariantHeader};
use biodata::VariantId;

#[derive(Default)]
pub(super) struct Identity {
    pub rows: Vec<VariantHeader>,
    pub sources: Vec<SourceOutcome>,
    pub problems: Vec<Problem>,
    pub local: Option<RequestError>,
    pub reason: Option<Reason>,
}
pub(super) fn read(cx: &Ctx<'_>, query: &VariantGet, deadline: Deadline<'_>) -> Identity {
    let mut found = Identity::default();
    let spec = super::source(query);
    let leg = cx.fetch.leg(spec, deadline).fresh(cx.fresh);
    let result = if spec.source == Source::MyVariant {
        super::myvariant_identity(query, &leg, &mut found.problems, true)
    } else {
        super::resolve(query, &leg, &mut found.problems)
    };
    let error = result.as_ref().err().copied();
    found
        .sources
        .push(status::source(spec.source, leg.finish(), error));
    found.local = error.and_then(status::local);
    found.reason = error.and_then(status::reason);
    if let Ok((rows, rejection)) = result {
        found.rows = rows;
        if rejection.is_some() {
            found.reason = Some(Reason::Rejected);
        }
    }
    if found.rows.len() == 1 && spec.source == Source::MyVariant {
        canonical(cx, deadline, &mut found);
    }
    found
}
fn canonical(cx: &Ctx<'_>, deadline: Deadline<'_>, found: &mut Identity) {
    let row = &mut found.rows[0];
    let ids: Vec<_> = row
        .identity
        .ids
        .iter()
        .filter(|id| matches!(id, VariantId::Ca(_)))
        .collect();
    let [id] = ids.as_slice() else {
        return;
    };
    let leg = cx
        .fetch
        .leg(&allele_registry::SPEC, deadline)
        .fresh(cx.fresh);
    let result = allele_registry::by_id(&leg, &id.to_string());
    let error = result.as_ref().err().copied();
    found
        .sources
        .push(status::source(Source::AlleleRegistry, leg.finish(), error));
    found.local = found.local.or(error.and_then(status::local));
    found.reason = error.and_then(status::reason);
    match result {
        Ok(allele_registry::Resolution::Found(mapped)) => {
            found.problems.extend(mapped.problems);
            let mut representations = row.identity.representations.to_vec();
            representations.extend(mapped.value.identity.representations);
            row.identity.representations = representations.into();
        }
        Ok(allele_registry::Resolution::Rejected(_)) => found.reason = Some(Reason::Rejected),
        Err(_) => (),
    }
}
