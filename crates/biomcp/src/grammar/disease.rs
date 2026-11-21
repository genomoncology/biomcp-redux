//! Recover disease get selectors before a request can be constructed.
use super::{ParseInvalid, recovery, rewrite, validation};
use biomcp_query::Action;
use biomcp_types::{
    DiseaseSource, ErrorMessage, QueryInvalid, Request, Rewrite, UsageArguments, UsageError,
};
use serde_json::Value;

const PATH: (&str, &str) = ("get", "disease");

/// Check replacement sources against the same preparation step as the real command.
pub(super) fn unknown_source(
    via: Rewrite,
    value: &Value,
    cause: &QueryInvalid,
    fields: &[String],
) -> Option<ParseInvalid> {
    if !matches!(cause.arguments(), [name] if name.as_ref() == "source") {
        return None;
    }
    let mut candidate = value.clone();
    candidate.as_object_mut()?.remove("source");
    let default = runnable(via, candidate.clone());
    let choices: Vec<_> = DiseaseSource::ALL
        .iter()
        .filter_map(|source| {
            candidate
                .as_object_mut()?
                .insert("source".into(), source.as_str().into());
            let request = runnable(via, candidate.clone())?;
            Some((biodata::Text::new(source.as_str()).ok()?, request))
        })
        .collect();
    let corrected = default.as_ref().or(match choices.as_slice() {
        [(_, only)] => Some(only),
        _ => None,
    });
    let next = corrected.and_then(|request| {
        let mut step = recovery::run(request)?;
        crate::fields::carry_to_correction(request, fields, &mut step);
        Some(step)
    });
    let valid_values: Vec<_> = choices.into_iter().map(|(source, _)| source).collect();
    let mut arguments = UsageArguments {
        argument: Some("--source".into()),
        message: (!valid_values.is_empty()).then_some(ErrorMessage::DiseaseGetSource),
        valid_values: (!valid_values.is_empty()).then(|| valid_values.into()),
        next: vec![validation::help([PATH.0, PATH.1])],
    };
    let dropped = next.is_some();
    recovery::next(&mut arguments, next);
    Some(ParseInvalid {
        usage: Some(UsageError::Arguments(arguments)),
        dropped,
        ..ParseInvalid::default()
    })
}

fn runnable(via: Rewrite, value: Value) -> Option<Request> {
    let request = rewrite::request(via, PATH.0, PATH.1, value).ok()?;
    Action::Run {
        request: request.clone(),
        fresh: false,
    }
    .prepare()
    .ok()?;
    Some(request)
}
