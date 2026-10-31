//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "Ontology Lookup Service",
    registry_order: Some(52),
    probe: Probe::Get("/api/ontologies", &[("size", Arg::Num(1))]),
    components: &[COMPONENT_0, COMPONENT_1, COMPONENT_2],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

pub(crate) static MONDO: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(
        biomcp_types::Source::Mondo,
        &super::SPEC,
        crate::ols::decl::COMPONENT_0.terms,
    ),
    display: "MONDO",
    registry_order: None,
    probe: Probe::Component,
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

pub(crate) static DOID: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(
        biomcp_types::Source::Doid,
        &super::SPEC,
        crate::ols::decl::COMPONENT_1.terms,
    ),
    display: "Disease Ontology",
    registry_order: None,
    probe: Probe::Component,
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

pub(crate) static NCIT: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(
        biomcp_types::Source::Ncit,
        &super::SPEC,
        crate::ols::decl::COMPONENT_2.terms,
    ),
    display: "NCI Thesaurus",
    registry_order: None,
    probe: Probe::Component,
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

pub(crate) const COMPONENT_0: crate::registry::Component =
    crate::disease_sources::components::component(
        biomcp_types::Source::Ols,
        biomcp_types::Source::Mondo,
        "mondo",
        crate::disease_sources::components::MONDO,
    );

pub(crate) const COMPONENT_1: crate::registry::Component =
    crate::disease_sources::components::component(
        biomcp_types::Source::Ols,
        biomcp_types::Source::Doid,
        "doid",
        crate::disease_sources::components::DOID,
    );

pub(crate) const COMPONENT_2: crate::registry::Component =
    crate::disease_sources::components::component(
        biomcp_types::Source::Ols,
        biomcp_types::Source::Ncit,
        "ncit",
        crate::disease_sources::components::NCIT,
    );
