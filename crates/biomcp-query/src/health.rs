//! Bounded source probes, never local storage or HTTP service liveness.
use biodata::Text;
use biomcp_adapters::registry;
use biomcp_fetch::{Fetch, KeyUse, RequestError};
use biomcp_types::{Health, Reason, SourceHealth, SourceState};
use std::time::Duration;
const PROBE_BUDGET: Duration = Duration::from_secs(3);
/// Probe every implemented source with one shared command deadline and short source caps.
pub fn run(fetch: &Fetch, budget: Duration) -> Result<Health, RequestError> {
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
                        let leg = fetch.leg(spec, deadline.child(PROBE_BUDGET)).fresh(true);
                        let start = leg.remaining();
                        let result = registry::probe(spec, &leg);
                        let elapsed = start.saturating_sub(leg.remaining()).as_millis() as u64;
                        if let Err(error) = result
                            && crate::status::local(error).is_some()
                        {
                            return Err(error);
                        }
                        let reason = result.err().map(|e| crate::status::reason(e));
                        (
                            if reason == Some(Reason::Disabled) {
                                SourceState::Skipped
                            } else if reason.is_some() {
                                SourceState::Failed
                            } else {
                                SourceState::Ok
                            },
                            reason,
                            Some(elapsed),
                        )
                    };
                    Ok(SourceHealth {
                        components: component_health(spec.source, fetch)?.into(),
                        source: spec.source,
                        terms: Text::new(spec.terms.name)
                            .map_err(|_| biomcp_fetch::InputError::Serialization)?,
                        terms_url: Text::new(spec.terms.url)
                            .map_err(|_| biomcp_fetch::InputError::Serialization)?,
                        usage: spec.terms.usage,
                        disabled,
                        state,
                        reason,
                        latency_ms,
                    })
                }) as Box<dyn FnOnce() -> Result<SourceHealth, RequestError> + Send>
            })
            .collect(),
    );
    Ok(Health {
        sources: sources.into_iter().collect::<Result<Vec<_>, _>>()?.into(),
    })
}

fn component_health(
    source: biomcp_types::Source,
    fetch: &Fetch,
) -> Result<Vec<biomcp_types::ComponentHealth>, RequestError> {
    registry::components()
        .filter(|component| component.source == source)
        .map(|component| {
            Ok(biomcp_types::ComponentHealth {
                name: Text::new(component.name)
                    .map_err(|_| biomcp_fetch::InputError::Serialization)?,
                terms: biomcp_adapters::myvariant::components::label(component.terms)?,
                disabled: component.disabled(fetch),
            })
        })
        .collect()
}
