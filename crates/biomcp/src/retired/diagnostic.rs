//! Match command boundaries without inspecting or changing selector values.
use biomcp_types::ErrorMessage;
use std::ffi::OsString;
#[derive(Default)]
struct Words<'a> {
    positional: Vec<&'a str>,
    source: Option<&'a str>,
    section: Option<&'a str>,
    shared: Option<ErrorMessage>,
}
pub(super) fn message(args: &[OsString]) -> Option<ErrorMessage> {
    let mut words = Words::default();
    let mut literal = false;
    let mut args = args.iter();
    while let Some(word) = args.next() {
        let word = word.to_str()?;
        if literal || !word.starts_with('-') {
            words.positional.push(word);
            continue;
        }
        if word == "--" {
            literal = true;
            continue;
        }
        let (flag, inline) = word
            .split_once('=')
            .map_or((word, None), |(a, b)| (a, Some(b)));
        if matches!(flag, "-j" | "--json") {
            words.shared = Some(ErrorMessage::DiseaseJson);
        }
        let diagnostic = matches!(
            words.positional.as_slice(),
            ["search" | "get", "diagnostic", ..]
        );
        if diagnostic && let Some(message) = flag_message(flag) {
            return Some(message);
        }
        if matches!(flag, "--section" | "--query" | "--marker")
            || super::takes_value(flag, &words.positional).unwrap_or(false)
        {
            let value = inline.or_else(|| args.next().and_then(|v| v.to_str()));
            match flag {
                "--source" => words.source = value,
                "--section" => words.section = value,
                _ => (),
            }
        }
    }
    guidance(words)
}
fn flag_message(flag: &str) -> Option<ErrorMessage> {
    Some(match flag {
        "-g" => ErrorMessage::DiagnosticGene,
        "-t" => ErrorMessage::DiagnosticType,
        "-l" => ErrorMessage::DiseaseLimit,
        "-d" | "--disease" => ErrorMessage::DiagnosticCondition,
        "--manufacturer" | "--laboratory" => ErrorMessage::DiagnosticManufacturer,
        "--no-cache" => ErrorMessage::DiseaseFresh,
        "--full" => ErrorMessage::DiagnosticSection,
        "-j" | "--json" => ErrorMessage::DiseaseJson,
        _ => return None,
    })
}
fn guidance(words: Words<'_>) -> Option<ErrorMessage> {
    match words.positional.as_slice() {
        ["gtr", "sync", ..] => return Some(ErrorMessage::DiagnosticGtrSync),
        ["who-ivd", "sync", ..] => return Some(ErrorMessage::DiagnosticWhoSync),
        ["list", "diagnostic", ..] => return Some(ErrorMessage::DiagnosticList),
        _ => (),
    }
    let (verb, rest) = match words.positional.as_slice() {
        [verb @ ("search" | "get"), "diagnostic", rest @ ..] => (*verb, rest),
        _ => return None,
    };
    if words.shared.is_some() {
        return words.shared;
    }
    if words.source.is_none() || words.source == Some("all") {
        return Some(ErrorMessage::DiagnosticSource);
    }
    let section = words.section.or_else(|| rest.get(1).copied());
    if section == Some("all") {
        return Some(ErrorMessage::DiagnosticSection);
    }
    if section == Some("regulatory") && matches!(words.source, Some("gtr" | "who-ivd")) {
        return Some(ErrorMessage::DiagnosticRegulatory);
    }
    match (verb, words.source) {
        ("search", Some("who-ivd")) => Some(ErrorMessage::DiagnosticWho),
        ("search", Some("fda-pma")) => Some(ErrorMessage::DiagnosticPma),
        ("search", Some("fda-510k")) => Some(ErrorMessage::Diagnostic510k),
        _ => None,
    }
}
