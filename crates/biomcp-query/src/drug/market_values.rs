//! Values and field diagnostics from individual drug markets.
use crate::Outcome;
use biodata::{AuthorizationDate, PartialDate, Text};
use biomcp_types::{DrugMarket, DrugMarketState, DrugSource, Source};

pub(in crate::drug) fn row(source: DrugSource) -> DrugMarket {
    DrugMarket {
        source,
        state: DrugMarketState::NoRecord,
        record: None,
        name: None,
        holder: None,
        status: Box::default(),
        first_approval: None,
        url: None,
        matches: None,
        choice: None,
        reason: None,
    }
}

pub(super) fn distinct(values: impl Iterator<Item = Text>) -> Box<[Text]> {
    let mut seen = Vec::new();
    for value in values {
        if !seen.contains(&value) {
            seen.push(value);
        }
    }
    seen.into()
}

pub(in crate::drug) fn dated(
    role: &str,
    date: PartialDate,
    source: Source,
    out: &mut Outcome,
) -> Option<AuthorizationDate> {
    let role = kept(role.parse::<Text>(), source, "first_approval", out)?;
    Some(AuthorizationDate { role, date })
}

pub(in crate::drug) fn text(
    value: &str,
    source: Source,
    field: &str,
    out: &mut Outcome,
) -> Option<Text> {
    kept(Text::new(value.trim()), source, field, out)
}

/// Keep a value the source supplied; a value that cannot be built is a problem on its field.
pub(in crate::drug) fn kept<T, E>(
    result: Result<T, E>,
    source: Source,
    field: &str,
    out: &mut Outcome,
) -> Option<T> {
    match result {
        Ok(value) => Some(value),
        Err(_) => {
            out.problems.push(biomcp_types::Problem {
                source,
                record: None,
                field: field.into(),
                code: biomcp_types::ProblemCode::ValueUnknown,
            });
            None
        }
    }
}
