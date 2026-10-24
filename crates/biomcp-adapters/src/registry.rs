//! Implemented source declarations are the sole registry for terms and source switches.
use biodata::{Source, Use};
use biomcp_fetch::SourceSpec;
use std::collections::BTreeSet;
/// Every implemented source, independent of quota-provider identities.
pub const SOURCES: &[&SourceSpec] = &[
    &crate::hgnc::SPEC, &crate::mygene::SPEC, &crate::oncokb::SPEC,
    &crate::gnomad::SPEC, &crate::uniprot::SPEC, &crate::reactome::SPEC,
    &crate::civic::SPEC, &crate::clingen::SPEC, &crate::hpa::SPEC,
    &crate::pubmed::SPEC, &crate::europepmc::SPEC, &crate::pubtator::SPEC,
    &crate::pmc_oa::SPEC, &crate::icite::SPEC,
];
/// Expand checked source names and the two user-selectable terms groups.
pub fn disabled(names: &[String]) -> Result<BTreeSet<Source>, ()> {
    let mut disabled = BTreeSet::new();
    for name in names {
        let group = match name.as_str() {
            "noncommercial" => Some(Use::Noncommercial),
            "restricted" => Some(Use::Restricted),
            _ => None,
        };
        if let Some(group) = group {
            disabled.extend(SOURCES.iter().filter(|s| s.terms.usage == group).map(|s| s.source));
        } else {
            let source = SOURCES.iter().find(|s| s.source.to_string() == *name).ok_or(())?;
            disabled.insert(source.source);
        }
    }
    Ok(disabled)
}
