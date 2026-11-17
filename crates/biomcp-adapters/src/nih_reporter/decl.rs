//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::search::SPEC,
    display: "NIH RePORTER",
    discovery_aliases: &["RePORTER"],
    registry_order: Some(64),
    probe: Probe::Custom(super::search::probe),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};
