//! Bounded source probes, never local storage or HTTP service liveness.
use biodata::{Health, Reason, SourceHealth, SourceState, Text};
use biomcp_adapters::registry;
use biomcp_fetch::{Fetch, KeyUse, RequestError};
use std::time::Duration;
/// Probe every implemented source with one shared command deadline and short source caps.
pub fn run(fetch: &Fetch, budget: Duration) -> Result<Health, biodata::Error> {
    let deadline = fetch.deadline(budget);
    let sources = crate::fan_out::run(
        registry::SOURCES
            .iter()
            .map(|&spec| {
                Box::new(move || {
                    let disabled = fetch.disabled(spec.source);
                    let skip = if disabled {
                        Some(Reason::Disabled)
                    } else {
                        match spec.key {
                            KeyUse::Required(c) if !fetch.has_key(c.secret) => Some(Reason::NoKey),
                            _ => None,
                        }
                    };
                    let (state, reason, latency_ms) = if let Some(reason) = skip {
                        (SourceState::Skipped, Some(reason), None)
                    } else {
                        let leg = fetch
                            .leg(spec, deadline.child(Duration::from_secs(3)))
                            .fresh(true);
                        let start = leg.remaining();
                        let result = registry::probe(spec, &leg);
                        let elapsed = start.saturating_sub(leg.remaining()).as_millis() as u64;
                        let reason = result.err().map(|e| match e {
                            RequestError::Source(e) => e.reason,
                            _ => Reason::Unavailable,
                        });
                        (
                            if reason.is_some() {
                                SourceState::Failed
                            } else {
                                SourceState::Ok
                            },
                            reason,
                            Some(elapsed),
                        )
                    };
                    Ok(SourceHealth {
                        source: spec.source,
                        terms: Text::new(spec.terms.name)?,
                        terms_url: Text::new(spec.terms.url)?,
                        usage: spec.terms.usage,
                        disabled,
                        state,
                        reason,
                        latency_ms,
                    })
                })
                    as Box<dyn FnOnce() -> Result<SourceHealth, biodata::Error> + Send>
            })
            .collect(),
    );
    Ok(Health {
        sources: sources.into_iter().collect::<Result<Vec<_>, _>>()?.into(),
    })
}
