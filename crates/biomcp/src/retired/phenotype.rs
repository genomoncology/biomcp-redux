//! Retired phenotype forms never rewrite text or choose a candidate.
use biomcp_types::{ErrorMessage, PhenotypeSource};
use std::ffi::OsString;
pub(super) fn message(root: &clap::Command, args: &[OsString]) -> Option<ErrorMessage> {
    let words = super::words(root, args)?;
    match words.positional.as_slice() {
        ["list", "phenotype", ..] => Some(ErrorMessage::PhenotypeHelp),
        ["search", "phenotype", ..] => {
            let mut args = args.iter().filter_map(|v| v.to_str());
            while let Some(word) = args.next() {
                if word == "--" {
                    break;
                }
                let source = if word == "--source" {
                    args.next()
                } else {
                    word.strip_prefix("--source=")
                };
                if source
                    .is_some_and(|value| value.eq_ignore_ascii_case(PhenotypeSource::Hpo.as_str()))
                {
                    return Some(ErrorMessage::PhenotypeHpoSearch);
                }
            }
            None
        }
        _ => None,
    }
}
