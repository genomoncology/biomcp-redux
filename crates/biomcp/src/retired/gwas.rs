//! Explain failed checked parsing without reproducing GWAS input validation.
use biomcp_types::ErrorMessage;
use std::ffi::OsString;
pub(crate) fn usage(args: &[OsString]) -> Option<ErrorMessage> {
    let mut positional = Vec::new();
    let mut words = args.iter();
    while let Some(word) = words.next().and_then(|word| word.to_str()) {
        if word == "--" {
            return None;
        }
        if word.starts_with('-') {
            let (flag, inline) = word
                .split_once('=')
                .map_or((word, false), |(flag, _)| (flag, true));
            if !inline && super::takes_value(flag, &positional)? {
                words.next();
            }
            continue;
        }
        positional.push(word);
        match positional.as_slice() {
            ["search", "gwas"] => return Some(ErrorMessage::GwasSearchArguments),
            ["get", "gwas"] => return Some(ErrorMessage::GwasGetArguments),
            [_, _] => return None,
            _ => {}
        }
    }
    None
}
