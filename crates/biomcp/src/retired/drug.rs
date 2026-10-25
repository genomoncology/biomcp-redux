//! Drug migration guidance runs before parsing or source admission.
use super::{Words, find};
use biomcp_types::{DrugSection, ErrorMessage};
pub(super) const SEARCH: &[(&str, ErrorMessage)] = &[
    ("--query", ErrorMessage::DrugQuery),
    ("--mechanism", ErrorMessage::DrugMechanism),
    ("--interactions", ErrorMessage::DrugPartnerSearch),
    ("--region", ErrorMessage::DrugRegion),
    ("--product-type", ErrorMessage::DrugProductType),
    ("--target", ErrorMessage::DrugUnproved),
    ("--indication", ErrorMessage::DrugUnproved),
    ("--type", ErrorMessage::DrugUnproved),
    ("--atc", ErrorMessage::DrugUnproved),
    ("--pharm-class", ErrorMessage::DrugUnproved),
];
pub(super) const GET: &[(&str, ErrorMessage)] = &[
    ("--name", ErrorMessage::DrugName),
    ("--region", ErrorMessage::DrugRegion),
    ("--raw", ErrorMessage::DrugRaw),
];
const SECTIONS: &[(&str, ErrorMessage)] = &[
    ("safety", ErrorMessage::DrugSafety),
    ("shortage", ErrorMessage::DrugShortage),
    ("all", ErrorMessage::DrugAll),
    ("cell_lines", ErrorMessage::DrugCellLines),
    ("cell-lines", ErrorMessage::DrugCellLines),
];
pub(super) fn command(words: &[&str]) -> Option<ErrorMessage> {
    match words {
        ["drug", "cell-lines", ..] => Some(ErrorMessage::DrugCellLines),
        ["drug", "interactions", ..] => Some(ErrorMessage::DrugInteractions),
        ["cvx", "sync", ..] => Some(ErrorMessage::CvxUnavailable),
        ["ema", "sync", ..] => Some(ErrorMessage::EmaSync),
        ["ddinter", "sync", ..] => Some(ErrorMessage::DdinterSync),
        ["who", "sync", ..] => Some(ErrorMessage::WhoSync),
        ["gene", "drugs", ..] => Some(ErrorMessage::DrugUnproved),
        _ => None,
    }
}
pub(super) fn section(words: &Words<'_>, input: &[&str]) -> Option<ErrorMessage> {
    if !words.sections.is_empty() {
        return words.sections.iter().find_map(|word| find(SECTIONS, word));
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
            find(SECTIONS, word).is_some()
                || serde_json::from_value::<DrugSection>(serde_json::Value::String((**word).into()))
                    .is_ok()
        })
        .find_map(|word| find(SECTIONS, word))
}
