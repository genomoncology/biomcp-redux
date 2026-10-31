use super::{Outcome, bundle_source, encode};
use biomcp_query::Bundle;
use biomcp_types::{Envelope, Status};
use std::io;
pub(super) fn run(fetch: &biomcp_fetch::Fetch, bundle: Bundle) -> io::Result<Outcome> {
    match acquire(fetch, bundle) {
        Some(result) => {
            let (result, stale) = match result {
                Ok((mapped, stale)) => (Ok(mapped), stale),
                Err(error) => (Err(error), false),
            };
            finish(bundle, result, stale)
        }
        None => finish(bundle, biomcp_adapters::wikipathways::sync(fetch), false),
    }
}
fn finish<T: biomcp_types::ResponseBody<Query = biomcp_types::SyncQuery>>(
    bundle: Bundle,
    result: Result<biomcp_adapters::Mapped<T>, biomcp_fetch::RequestError>,
    stale: bool,
) -> io::Result<Outcome> {
    let source = bundle_source(bundle);
    let source_state = if stale {
        biomcp_types::SourceState::Stale
    } else if result.is_ok() {
        biomcp_types::SourceState::Ok
    } else {
        biomcp_types::SourceState::Failed
    };
    let refusals = result
        .as_ref()
        .map_or(biomcp_adapters::Refusal::None, |mapped| mapped.refusals);
    let local = result
        .as_ref()
        .err()
        .is_some_and(|error| !matches!(error, biomcp_fetch::RequestError::Source(_)));
    let reason = result.as_ref().err().and_then(|error| match error {
        biomcp_fetch::RequestError::Source(error) => Some(error.reason),
        _ => None,
    });
    let local_error = super::local::kind(result.as_ref().err().copied());
    let mut problems = Vec::new();
    let value = result.ok().map(|mapped| {
        problems = mapped.problems;
        mapped.value
    });
    let mut envelope =
        Envelope::<T>::new(Status::Failed, biomcp_types::SyncQuery { source }, value);
    envelope.problems = problems;
    envelope.local_error = local_error;
    envelope.sources.push(biomcp_adapters::registry::outcome(
        source,
        if reason == Some(biomcp_types::Reason::Disabled) {
            biomcp_types::SourceState::Skipped
        } else {
            source_state
        },
        reason,
    ));
    let status = biomcp_query::sync_status(&envelope.sources, refusals);
    envelope.status = status;
    if matches!(
        source,
        biomcp_types::Source::Cbioportal | biomcp_types::Source::WikiPathways
    ) && reason == Some(biomcp_types::Reason::BadReply)
    {
        envelope.problems.push(biomcp_types::Problem {
            source,
            record: None,
            field: "catalog".into(),
            code: biomcp_types::ProblemCode::CollectionIncomplete,
        });
    }
    envelope.next = biomcp_query::next::network(&envelope.sources)
        .iter()
        .map(|step| crate::spell::step(step.0, &step.1))
        .collect::<Result<Vec<_>, _>>()?;
    encode(
        envelope,
        if local {
            crate::exit::Status::Local
        } else {
            status.into()
        },
    )
}

fn acquire(
    fetch: &biomcp_fetch::Fetch,
    bundle: Bundle,
) -> Option<
    Result<(biomcp_adapters::Mapped<biomcp_types::SyncResult>, bool), biomcp_fetch::RequestError>,
> {
    Some(
        match bundle {
            Bundle::WikiPathways => return None,
            Bundle::CbioportalStudies => {
                return Some(biomcp_adapters::cbioportal::sync_studies(fetch));
            }
            Bundle::FdaCdx => biomcp_adapters::fda_cdx::sync(fetch),
            Bundle::Gtr => biomcp_adapters::gtr::sync(fetch),
            Bundle::WhoIvd => biomcp_adapters::who_ivd::sync(fetch),
            Bundle::WhoPq => biomcp_adapters::who_pq::sync(fetch),
            Bundle::Ddinter => biomcp_adapters::ddinter::sync(fetch),
            Bundle::Ema => biomcp_adapters::ema::sync(fetch),
            Bundle::ClinGen => biomcp_adapters::clingen::sync(fetch),
            Bundle::Hgnc => biomcp_adapters::hgnc::sync_all(fetch),
            Bundle::Gencc => biomcp_adapters::gencc::sync(fetch),
        }
        .map(|mapped| (mapped, false)),
    )
}
