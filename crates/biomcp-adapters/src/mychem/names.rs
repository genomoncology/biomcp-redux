//! Complete native names used by the same MyChem search source.
use super::{links::strings, raw::Component};
use biomcp_types::DrugNameMatch;
pub(super) fn name(c: &Component) -> Option<&str> {
    c.name
        .as_deref()
        .or(c.pref_name.as_deref())
        .or(c.display_name.as_deref())
        .or(c.proprietaryname.as_deref())
        .or(c.nonproprietaryname.as_deref())
}
pub(super) fn matching(rows: &[(&str, Component)], query: &str) -> Option<DrugNameMatch> {
    let exact = |s: &str| DrugNameMatch::same_name(s, query);
    let brand = rows.iter().any(|(_, c)| {
        let real_brand = |brand: &str| {
            exact(brand)
                && !generic_names(c)
                    .into_iter()
                    .flatten()
                    .any(|g| DrugNameMatch::same_name(brand, g))
                && !c.generic_name.as_ref().is_some_and(|v| {
                    strings(v)
                        .into_iter()
                        .any(|g| DrugNameMatch::same_name(brand, g))
                })
        };
        c.proprietaryname.as_deref().is_some_and(real_brand)
            || c.brand_name
                .as_ref()
                .is_some_and(|v| strings(v).into_iter().any(real_brand))
    });
    let generic = rows.iter().any(|(_, c)| {
        generic_names(c)
            .into_iter()
            .flatten()
            .any(|name| DrugNameMatch::generic_name(name, query))
            || c.generic_name.as_ref().is_some_and(|v| {
                strings(v)
                    .into_iter()
                    .any(|name| DrugNameMatch::generic_name(name, query))
            })
            || c.synonyms
                .as_ref()
                .is_some_and(|v| strings(v).into_iter().any(exact))
            || c.molecule_synonyms
                .iter()
                .flatten()
                .any(|s| s.molecule_synonym.as_deref().is_some_and(exact))
    });
    DrugNameMatch::from_fields(brand, generic)
}

/// Every name the rows state, with source values that join several names split apart.
pub(super) fn stated(rows: &[(&str, Component)]) -> Vec<String> {
    rows.iter()
        .flat_map(|(_, c)| {
            generic_names(c)
                .into_iter()
                .chain([c.proprietaryname.as_deref()])
                .flatten()
                .flat_map(split)
                .map(str::to_owned)
                .chain(super::map::aliases_for(c))
        })
        .collect()
}

/// One source value may hold several names joined by a pipe.
pub(super) fn split(value: &str) -> impl Iterator<Item = &str> {
    value
        .split('|')
        .map(str::trim)
        .filter(|name| !name.is_empty())
}

/// The structure a row describes: an InChIKey's skeleton, so stereo forms of one molecule agree.
/// A row with no chemical identity of its own, such as an NDC product row, has none.
pub(super) fn structure(id: &str, rows: &[(&str, Component)]) -> Option<String> {
    let blocks: Vec<_> = id.split('-').collect();
    if let [skeleton, stereo, charge] = blocks[..]
        && (skeleton.len(), stereo.len(), charge.len()) == (14, 10, 1)
        && id.chars().all(|c| c == '-' || c.is_ascii_uppercase())
    {
        return Some(skeleton.to_owned());
    }
    rows.iter()
        .any(|(component, _)| !matches!(*component, "ndc" | "openfda"))
        .then(|| id.to_owned())
}

fn generic_names(c: &Component) -> [Option<&str>; 4] {
    [
        c.name.as_deref(),
        c.pref_name.as_deref(),
        c.display_name.as_deref(),
        c.nonproprietaryname.as_deref(),
    ]
}
