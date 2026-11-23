//! Spell checked follow-ups and retain the caller's global restrictions.
use biomcp_query::Action;
use biomcp_types::{Request, SourceOutcome, StepKind};
use std::{borrow::Cow, io};
pub(super) fn next(
    request: &biomcp_types::Request,
    result: &biomcp_query::Outcome,
    globals: &crate::grammar::Globals,
    allow_fresh: bool,
) -> io::Result<Vec<biomcp_types::Step>> {
    let retryable = result.sources.iter().any(SourceOutcome::retryable);
    let mut steps = followups(request, result, retryable, allow_fresh)
        .filter(|(_, action)| !globals.hosted || !super::mcp_policy::hosted_refused(action))
        .map(|(kind, action)| {
            let mut spelled = crate::spell::step(kind, &action)?;
            // A section a card lists comes with the options that section takes.
            if kind == StepKind::Section
                && let biomcp_query::Action::Run {
                    request: section, ..
                } = action.as_ref()
            {
                spelled.options = crate::grammar::options(section);
                // Trial help lists the site filters together. Keep section links to paging options.
                if matches!(section, biomcp_types::Request::GetTrial(trial)
                    if trial.sections() == [biomcp_types::TrialSection::Locations])
                {
                    spelled.options.remove("--status");
                }
            }
            let carried = if globals.fields.is_empty() {
                Vec::new()
            } else {
                crate::fields::carried(kind, &action, &globals.fields)
            };
            Ok((spelled, carried))
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;
    if let Some(step) = adjust(request, result)? {
        steps.push((step, Vec::new()));
    }
    Ok(source_steps(globals, steps))
}

fn followups<'a>(
    request: &'a Request,
    result: &'a biomcp_query::Outcome,
    retryable: bool,
    allow_fresh: bool,
) -> impl Iterator<Item = (StepKind, Cow<'a, Action>)> {
    let missing = retryable
        && !result.next.iter().any(|step| {
            matches!(
                step.1,
                Action::Run { .. } | Action::PubmedSearch(_) | Action::AuthorPapers(_)
            )
        });
    let retry = |request: &Request| {
        (
            StepKind::Restart,
            Cow::Owned(Action::Run {
                request: request.clone(),
                fresh: false,
            }),
        )
    };
    result
        .next
        .iter()
        .map(move |step| {
            if retryable
                && !allow_fresh
                && step.0 == StepKind::Fresh
                && let Action::Run {
                    request,
                    fresh: true,
                } = &step.1
            {
                return retry(request);
            }
            (step.0, Cow::Borrowed(&step.1))
        })
        .chain(missing.then(|| retry(request)))
}

const PAGING: [&str; 6] = [
    "--limit",
    "--offset",
    "--section-limit",
    "--section-offset",
    "--candidate-limit",
    "--candidate-offset",
];

/// A section's own first page also offers its own call with the options it could add, after
/// the follow-ups and outside their cap. A card's options live on the sections it lists, and a
/// search page offers none: its reply stays lean, and typed MCP lists search arguments in its
/// schema.
fn adjust(
    request: &biomcp_types::Request,
    result: &biomcp_query::Outcome,
) -> Result<Option<biomcp_types::Step>, serde_json::Error> {
    use biomcp_types::Status::{Complete, Partial};
    // Full openFDA label pages only offer actual continuations.
    if !matches!(result.status(), Complete | Partial)
        || result.page.is_none()
        || matches!(request, biomcp_types::Request::GetDrug(query) if query.sections() == [biomcp_types::DrugSection::Label])
        || !crate::grammar::first_page(request)
        || !crate::grammar::section_page(request)
    {
        return Ok(None);
    }
    // Paging alone is what the page step already offers.
    let options = crate::grammar::options(request);
    if options.keys().all(|name| PAGING.contains(&name.as_ref())) {
        return Ok(None);
    }
    let mut step = crate::spell::step(
        biomcp_types::StepKind::Configure,
        &biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        },
    )?;
    step.options = options;
    Ok(Some(step))
}

fn source_steps(
    globals: &crate::grammar::Globals,
    steps: Vec<(biomcp_types::Step, Vec<String>)>,
) -> Vec<biomcp_types::Step> {
    let mut prefix: Vec<Box<str>> = Vec::new();
    // A server call inherits its configuration, and refuses a call that names one.
    if let Some(path) = &globals.config
        && !globals.served
    {
        prefix.extend([
            "--config".into(),
            path.to_string_lossy().into_owned().into(),
        ]);
    }
    if !globals.disable_sources.is_empty() {
        prefix.extend([
            "--disable-sources".into(),
            globals.disable_sources.join(",").into(),
        ]);
    }
    if prefix.is_empty() && globals.fields.is_empty() {
        return steps.into_iter().map(|(step, _)| step).collect();
    }
    steps
        .into_iter()
        .map(|(mut step, carried)| {
            let mut args = step.args.into_vec();
            if !carried.is_empty() {
                args.insert(0, format!("--fields={}", carried.join(",")).into());
            }
            args.splice(0..0, prefix.iter().cloned());
            step.args = args.into();
            step
        })
        .collect()
}
