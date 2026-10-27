//! Registry-derived consequences of supported source-disable groups.
use super::{SOURCES, components};
use biomcp_types::{DisableGroup, DisabledComponent, Use};
use std::collections::BTreeSet;

pub(super) const USES: &[Use] = &[Use::Open, Use::Noncommercial, Use::Restricted];

/// List whole-source exclusions separately from independently labeled carried fields.
pub fn disable_groups() -> Vec<DisableGroup> {
    USES.iter().map(|&group| DisableGroup {
        group,
        sources: SOURCES.iter()
            .filter(|s| s.terms.usage == group && !components().any(|c| c.source == s.source))
            .map(|s| s.source).collect::<BTreeSet<_>>().into_iter().collect(),
        components: components().filter(|c| c.terms.usage == group)
            .map(|c| DisabledComponent { source: c.source, name: c.name.into() }).collect(),
    }).collect()
}
