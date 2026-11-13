//! Retired phenotype forms never rewrite text or choose a candidate.
use biomcp_types::ErrorMessage;
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
                if word == "--source=hpo" || (word == "--source" && args.next() == Some("hpo")) {
                    return Some(ErrorMessage::PhenotypeHpoSearch);
                }
            }
            None
        }
        _ => None,
    }
}
