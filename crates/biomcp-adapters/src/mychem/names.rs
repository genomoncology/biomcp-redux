//! Complete native names used by the same MyChem search source.
use super::{raw::Component, links::strings};
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
    let brand = rows.iter().any(|(_, c)| c.proprietaryname.as_deref().is_some_and(exact)
        || c.brand_name.as_ref().is_some_and(|v| strings(v).into_iter().any(exact)));
    let generic = rows.iter().any(|(_, c)| name(c).is_some_and(exact)
        || c.nonproprietaryname.as_deref().is_some_and(exact)
        || c.generic_name.as_ref().is_some_and(|v| strings(v).into_iter().any(exact))
        || c.synonyms.as_ref().is_some_and(|v| strings(v).into_iter().any(exact))
        || c.molecule_synonyms.iter().flatten().any(|s| s.molecule_synonym.as_deref().is_some_and(exact)));
    DrugNameMatch::from_fields(brand, generic)
}
