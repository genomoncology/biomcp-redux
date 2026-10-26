//! Schema discovery generated from checked-in artifact identities.
const SCHEMAS: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/schemas.rs"));

pub(crate) fn find(name: &str) -> Option<&'static str> {
    SCHEMAS
        .iter()
        .find_map(|(id, text)| (*id == name).then_some(*text))
}

pub(crate) fn versions() -> Vec<&'static str> {
    SCHEMAS
        .iter()
        .filter_map(|(id, _)| id.rsplit_once('/').map(|(_, version)| version))
        .collect::<std::collections::BTreeSet<_>>()
        .into_iter()
        .collect()
}
