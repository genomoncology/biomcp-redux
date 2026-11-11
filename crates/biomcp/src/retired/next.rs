//! The replacement for a retired spelling, written out for the request it came in.
//!
//! The candidate is the same request with the retired option renamed, or with an option
//! that was dropped left out. It is offered only when the grammar accepts it and nothing
//! in it is retired, so the next call never repeats the refusal.
use biomcp_types::{Step, StepKind};
use std::ffi::OsString;

/// Retired option spellings and the option that took their place.
const RENAMES: &[(&str, &str)] = &[
    ("-k", "--query"),
    ("-g", "--gene"),
    ("--region", "--source"),
    ("--type", "--source"),
];
/// Region words and the source that now answers for them.
const REGIONS: &[(&str, &str)] = &[("who", "who-pq"), ("eu", "ema"), ("europe", "ema")];
/// Options whose search was dropped; the request runs without them.
const DROPPED: &[&str] = &["--drug", "-d"];

/// The next call for a retired request, when one checks.
pub(crate) fn steps(root: &clap::Command, args: &[OsString]) -> Vec<Step> {
    let renamed = rename(args);
    let candidates = [Some(renamed.clone()), drop(&renamed), Some(args.to_vec())];
    candidates
        .into_iter()
        .flatten()
        .find_map(|candidate| checked(root, &candidate))
        .into_iter()
        .collect()
}

fn rename(args: &[OsString]) -> Vec<OsString> {
    let has_source = args.iter().any(|word| {
        word.to_str()
            .is_some_and(|word| word == "--source" || word.starts_with("--source="))
    });
    let mut result = Vec::with_capacity(args.len());
    let mut words = args.iter();
    while let Some(word) = words.next() {
        let renamed = word.to_str().and_then(|text| {
            let (_, to) = RENAMES.iter().find(|(from, _)| *from == text)?;
            (*to != "--source" || !has_source).then_some(*to)
        });
        let Some(to) = renamed else {
            result.push(word.clone());
            continue;
        };
        result.push(to.into());
        if let Some(value) = words.next() {
            let text = value.to_str().unwrap_or_default().to_ascii_lowercase();
            let mapped = REGIONS
                .iter()
                .find(|(region, _)| *region == text)
                .filter(|_| word.to_str() == Some("--region"))
                .map_or_else(|| value.clone(), |(_, source)| (*source).into());
            result.push(mapped);
        }
    }
    result
}

fn drop(args: &[OsString]) -> Option<Vec<OsString>> {
    let index = args
        .iter()
        .position(|word| word.to_str().is_some_and(|word| DROPPED.contains(&word)))?;
    let mut args = args.to_vec();
    args.drain(index..(index + 2).min(args.len()));
    Some(args)
}

/// The canonical call for a candidate the grammar accepts, or the corrected call its
/// refusal already names, when that call is not retired too.
fn checked(root: &clap::Command, args: &[OsString]) -> Option<Step> {
    let step = match crate::grammar::parse(root, args) {
        Ok(parsed) => match parsed.action? {
            action @ biomcp_query::Action::Run { .. } => {
                crate::spell::step(StepKind::Configure, &action).ok()?
            }
            _ => return None,
        },
        Err(cause) => match cause.usage? {
            biomcp_types::UsageError::Arguments(arguments) => arguments
                .next
                .into_iter()
                .next()
                .filter(|step| step.args.first().is_some_and(|word| &**word != "help"))?,
            _ => return None,
        },
    };
    let words = step
        .args
        .iter()
        .map(|word| OsString::from(&**word))
        .collect::<Vec<_>>();
    super::message(root, &words).is_none().then_some(step)
}
