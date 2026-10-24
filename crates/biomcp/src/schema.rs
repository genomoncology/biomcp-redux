//! Schema discovery generated from checked-in artifact identities.
const SCHEMAS: &[(&str, &str)] = include!(concat!(env!("OUT_DIR"), "/schemas.rs"));

pub(crate) fn find(name: &str) -> Option<&'static str> {
    SCHEMAS
        .iter()
        .find_map(|(id, text)| (*id == name).then_some(*text))
}

pub(crate) fn names() -> impl Iterator<Item = &'static str> {
    SCHEMAS.iter().map(|(name, _)| *name)
}
