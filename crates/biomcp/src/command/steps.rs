//! Spell checked follow-ups and retain the caller's global restrictions.
use std::io;
pub(super) fn next(
    request: &biomcp_types::Request,
    result: &biomcp_query::Outcome,
    globals: &crate::grammar::Globals,
) -> io::Result<Vec<biomcp_types::Step>> {
    let mut steps = result
        .next
        .iter()
        .filter(|step| !globals.hosted || !super::mcp_policy::hosted_refused(&step.1))
        .map(|step| {
            let mut spelled = crate::spell::step(step.0, &step.1)?;
            // A section a card lists comes with the options that section takes.
            if step.0 == biomcp_types::StepKind::Section
                && let biomcp_query::Action::Run { request, .. } = &step.1
            {
                spelled.options = crate::grammar::options(request);
            }
            Ok(spelled)
        })
        .collect::<Result<Vec<_>, serde_json::Error>>()?;
    if let Some(step) = adjust(request, result)? {
        steps.push(step);
    }
    Ok(source_steps(globals, steps))
}

/// A first page also offers its own call with the options it could add, after the follow-ups
/// and outside their cap.
fn adjust(
    request: &biomcp_types::Request,
    result: &biomcp_query::Outcome,
) -> Result<Option<biomcp_types::Step>, serde_json::Error> {
    use biomcp_types::Status::{Complete, Partial};
    // Only a first page offers it; an identity card's options live on the sections it lists.
    if !matches!(result.status(), Complete | Partial)
        || result.page.is_none()
        || !crate::grammar::first_page(request)
        || crate::grammar::identity_card(request)
    {
        return Ok(None);
    }
    let options = crate::grammar::options(request);
    if options.is_empty() {
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
    steps: Vec<biomcp_types::Step>,
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
    let fields: Option<Box<str>> = (!globals.fields.is_empty())
        .then(|| format!("--fields={}", globals.fields.join(",")).into());
    if prefix.is_empty() && fields.is_none() {
        return steps;
    }
    steps
        .into_iter()
        .map(|mut step| {
            let carried = crate::fields::carried(&step);
            let mut args = step.args.into_vec();
            if carried && let Some(fields) = &fields {
                args.insert(0, fields.clone());
            }
            args.splice(0..0, prefix.iter().cloned());
            step.args = args.into();
            step
        })
        .collect()
}
