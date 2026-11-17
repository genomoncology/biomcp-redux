//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::read::SPEC,
    display: "WHO Essential Medicines List",
    discovery_aliases: &["EML", "eEML"],
    registry_order: Some(61),
    probe: Probe::Get("/about", &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};
