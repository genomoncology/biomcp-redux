use super::{Outcome, bundle_source, encode, steps};
use biomcp_query::Bundle;
use biomcp_types::{Envelope, Status};
use std::io;
pub(super) fn run(fetch: &biomcp_fetch::Fetch, bundle: Bundle) -> io::Result<Outcome> {
    match acquire(fetch, bundle) {
        Some(result) => finish(bundle, result),
        None => finish(bundle, biomcp_adapters::wikipathways::sync(fetch)),
    }
}
fn finish<T: biomcp_types::ResponseBody<Query = biomcp_types::SyncQuery>>(
    bundle: Bundle,
    result: Result<T, biomcp_fetch::RequestError>,
) -> io::Result<Outcome> {
    let source = bundle_source(bundle);
    let status = if result.is_ok() {
        Status::Complete
    } else {
        Status::Failed
    };
    let local = result
        .as_ref()
        .err()
        .is_some_and(|error| !matches!(error, biomcp_fetch::RequestError::Source(_)));
    let reason = result.as_ref().err().and_then(|error| match error {
        biomcp_fetch::RequestError::Source(error) => Some(error.reason),
        _ => None,
    });
    let local_error = super::local::kind(result.as_ref().err().copied());
    let mut envelope = Envelope::<T>::new(status, biomcp_types::SyncQuery { source }, result.ok());
    envelope.local_error = local_error;
    envelope.sources.push(biomcp_adapters::registry::outcome(
        source,
        if reason == Some(biomcp_types::Reason::Disabled) {
            biomcp_types::SourceState::Skipped
        } else if status == Status::Complete {
            biomcp_types::SourceState::Ok
        } else {
            biomcp_types::SourceState::Failed
        },
        reason,
    ));
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
    steps::network_steps(&mut envelope.next, &envelope.sources, None);
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
) -> Option<Result<biomcp_types::SyncResult, biomcp_fetch::RequestError>> {
    Some(match bundle {
        Bundle::WikiPathways => return None,
        Bundle::CbioportalStudies => biomcp_adapters::cbioportal::sync_studies(fetch),
        Bundle::FdaCdx => biomcp_adapters::fda_cdx::sync(fetch),
        Bundle::Gtr => biomcp_adapters::gtr::sync(fetch),
        Bundle::WhoIvd => biomcp_adapters::who_ivd::sync(fetch),
        Bundle::WhoPq => biomcp_adapters::who_pq::sync(fetch),
        Bundle::Ddinter => {
            biomcp_adapters::ddinter::sync(fetch).map(|info| biomcp_types::SyncResult {
                snapshot: None,
                etag: None,
                last_modified: None,
                duplicate_identities: None,
                rows: info.rows,
                bytes: info.bytes,
                downloaded_bytes: info.downloaded_bytes,
            })
        }
        Bundle::Ema => biomcp_adapters::ema::sync(fetch).map(|info| biomcp_types::SyncResult {
            snapshot: None,
            etag: None,
            last_modified: None,
            duplicate_identities: None,
            rows: info.rows,
            bytes: info.bytes,
            downloaded_bytes: info.downloaded_bytes,
        }),
        Bundle::ClinGen => biomcp_adapters::clingen::sync(fetch),
        Bundle::Hgnc => {
            biomcp_adapters::hgnc::sync(fetch, biomcp_adapters::hgnc::HgncBundle::Complete)
                .and_then(|complete| {
                    let withdrawn = biomcp_adapters::hgnc::sync(
                        fetch,
                        biomcp_adapters::hgnc::HgncBundle::Withdrawn,
                    )?;
                    Ok(biomcp_types::SyncResult {
                        snapshot: None,
                        etag: None,
                        last_modified: None,
                        duplicate_identities: None,
                        rows: complete.rows + withdrawn.rows,
                        bytes: complete.bytes + withdrawn.bytes,
                        downloaded_bytes: complete.downloaded_bytes + withdrawn.downloaded_bytes,
                    })
                })
        }
    })
}
