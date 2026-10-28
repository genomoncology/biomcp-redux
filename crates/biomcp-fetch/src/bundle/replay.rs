//! Recorded bundle transfers obey the same bounded status retry policy as live transfers.
use super::{Bundle, Row};
use crate::{Leg, RecordError, RequestError, record::Exchange, retry};
use biomcp_types::Reason;
use std::time::Duration;

pub(super) fn exchange<R: Row>(
    leg: &Leg<'_>,
    bundle: &Bundle<R>,
    key: &[u8; 32],
    url: &str,
) -> Result<Exchange, RequestError> {
    for attempt in 0..retry::ATTEMPTS {
        let permit = leg
            .fetch
            .limiters
            .acquire(leg.spec.quota, leg.keyed(), leg.budget())
            .ok_or_else(|| leg.failure(Reason::Syncing, None))?;
        let exchange = leg
            .fetch
            .recordings
            .replay_bundle(leg.spec.source, key, url)
            .map_err(RequestError::Recording)?
            .ok_or(RequestError::Recording(RecordError::Missing))?;
        let delay = Duration::from_millis(exchange.delay_ms);
        if !leg.budget().advance(delay) {
            return Err(leg.failure(Reason::DeadlineExceeded, None).into());
        }
        leg.observe_bundle(
            (key, url),
            Some(exchange.status),
            exchange.body.bytes(),
            bundle.format,
            delay,
        );
        if exchange.status == 200 {
            return Ok(exchange);
        }
        drop(permit);
        if !retry::retry_status(exchange.status) || attempt + 1 == retry::ATTEMPTS {
            return Err(leg
                .failure(retry::status_reason(exchange.status), Some(exchange.status))
                .into());
        }
        let wait = exchange
            .retry_after_ms
            .map(Duration::from_millis)
            .unwrap_or_else(|| retry::backoff(key, attempt));
        if !retry::wait(wait, leg.budget()) {
            return Err(leg.failure(Reason::Syncing, Some(exchange.status)).into());
        }
    }
    Err(leg.failure(Reason::Unavailable, None).into())
}
