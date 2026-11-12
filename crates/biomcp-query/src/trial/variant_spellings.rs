//! Search a typed variant's other spellings, and its general text with its eligibility criteria.
use biodata::Text;
use biomcp_types::{SourceNotice, SourceOutcome, TrialSearch, VariantSpellings, variant_terms};

/// The registry request, and one notice per widened term.
pub(super) struct Widened {
    pub query: TrialSearch,
    pub notices: Vec<SourceNotice>,
}

/// A page token already belongs to one registry query, so continuing never widens again.
pub(super) fn widen(query: &TrialSearch, gene: bool) -> Widened {
    let mut notices = Vec::new();
    if query.exact_term() || query.next_page().is_some() {
        return Widened {
            query: query.clone(),
            notices,
        };
    }
    let variant = query.variant().map(|typed| {
        let spellings = VariantSpellings::of(typed.as_str(), gene);
        match Text::new(&variant_terms(typed.as_str(), spellings.as_ref())) {
            Ok(terms) => {
                notices.push(notice(typed, spellings.as_ref(), true));
                terms
            }
            Err(_) => typed.clone(),
        }
    });
    let criteria = query.criteria().map(|typed| {
        let Some(spellings) = VariantSpellings::of(typed.as_str(), gene) else {
            return typed.clone();
        };
        match Text::new(&spellings.criteria_terms(typed.as_str())) {
            Ok(terms) => {
                notices.push(notice(typed, Some(&spellings), false));
                terms
            }
            Err(_) => typed.clone(),
        }
    });
    Widened {
        query: query.with_variant_terms(variant, criteria),
        notices,
    }
}

fn notice(typed: &Text, spellings: Option<&VariantSpellings>, general: bool) -> SourceNotice {
    let place = if general {
        "general text and eligibility criteria"
    } else {
        "eligibility criteria"
    };
    let exact = if general {
        "--exact-term searches the typed text in general text only."
    } else {
        "--exact-term searches the typed text only."
    };
    let Some(spellings) = spellings else {
        return SourceNotice::VariantSearch {
            input: typed.clone(),
            spellings: vec![typed.clone()].into(),
            codon: Box::default(),
            message: format!(
                "Searched {place} for the typed text; BioMCP knows no other spellings for it. {exact}"
            )
            .into(),
        };
    };
    let mut message = format!("Searched {place} for {}", spellings.spellings.join(", "));
    if !spellings.codon.is_empty() {
        message.push_str(&format!(
            " and codon forms {} (any change at this position; check each trial's allele)",
            spellings.codon.join(", ")
        ));
    }
    if !spellings.codon_without_gene.is_empty() {
        message.push_str(&format!(
            "; add --gene to also search codon forms {}",
            spellings.codon_without_gene.join(", ")
        ));
    }
    message.push_str(&format!(". {exact}"));
    let texts = |values: &[String]| -> Box<[Text]> {
        values
            .iter()
            .filter_map(|value| Text::new(value).ok())
            .collect()
    };
    SourceNotice::VariantSearch {
        input: typed.clone(),
        spellings: texts(&spellings.spellings),
        codon: texts(&spellings.codon),
        message: message.into(),
    }
}

/// Name each returned trial that matched only through a codon form.
pub(super) fn codon_notice(
    sent: &TrialSearch,
    rows: &[biodata::NctId],
    source: &mut SourceOutcome,
) {
    let Some((exact, codon)) = sent
        .variant()
        .and_then(|variant| biomcp_types::codon_split(variant.as_str()))
    else {
        return;
    };
    if rows.is_empty() {
        return;
    }
    let ids: Vec<_> = rows.iter().map(ToString::to_string).collect();
    let change = exact
        .iter()
        .find(|value| {
            !value.starts_with("p.") && value.chars().nth(1).is_some_and(|c| c.is_ascii_digit())
        })
        .or(exact.first())
        .map_or("", String::as_str);
    source.notices.push(SourceNotice::RequestGuidance {
        data_date: None,
        message: format!(
            "Matched only by codon form {}: {}. Their returned title, summary, description, conditions, \
             keywords and criteria do not write {change} or its other spellings; check each trial's allele.",
            codon.join(" or "),
            ids.join(", ")
        )
        .into(),
        retry_after_seconds: None,
    });
}

/// Say the registry searched only the typed text after refusing the widened request.
pub(super) fn retract(source: &mut SourceOutcome) {
    for notice in &mut source.notices {
        if let SourceNotice::VariantSearch {
            input,
            spellings,
            codon,
            message,
        } = notice
        {
            *spellings = vec![input.clone()].into();
            *codon = Box::default();
            *message =
                "ClinicalTrials.gov rejected the widened request; searched the typed text as written."
                    .into();
        }
    }
}
