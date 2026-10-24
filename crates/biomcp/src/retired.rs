//! Fixed migration guidance matched at command and section boundaries.
use biodata::{ErrorMessage, GeneSectionWord};
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
fn find(table: &[(&str, ErrorMessage)], word: &str) -> Option<ErrorMessage> {
    table
        .iter()
        .find_map(|(key, value)| (*key == word).then_some(*value))
}

pub(crate) fn message(args: &[OsString]) -> Option<ErrorMessage> {
    let words = words(args)?;
    match words.positional.as_slice() {
        ["gene", rest @ ..] if !rest.is_empty() => find(GENE, rest[0]).or(Some(ErrorMessage::Gene)),
        ["search", "gene", ..] => words.retired_option,
        ["get", "gene", input @ ..] if !input.is_empty() => {
            if !words.sections.is_empty() {
                return words.sections.iter().find_map(|word| find(SECTIONS, word));
            }
            input[1..]
                .iter()
                .rev()
                .take_while(|word| section(word))
                .find_map(|word| find(SECTIONS, word))
        }
        _ => None,
    }
}
fn section(word: &str) -> bool {
    find(SECTIONS, word).is_some()
        || serde_json::from_value::<GeneSectionWord>(serde_json::Value::String(word.into())).is_ok()
}
struct Words<'a> {
    positional: Vec<&'a str>,
    sections: Vec<&'a str>,
    retired_option: Option<ErrorMessage>,
}
fn words(args: &[OsString]) -> Option<Words<'_>> {
    let mut result = Words {
        positional: Vec::new(),
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
            literal = true;
            continue;
        }
        if !arg.starts_with('-') {
            result.positional.push(arg);
            continue;
        }
        let (flag, inline) = arg
            .split_once('=')
            .map_or((arg, None), |(key, value)| (key, Some(value)));
        if ["--fresh", "--offline"].contains(&flag) && inline.is_none() {
            continue;
        }
        let retired = find(SEARCH, flag);
        if retired.is_some() {
            result.retired_option = result.retired_option.or(retired);
        } else if ![
            "--replay",
            "--record",
            "--case-cache",
            "--section",
            "--band",
            "--locus-group",
            "--limit",
            "--offset",
        ]
        .contains(&flag)
        {
            return None;
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
