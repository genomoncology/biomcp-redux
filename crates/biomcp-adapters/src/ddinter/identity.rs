use super::{NativeRow, PartRow};
use biodata::{DrugIdentity, DrugIdentityLevel, Text};
use biomcp_types::{DrugHeader, DrugSource};
use std::collections::BTreeSet;
/// Literal source-name candidates in native partition and row order.
pub(super) fn candidates(mut rows: Vec<PartRow>, query: &str) -> Vec<DrugHeader> {
    rows.sort_by(|a, b| (&a.part, a.row).cmp(&(&b.part, b.row)));
    let query = query.to_lowercase();
    let mut seen = BTreeSet::new();
    let mut items = Vec::new();
    for row in rows {
        for (id, name) in identities(&row.value) {
            if name.is_some_and(|name| name.to_lowercase().contains(&query))
                && seen.insert(id.to_owned())
                && let Ok(identity) = identity(id, name)
            {
                items.push(header(identity));
            }
        }
    }
    items
}
pub(super) fn identities(row: &NativeRow) -> [(&str, Option<&str>); 2] {
    [(row.id_a(), row.name_a()), (row.id_b(), row.name_b())]
}
pub(super) fn identity(id: &str, name: Option<&str>) -> Result<DrugIdentity, biodata::Error> {
    Ok(DrugIdentity {
        id: id.parse()?,
        source: Text::new("ddinter")?,
        name: name.map(Text::from_source).transpose()?,
        // The CSV describes drug pairs, without identifying ingredient/product level.
        identity_level: DrugIdentityLevel::Unknown,
        r#type: None,
        url: None,
    })
}
pub(super) fn header(identity: DrugIdentity) -> DrugHeader {
    DrugHeader {
        matched_name: None,
        id: identity.id,
        source: DrugSource::Ddinter,
        name: identity.name,
        identity_level: identity.identity_level,
        r#type: identity.r#type,
        url: identity.url,
        version: None,
    }
}
