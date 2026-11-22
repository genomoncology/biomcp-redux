//! Recognize comma lists without splitting identifiers that already contain a comma.
use biodata::NctId;
use biomcp_types::{
    ArticleInput, ErrorMessage, GeneInput, ProteinInput, UsageArguments, UsageError,
};
use serde::Deserialize;
use serde_json::Value;

const FORM: &str = "Separate IDs with spaces or commas.";

struct Ids {
    item: fn(&str) -> bool,
    allow_literal_comma: bool,
}

fn batch(verb: &str, entity: &str) -> Option<Ids> {
    if verb != "get" {
        return None;
    }
    let (item, allow_literal_comma): (fn(&str) -> bool, bool) = match entity {
        "trial" => (|word| word.parse::<NctId>().is_ok(), false),
        "article" => (article, true),
        "gene" => (gene, false),
        "protein" => (
            |word| {
                ProteinInput::deserialize(serde::de::value::StrDeserializer::<
                    serde::de::value::Error,
                >::new(word))
                .is_ok()
            },
            false,
        ),
        _ => return None,
    };
    Some(Ids {
        item,
        allow_literal_comma,
    })
}

fn article(word: &str) -> bool {
    word.parse::<ArticleInput>().is_ok()
}

fn gene(word: &str) -> bool {
    match word.parse::<GeneInput>() {
        Ok(GeneInput::Text(_)) => word.parse::<biodata::GeneSymbol>().is_ok(),
        Ok(GeneInput::HgncId(_) | GeneInput::EnsemblId(_) | GeneInput::EntrezId(_)) => true,
        Err(_) => false,
    }
}

pub(super) fn guidance(verb: &str, entity: &str) -> Option<&'static str> {
    batch(verb, entity).map(|_| FORM)
}

/// Split an argv word only when all its nonempty pieces are checked IDs. Free-text gene aliases
/// and valid comma-bearing article IDs keep their original spelling. Flag empty entries so the
/// caller can offer the cleaned-up input without running it.
pub(super) fn normalize(verb: &str, entity: &str, value: &Value) -> Option<(Value, bool)> {
    let ids = batch(verb, entity)?;
    let input = value.get("input")?.as_array()?;
    let mut normalized: Option<Vec<Value>> = None;
    let mut empty = false;
    for (index, original) in input.iter().enumerate() {
        if let Some(word) = original.as_str()
            && word.contains(',')
            && !(ids.allow_literal_comma && (ids.item)(word))
        {
            let pieces = word.split(',').map(str::trim);
            if pieces
                .clone()
                .all(|part| part.is_empty() || (ids.item)(part))
            {
                let expanded =
                    normalized.get_or_insert_with(|| input.iter().take(index).cloned().collect());
                empty |= pieces.clone().any(str::is_empty);
                expanded.extend(pieces.filter(|part| !part.is_empty()).map(Value::from));
                continue;
            }
        }
        if let Some(expanded) = &mut normalized {
            expanded.push(original.clone());
        }
    }
    let expanded = normalized?;
    let mut value = value.clone();
    *value.get_mut("input")? = Value::Array(expanded);
    Some((value, empty))
}

pub(super) fn refusal(
    verb: &str,
    entity: &str,
    candidate: Result<biomcp_types::Request, biomcp_types::QueryInvalid>,
    fields: &[String],
) -> super::ParseInvalid {
    let next = candidate.ok().and_then(|request| {
        biomcp_query::Action::Run {
            request: request.clone(),
            fresh: false,
        }
        .prepare()
        .ok()?;
        let mut step = super::recovery::run(&request)?;
        crate::fields::carry_to_correction(&request, fields, &mut step);
        Some(step)
    });
    super::ParseInvalid {
        usage: Some(UsageError::Arguments(UsageArguments {
            argument: Some("input".into()),
            message: Some(ErrorMessage::ArgumentValue),
            valid_values: biodata::Text::new(FORM).ok().map(|text| vec![text].into()),
            next: vec![next.unwrap_or_else(|| super::validation::help([verb, entity]))],
        })),
        prefer_argument_error: true,
        ..super::ParseInvalid::default()
    }
}
