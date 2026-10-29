//! Exact published registry country labels, including source whitespace.
// Derived from all LocationCountry topValues; uniqueValuesCount equals list length.
// https://clinicaltrials.gov/api/v2/stats/field/values?fields=LocationCountry
// Retrieved 2026-10-29. Original SHA-256: 3f567bd9d90c544328ef322921c3484fa9cb30d5ab9f7875923f87021acc00db.
const NAMES: &str = include_str!("countries.txt");

pub(super) fn valid(value: &str) -> bool {
    NAMES.lines().any(|name| name == value)
}

#[cfg(feature = "schema")]
pub(super) fn schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    let values: Vec<_> = NAMES.lines().collect();
    schemars::json_schema!({
        "type": "array",
        "maxItems": super::MAX_ALTERNATIVES,
        "items": {"type": "string", "enum": values}
    })
}
