//! Carry active request restrictions through every generated command.
use std::io;
pub(super) fn next(
    fetch: &biomcp_fetch::Fetch,
    result: &biomcp_query::Outcome,
) -> io::Result<Vec<biodata::Step>> {
    let mut steps = result
        .next
        .iter()
        .map(|(kind, action)| crate::spell::step(*kind, action))
        .collect::<Result<Vec<_>, _>>()?;
    network_steps(&mut steps, &result.sources);
    Ok(source_steps(fetch, steps))
}

fn source_steps(fetch: &biomcp_fetch::Fetch, steps: Vec<biodata::Step>) -> Vec<biodata::Step> {
    let registered = biomcp_adapters::registry::SOURCES;
    let sources = registered.iter().map(|spec| spec.source).chain(
        fetch
            .disabled_sources()
            .filter(|source| !registered.iter().any(|spec| spec.source == *source)),
    );
    let disabled = sources
        .filter(|source| fetch.disabled(*source))
        .map(|source| source.to_string())
        .chain(fetch.disabled_uses().map(|usage| usage.as_str().to_owned()))
        .collect::<Vec<_>>()
        .join(",");
    if disabled.is_empty() {
        return steps;
    }
    steps
        .into_iter()
        .map(|mut step| {
            let mut args = step.args.into_vec();
            args.splice(
                0..0,
                [
                    "--disable-sources".into(),
                    disabled.clone().into_boxed_str(),
                ],
            );
            step.args = args.into();
            step
        })
        .collect()
}

pub(super) fn network_steps(steps: &mut Vec<biodata::Step>, sources: &[biodata::SourceOutcome]) {
    for (reason, setting) in [
        (biodata::Reason::Tls, "ca_bundle"),
        (biodata::Reason::Proxy, "proxy"),
    ] {
        if sources.iter().any(|source| source.reason == Some(reason)) {
            steps.insert(
                0,
                biodata::Step {
                    kind: biodata::StepKind::Configure,
                    args: vec!["help".into(), "network".into(), setting.into()].into(),
                },
            );
        }
    }
    steps.truncate(5);
}
