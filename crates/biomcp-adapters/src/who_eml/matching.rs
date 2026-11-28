//! Resolve full and alternate WHO eEML names among source search hits.
use biomcp_types::{DrugHeader, DrugNameMatch};

pub(super) fn matching_medicine<'a>(hits: &'a [DrugHeader], name: &str) -> Option<&'a DrugHeader> {
    let name = name.trim();
    let named = hits
        .iter()
        .filter_map(|header| Some((header.name.as_ref()?.as_str(), header)));
    if let Some((_, exact)) = named
        .clone()
        .find(|(title, _)| DrugNameMatch::same_name(title, name))
    {
        return Some(exact);
    }
    let mut aliases = named.filter(|(title, _)| alternate_name(title, name));
    let (_, first) = aliases.next()?;
    aliases
        .all(|(_, header)| header.id == first.id)
        .then_some(first)
}

fn alternate_name(title: &str, name: &str) -> bool {
    if DrugNameMatch::generic_name(title, name) {
        return true;
    }
    let bracketed = title
        .strip_suffix(')')
        .and_then(|title| title.split_once(" ("))
        .or_else(|| {
            title
                .strip_suffix(']')
                .and_then(|title| title.split_once(" ["))
        });
    bracketed.is_some_and(|(base, alternate)| {
        DrugNameMatch::generic_name(base, name)
            || (DrugNameMatch::generic_name(alternate, name) && !qualifier(alternate))
    })
}

fn qualifier(value: &str) -> bool {
    // eEML also brackets formulations, sources and other qualifiers.
    let value = value.to_ascii_lowercase();
    value.contains(',')
        || value.starts_with("as ")
        || matches!(
            value.as_str(),
            "injection"
                | "native"
                | "pathogen-reduced"
                | "seasonal"
                | "high-viscosity"
                | "low-viscosity"
        )
}
