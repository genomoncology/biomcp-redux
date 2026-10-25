//! Disease migration is fixed guidance, never a source request or query rewrite.
use biomcp_types::ErrorMessage;
use std::ffi::OsString;
const FLAGS: &[(&str, ErrorMessage)] = &[
    ("-q", ErrorMessage::DiseaseMigration0),
    ("--query", ErrorMessage::DiseaseMigration0),
    ("--inheritance", ErrorMessage::DiseaseMigration1),
    ("--phenotype", ErrorMessage::DiseaseMigration2),
    ("--onset", ErrorMessage::DiseaseMigration3),
    ("--no-fallback", ErrorMessage::DiseaseMigration4),
    ("-l", ErrorMessage::DiseaseLimit),
    ("-j", ErrorMessage::DiseaseJson),
    ("--json", ErrorMessage::DiseaseJson),
    ("--no-cache", ErrorMessage::DiseaseFresh),
];
const SECTIONS: &[(&str, ErrorMessage)] = &[
    ("pathways", ErrorMessage::DiseaseMigration9),
    ("diagnostics", ErrorMessage::DiseaseMigration10),
    ("variants", ErrorMessage::DiseaseMigration11),
    ("models", ErrorMessage::DiseaseMigration12),
    ("prevalence", ErrorMessage::DiseaseMigration13),
    ("funding", ErrorMessage::DiseaseMigration14),
    ("civic", ErrorMessage::DiseaseMigration15),
    ("disgenet", ErrorMessage::DiseaseMigration16),
    ("all", ErrorMessage::DiseaseMigration17),
    ("clinical_features", ErrorMessage::DiseaseMigration18),
];
pub(super) fn message(args: &[OsString]) -> Option<ErrorMessage> {
    let mut positional = Vec::new();
    let mut source = None;
    let mut section = None;
    let mut option = None;
    let mut literal = false;
    let mut args = args.iter();
    while let Some(word) = args.next() {
        let word = word.to_str()?;
        if literal || !word.starts_with('-') {
            positional.push(word);
            continue;
        }
        if word == "--" {
            literal = true;
            continue;
        }
        let (flag, inline) = word.split_once('=').map_or((word, None), |(a,b)| (a, Some(b)));
        let disease = matches!(positional.as_slice(), ["search" | "get", "disease", ..]);
        if disease && let Some(found) = super::find(FLAGS, flag) {
            return Some(found);
        }
        if disease && flag == "--name" {
            option = Some(ErrorMessage::DiseaseMigration19);
        }
        if flag == "--name" || super::takes_value(flag, &positional).unwrap_or(false) {
            let value = inline.or_else(|| args.next().and_then(|v| v.to_str()));
            match flag {
                "--source" => source = value,
                "--section" => section = value,
                _ => (),
            }
        }
    }
    match positional.as_slice() {
        ["list", "disease", ..] => Some(ErrorMessage::DiseaseMigration24),
        ["search", "phenotype", ..] => Some(ErrorMessage::DiseaseMigration23),
        ["disease", "drugs", ..] => Some(ErrorMessage::DiseaseMigration22),
        ["search", "disease", ..] if matches!(source, Some("mondo" | "doid" | "mesh")) => Some(ErrorMessage::DiseaseMigration8),
        ["get", "disease", id, rest @ ..] => {
            let section = section.or_else(|| (!literal).then(|| rest.first().copied()).flatten());
            if section == Some("genes") && source == Some("opentargets") {
                return Some(ErrorMessage::DiseaseMigration21);
            }
            if section == Some("survival") && source != Some("seer") {
                return Some(ErrorMessage::DiseaseMigration20);
            }
            if let Some(retired) = section.and_then(|s| super::find(SECTIONS, s)) {
                return Some(retired);
            }
            if !id.contains(':') && source != Some("seer") {
                return Some(ErrorMessage::DiseaseMigration19);
            }
            option
        }
        _ => option,
    }
}
