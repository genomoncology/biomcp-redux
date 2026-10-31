//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "Ontology Lookup Service",
    registry_order: Some(52), probe: Probe::Get("/api/ontologies", &[("size", Arg::Num(1))]),
    components: || crate::disease_sources::components::COMPONENTS.get(..3).unwrap_or_default(),
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};

pub(crate) static MONDO: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Mondo, &super::SPEC, crate::disease_sources::components::COMPONENTS[0].terms),
    display: "MONDO", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static DOID: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Doid, &super::SPEC, crate::disease_sources::components::COMPONENTS[1].terms),
    display: "Disease Ontology", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static NCIT: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Ncit, &super::SPEC, crate::disease_sources::components::COMPONENTS[2].terms),
    display: "NCI Thesaurus", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};
