//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "MyChem.info",
    registry_order: Some(44), probe: Probe::Get("/metadata", &[]),
    components: || crate::drug_sources::components::COMPONENTS,
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};

pub(crate) static DRUGBANKOPEN: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::DrugbankOpen, &super::SPEC, crate::drug_sources::components::COMPONENTS[0].terms),
    display: "DrugBank Open", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static DRUGCENTRAL: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Drugcentral, &super::SPEC, crate::drug_sources::components::COMPONENTS[2].terms),
    display: "DrugCentral", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static GTOPDB: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Gtopdb, &super::SPEC, crate::drug_sources::components::COMPONENTS[3].terms),
    display: "Guide to Pharmacology", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static UNII: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Unii, &super::SPEC, crate::drug_sources::components::COMPONENTS[4].terms),
    display: "FDA UNII", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static NDC: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Ndc, &super::SPEC, crate::drug_sources::components::COMPONENTS[5].terms),
    display: "FDA NDC", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};

pub(crate) static OPENFDAXREF: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::OpenfdaXref, &super::SPEC, crate::drug_sources::components::COMPONENTS[6].terms),
    display: "openFDA cross-reference", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};
