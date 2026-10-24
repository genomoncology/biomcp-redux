//! Fixed migration guidance matched at command and section boundaries.
use biodata::{ArticleSectionWord, ErrorMessage, GeneSectionWord};
use std::ffi::OsString;

const GENE: &[(&str, ErrorMessage)] = &[
    ("pathways", ErrorMessage::Pathways),
    ("trials", ErrorMessage::Trials),
    ("articles", ErrorMessage::Articles),
    ("drugs", ErrorMessage::Drugs),
    ("cell-lines", ErrorMessage::CellLines),
    ("cspec", ErrorMessage::Variant),
];
const SEARCH: &[(&str, ErrorMessage)] = &[
    ("--type", ErrorMessage::LocusGroup),
    ("--chromosome", ErrorMessage::Band),
    ("--region", ErrorMessage::Dropped),
    ("--pathway", ErrorMessage::Dropped),
    ("--go", ErrorMessage::Dropped),
];
const SECTIONS: &[(&str, ErrorMessage)] = &[
    ("hpa", ErrorMessage::Expression),
    ("ontology", ErrorMessage::Dropped),
    ("diseases", ErrorMessage::Dropped),
    ("go", ErrorMessage::Dropped),
    ("interactions", ErrorMessage::Dropped),
    ("funding", ErrorMessage::Dropped),
    ("disgenet", ErrorMessage::Dropped),
    ("druggability", ErrorMessage::Drug),
    ("diagnostics", ErrorMessage::Diagnostic),
    ("gencc", ErrorMessage::Deferred),
];
const ARTICLE_SEARCH: &[(&str, ErrorMessage)] = &[
    ("-k", ErrorMessage::ArticleText),
    ("-q", ErrorMessage::ArticleText),
    ("--query", ErrorMessage::ArticleText),
    ("--date-from", ErrorMessage::Since),
    ("--year-min", ErrorMessage::Since),
    ("--date-to", ErrorMessage::Until),
    ("--year-max", ErrorMessage::Until),
    ("--weight-citations", ErrorMessage::CitationSort),
    ("-g", ErrorMessage::GeneOption),
    ("-d", ErrorMessage::DiseaseOption),
    ("-a", ErrorMessage::AuthorOption),
    ("--ranking-mode", ErrorMessage::ScoreBlends),
    ("--weight-semantic", ErrorMessage::ScoreBlends),
    ("--weight-lexical", ErrorMessage::ScoreBlends),
    ("--weight-position", ErrorMessage::ScoreBlends),
    ("--max-per-source", ErrorMessage::Dropped),
    ("--session", ErrorMessage::Dropped),
    ("--debug-plan", ErrorMessage::Dropped),
    ("--full", ErrorMessage::Dropped),
];
const TRIAL_SEARCH: &[(&str, ErrorMessage)] = &[
    ("--line-of-therapy", ErrorMessage::TrialCriteria),
    ("--prior-therapies", ErrorMessage::TrialCriteria),
    ("--progression-on", ErrorMessage::TrialCriteria),
    ("--mutation", ErrorMessage::TrialVariant),
    ("--biomarker", ErrorMessage::TrialVariant),
    ("--date-from", ErrorMessage::TrialSince),
    ("--date-to", ErrorMessage::TrialUntil),
    ("--results-available", ErrorMessage::TrialResults),
    ("--offset", ErrorMessage::TrialPage),
    ("--no-alias-expand", ErrorMessage::TrialNoExpansion),
];
const ARTICLE_GET: &[(&str, ErrorMessage)] = &[
    ("--pdf", ErrorMessage::ArticlePdf),
    ("--outline", ErrorMessage::Fulltext),
    ("--lines", ErrorMessage::ArticlePart),
    ("--out", ErrorMessage::Dropped),
    ("--output", ErrorMessage::Dropped),
    ("--asset-view", ErrorMessage::Dropped),
    ("--asset-limit", ErrorMessage::Dropped),
    ("--asset-offset", ErrorMessage::Dropped),
];
const ARTICLE_SECTIONS: &[(&str, ErrorMessage)] = &[
    ("assets", ErrorMessage::Supplements),
    ("asset", ErrorMessage::Supplement),
];
fn find(table: &[(&str, ErrorMessage)], word: &str) -> Option<ErrorMessage> {
    table
        .iter()
        .find_map(|(key, value)| (*key == word).then_some(*value))
}

pub(crate) fn message(args: &[OsString]) -> Option<ErrorMessage> {
    let words = words(args)?;
    if let Some(message) = command(&words.positional) {
        return Some(message);
    }
    match words.positional.as_slice() {
        ["gene", rest @ ..] if !rest.is_empty() => {
            find(GENE, rest.first()?).or(Some(ErrorMessage::Gene))
        }
        ["search", "gene", ..] | ["search", "trial", ..] => words.retired_option,
        ["get", "gene", input @ ..] => trailing(&words, input, SECTIONS, false),
        ["search", "article", ..] => words.retired_option,
        ["get", "article", input @ ..] => words
            .retired_option
            .or_else(|| trailing(&words, input, ARTICLE_SECTIONS, true)),
        _ => None,
    }
}
fn command(positional: &[&str]) -> Option<ErrorMessage> {
    match positional {
        ["gene", word] => find(GENE, word),
        ["article", "supplement-search", ..] => Some(ErrorMessage::SupplementFind),
        ["article", "entities", ..] => Some(ErrorMessage::Annotations),
        ["article", "batch" | "authors", ..] | ["batch", "article", ..] => {
            Some(ErrorMessage::ArticleBatch)
        }
        ["variant", "articles", ..] => Some(ErrorMessage::VariantArticles),
        ["disease", "articles", ..] => Some(ErrorMessage::DiseaseArticles),
        ["drug", "articles", ..] => Some(ErrorMessage::DrugArticles),
        ["pathway", "articles", ..] => Some(ErrorMessage::Pathway),
        _ => None,
    }
}
fn trailing(
    words: &Words<'_>,
    input: &[&str],
    table: &[(&str, ErrorMessage)],
    article: bool,
) -> Option<ErrorMessage> {
    if !words.sections.is_empty() {
        return words.sections.iter().find_map(|word| find(table, word));
    }
    if words.literal {
        return None;
    }
    input
        .get(1..)
        .unwrap_or_default()
        .iter()
        .rev()
        .take_while(|word| {
            find(table, word).is_some()
                || if article {
                    serde_json::from_value::<ArticleSectionWord>(serde_json::Value::String(
                        (**word).into(),
                    ))
                    .is_ok()
                } else {
                    serde_json::from_value::<GeneSectionWord>(serde_json::Value::String(
                        (**word).into(),
                    ))
                    .is_ok()
                }
        })
        .find_map(|word| find(table, word))
}
struct Words<'a> {
    positional: Vec<&'a str>,
    literal: bool,
    sections: Vec<&'a str>,
    retired_option: Option<ErrorMessage>,
}
fn words(args: &[OsString]) -> Option<Words<'_>> {
    let mut result = Words {
        positional: Vec::new(),
        literal: false,
        sections: Vec::new(),
        retired_option: None,
    };
    let mut args = args.iter();
    let mut literal = false;
    while let Some(arg) = args.next() {
        let arg = arg.to_str()?;
        if literal {
            result.positional.push(arg);
            continue;
        }
        if arg == "--" {
            if result.positional.len() < 2 {
                return None;
            }
            literal = true;
            result.literal = true;
            continue;
        }
        if !arg.starts_with('-') {
            result.positional.push(arg);
            if command(&result.positional).is_some() {
                return Some(result);
            }
            continue;
        }
        let (flag, inline) = arg
            .split_once('=')
            .map_or((arg, None), |(key, value)| (key, Some(value)));
        let table = match result.positional.get(..2) {
            Some(["search", "article"]) => ARTICLE_SEARCH,
            Some(["search", "trial"]) => TRIAL_SEARCH,
            Some(["get", "article"]) => ARTICLE_GET,
            Some(["search", "gene"]) => SEARCH,
            _ => &[],
        };
        let retired = find(table, flag);
        if retired.is_some() {
            result.retired_option = result.retired_option.or(retired);
        } else if !takes_value(flag, &result.positional)? && inline.is_none() {
            continue;
        }
        let value = inline.or_else(|| args.next().and_then(|arg| arg.to_str()));
        if flag == "--section" {
            result.sections.push(value?);
        }
        if value.is_none() && retired.is_none() {
            return None;
        }
    }
    Some(result)
}

fn takes_value(flag: &str, positional: &[&str]) -> Option<bool> {
    let globals = crate::grammar::global_arguments();
    if let Some(argument) = globals
        .get_arguments()
        .find(|argument| argument.get_long() == flag.strip_prefix("--"))
    {
        return Some(argument.get_action().takes_values());
    }
    let Some([verb, entity]) = positional.get(..2) else {
        return None;
    };
    let (_, _, id) = crate::grammar::QUERIES
        .iter()
        .find(|(v, e, _)| v == verb && e == entity)?;
    crate::schema::find(id)
        .and_then(|text| serde_json::from_str::<serde_json::Value>(text).ok())
        .and_then(|schema| {
            schema
                .get("properties")
                .and_then(serde_json::Value::as_object)
                .and_then(|properties| {
                    properties
                        .iter()
                        .find(|(name, property)| {
                            property.get("x-positional").is_none()
                                && flag == format!("--{}", name.replace('_', "-"))
                        })
                        .map(|(_, property)| {
                            property.get("type").and_then(serde_json::Value::as_str)
                                != Some("boolean")
                        })
                })
        })
}
