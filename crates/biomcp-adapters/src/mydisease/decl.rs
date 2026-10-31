//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "MyDisease.info",
    registry_order: Some(53), probe: Probe::Get("/metadata", &[]),
    components: || crate::disease_sources::components::COMPONENTS.get(3..8).unwrap_or_default(),
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};

pub(crate) static ORPHANET: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Orphanet, &super::SPEC, crate::disease_sources::components::COMPONENTS[6].terms),
    display: "Orphadata Science", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static OMIM: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Omim, &super::SPEC, crate::disease_sources::components::COMPONENTS[7].terms),
    display: "OMIM", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};
