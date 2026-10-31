//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
fn probe(leg: &biomcp_fetch::Leg<'_>) -> Result<(), biomcp_fetch::RequestError> {
crate::adverse_event_sources::components::gmdn(leg)?;
(Probe::Get("", &[("limit", Arg::Num(1))])).run(&super::SPEC, leg)
}
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "FDA MAUDE",
    registry_order: Some(58), probe: Probe::Custom(probe),
    components: || crate::adverse_event_sources::components::COMPONENTS.get(1..).unwrap_or_default(),
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};

pub(crate) static GMDN: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Gmdn, &super::SPEC, crate::adverse_event_sources::components::COMPONENTS[1].terms),
    display: "GMDN", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};
