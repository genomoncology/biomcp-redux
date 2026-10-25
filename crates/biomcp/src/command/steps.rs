//! Carry active request restrictions through every generated command.
use std::io;
pub(super) fn next(
    fetch: &biomcp_fetch::Fetch,
    result: &biomcp_query::Outcome,
) -> io::Result<Vec<biomcp_types::Step>> {
    let mut steps = result
        .next
        .iter()
        .map(|(kind, action)| crate::spell::step(*kind, action))
        .collect::<Result<Vec<_>, _>>()?;
    let has_more = result
        .page
        .as_ref()
        .is_some_and(|page| u64::from(page.offset) + u64::from(page.returned) < page.total)
        || matches!(&result.body, biomcp_types::QueryBody::PgxAnnotations(biomcp_types::PgxAnnotations::Clinical {page,..}) if u64::from(page.offset)+u64::from(page.returned)<page.total);
    let candidates = matches!(
        &result.body,
        biomcp_types::QueryBody::PgxAnnotations(biomcp_types::PgxAnnotations::Clinical { .. })
            | biomcp_types::QueryBody::DiagnosticCompanion(_)
            | biomcp_types::QueryBody::DiagnosticPage(_)
            | biomcp_types::QueryBody::DiagnosticGenes(_)
            | biomcp_types::QueryBody::AdverseEventPage(_)
            | biomcp_types::QueryBody::DrugPage(_)
            | biomcp_types::QueryBody::DrugFdaPage(_)
            | biomcp_types::QueryBody::DrugCivic(_)
            | biomcp_types::QueryBody::DiseasePage(_)
            | biomcp_types::QueryBody::DiseaseGenes(_)
            | biomcp_types::QueryBody::DiseasePhenotypes(_)
            | biomcp_types::QueryBody::VariantPage(_)
    ) || matches!(
        &result.body,
        biomcp_types::QueryBody::VariantCard(card)
            if card.resolution == biomcp_types::VariantResolution::Candidates
    );
    network_steps(&mut steps, &result.sources, candidates.then_some(has_more));
    Ok(source_steps(fetch, steps))
}

fn source_steps(
    fetch: &biomcp_fetch::Fetch,
    steps: Vec<biomcp_types::Step>,
) -> Vec<biomcp_types::Step> {
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

pub(super) fn network_steps(
    steps: &mut Vec<biomcp_types::Step>,
    sources: &[biomcp_types::SourceOutcome],
    candidate_page: Option<bool>,
) {
    let association = steps
        .iter()
        .any(|step| step.kind == biomcp_types::StepKind::Pivot);
    if candidate_page.is_none() && !association {
        append_network_steps(steps, sources);
        steps.truncate(5);
        return;
    }
    // Some disambiguate actions are searches or relations, not candidate gets.
    let (mut candidates, mut extras): (Vec<_>, Vec<_>) =
        std::mem::take(steps).into_iter().partition(|step| {
            step.kind == biomcp_types::StepKind::Pivot
                || step.kind == biomcp_types::StepKind::Disambiguate
                    && step.args.first().is_some_and(|arg| arg.as_ref() == "get")
        });
    if candidates.is_empty() {
        append_network_steps(&mut extras, sources);
        extras.truncate(5);
        *steps = extras;
        return;
    }
    // A section may itself have page actions. Only the response page is protected.
    if (candidate_page == Some(true) || association)
        && let Some(index) = extras
            .iter()
            .position(|step| step.kind == biomcp_types::StepKind::Page)
    {
        candidates.push(extras.remove(index));
    }
    append_network_steps(&mut extras, sources);
    extras.truncate(3);
    candidates.extend(extras);
    *steps = candidates;
}
fn append_network_steps(
    steps: &mut Vec<biomcp_types::Step>,
    sources: &[biomcp_types::SourceOutcome],
) {
    for (reason, setting) in [
        (biomcp_types::Reason::Tls, "ca_bundle"),
        (biomcp_types::Reason::Proxy, "proxy"),
    ] {
        if sources.iter().any(|source| source.reason == Some(reason)) {
            steps.insert(
                0,
                biomcp_types::Step {
                    kind: biomcp_types::StepKind::Configure,
                    args: vec!["help".into(), "network".into(), setting.into()].into(),
                },
            );
        }
    }
}
