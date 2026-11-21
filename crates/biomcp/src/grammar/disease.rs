//! Recover disease get selectors before a request can be constructed.
use super::{ParseInvalid, recovery, rewrite, validation};
use biomcp_query::Action;
use biomcp_types::{
    DiseaseSource, ErrorMessage, QueryInvalid, Request, Rewrite, Step, UsageArguments, UsageError,
};
use serde_json::Value;

const PATH: (&str, &str) = ("get", "disease");

/// Check replacement sources against the same preparation step as the real command.
pub(super) fn source_refusal(
    via: Rewrite,
    value: &Value,
    cause: &QueryInvalid,
    fields: &[String],
) -> Option<ParseInvalid> {
    let prevalence = cause.message() == Some(ErrorMessage::DiseasePrevalenceSource);
    if !prevalence && !matches!(cause.arguments(), [name] if name.as_ref() == "source") {
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
    if choices.is_empty() && prevalence {
        // An invalid prevalence ID needs the constructor's ID guidance.
        return None;
    }
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
        message: (!valid_values.is_empty())
            .then_some(cause.message().unwrap_or(ErrorMessage::DiseaseGetSource)),
        valid_values: (!valid_values.is_empty()).then(|| valid_values.into()),
        next: vec![validation::help([PATH.0, PATH.1])],
    };
    recovery::next(&mut arguments, next);
    Some(ParseInvalid {
        usage: Some(UsageError::Arguments(arguments)),
        ..ParseInvalid::default()
    })
}

/// A disease card is a valid fallback when the requested section has no disease equivalent.
pub(super) fn gene_validity_card(via: Rewrite, value: &Value) -> Option<Step> {
    let mut candidate = value.clone();
    let fields = candidate.as_object_mut()?;
    fields.remove("section");
    if fields.get("source").and_then(Value::as_str) == Some("clingen") {
        fields.remove("source");
    }
    recovery::run(&runnable(via, candidate)?)
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
