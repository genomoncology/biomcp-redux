//! One way to page a list (1061): `limit` and `offset` page every list the reply returns.
//!
//! A section's `section_limit` and `section_offset` are other names for the same option, so a
//! command that has only one pair takes the other pair's names too. `get variant` has both: with
//! a named section, `limit` and `offset` page that section; without one they page candidates.
use serde_json::{Map, Value};

/// The two names of each paging option: the list name and the section name.
pub(crate) const PAIRS: [(&str, &str); 2] =
    [("limit", "section_limit"), ("offset", "section_offset")];

/// The schema field a paging name stands for in this command, when it is not its own field.
pub(crate) fn other_name(schema: &Value, name: &str) -> Option<&'static str> {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    PAIRS.iter().find_map(|&(list, section)| {
        if name == list && has(section) && !has(list) {
            Some(section)
        } else if name == section && has(list) && !has(section) && has("section") {
            Some(list)
        } else {
            None
        }
    })
}

/// How the help line naming a command's paging options starts.
pub(crate) const HELP_PREFIX: &str = "Paging: ";

/// The one help line that names the options paging this command's lists.
pub(crate) fn help_line(schema: &Value) -> Option<String> {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    let flag = |field: &str| format!("--{}", field.replace('_', "-"));
    let mut names: Vec<_> = PAIRS
        .iter()
        .filter(|(list, section)| has(list) || has(section))
        .map(|(list, _)| flag(list))
        .collect();
    if has("page") {
        names.push(flag("page"));
    }
    if names.is_empty() {
        return None;
    }
    let names = names.join(" and ");
    let others: Vec<_> = PAIRS
        .iter()
        .filter(|(_, section)| has(section) || other_name(schema, section).is_some())
        .map(|(_, section)| flag(section))
        .collect();
    let same = if others.len() == 1 {
        "is the same option"
    } else {
        "are the same options"
    };
    let others = others.join(" and ");
    Some(if both(schema) {
        format!(
            "{HELP_PREFIX}{names} page the named section, else the candidates; \
            --candidate-limit and --candidate-offset page the candidates when a section is named."
        )
    } else if others.is_empty() {
        format!("{HELP_PREFIX}{names}.")
    } else {
        format!("{HELP_PREFIX}{names}; {others} {same}.")
    })
}

/// A command with both pairs (`get variant`) also takes these names for its candidate list,
/// so a caller who names a section can still cap the candidates.
pub(crate) const CANDIDATES: [(&str, &str); 2] =
    [("candidate_limit", "limit"), ("candidate_offset", "offset")];

/// Read the candidate options a parsed `get variant` call gave, then move every paging value
/// onto its field; `None` for a malformed value.
pub(crate) fn read(
    matches: &clap::ArgMatches,
    schema: &Value,
    fields: &mut Map<String, Value>,
) -> Option<()> {
    for (candidate, _) in CANDIDATES {
        if let Ok(Some(value)) = matches.try_get_one::<String>(candidate) {
            fields.insert(candidate.into(), value.parse::<u64>().ok()?.into());
        }
    }
    rename(schema, fields);
    Some(())
}

/// Whether this command pages candidates and a named section with separate pairs.
pub(crate) fn both(schema: &Value) -> bool {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    PAIRS
        .iter()
        .all(|(list, section)| has(list) && has(section))
}

/// Move each paging value onto the field this command reads it from.
pub(crate) fn rename(schema: &Value, fields: &mut Map<String, Value>) {
    let has = |field: &str| schema.pointer(&format!("/properties/{field}")).is_some();
    let sectioned = fields.get("section").is_some_and(named);
    for (list, section) in PAIRS {
        let (from, to) = if has(list) && has(section) {
            if !sectioned {
                continue;
            }
            (list, section)
        } else if let Some(to) = other_name(schema, list) {
            (list, to)
        } else if let Some(to) = other_name(schema, section) {
            (section, to)
        } else {
            continue;
        };
        if !fields.contains_key(to)
            && let Some(value) = fields.remove(from)
        {
            fields.insert(to.into(), value);
        }
    }
    if both(schema) {
        for (candidate, list) in CANDIDATES {
            if !fields.contains_key(list)
                && let Some(value) = fields.remove(candidate)
            {
                fields.insert(list.into(), value);
            }
        }
    }
}

/// The option name taught for a paging field in a call with these values: `--limit` and
/// `--offset` for whatever list the call pages, and on `get variant` with a named section,
/// `--candidate-limit` and `--candidate-offset` for the candidates.
pub(crate) fn flag(schema: &Value, values: &Value, field: &str) -> String {
    let taught = if both(schema) {
        let sectioned = values.get("section").is_some_and(named);
        match (sectioned, field) {
            (true, "section_limit") => "limit",
            (true, "section_offset") => "offset",
            (true, "limit") => "candidate_limit",
            (true, "offset") => "candidate_offset",
            _ => field,
        }
    } else {
        PAIRS
            .iter()
            .find(|&&(list, section)| field == section && other_name(schema, list) == Some(section))
            .map_or(field, |(list, _)| list)
    };
    format!("--{}", taught.replace('_', "-"))
}

fn named(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Array(values) => !values.is_empty(),
        _ => true,
    }
}

/// The paging options a command may have, and other words callers try for them.
const OPTIONS: [&str; 7] = [
    "limit",
    "offset",
    "section-limit",
    "section-offset",
    "page",
    "candidate-limit",
    "candidate-offset",
];
const GUESSES: [&str; 5] = ["page-size", "per-page", "size", "skip", "rows"];

/// The name a refusal of `spelling` lists for the option `name`, if any. A paging word the
/// command lacks is answered with the paging options it has, under the names taught for them
/// (`--limit` and `--offset` where the section names are the real ones), so the refusal names
/// the working form; any other refused word, or a command without paging, lists every option.
pub(crate) fn listed<'a>(
    command: &clap::Command,
    spelling: &str,
    name: &'a str,
) -> Option<&'a str> {
    let word = spelling.trim_start_matches('-');
    let paging = |name: &str| OPTIONS.contains(&name);
    let longs: Vec<_> = command
        .get_arguments()
        .filter(|argument| !argument.is_hide_set())
        .filter_map(clap::Arg::get_long)
        .collect();
    if !(longs.iter().any(|long| paging(long)) && (paging(word) || GUESSES.contains(&word))) {
        return Some(name);
    }
    let taught = match name {
        "section-limit" if !longs.contains(&"limit") => "limit",
        "section-offset" if !longs.contains(&"offset") => "offset",
        _ => name,
    };
    paging(name).then_some(taught)
}
