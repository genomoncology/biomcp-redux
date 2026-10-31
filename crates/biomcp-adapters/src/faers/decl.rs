//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
fn probe(leg: &biomcp_fetch::Leg<'_>) -> Result<(), biomcp_fetch::RequestError> {
crate::adverse_event_sources::components::meddra(leg)?;
(Probe::Get("", &[("limit", Arg::Num(1))])).run(&super::SPEC, leg)
}
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "FDA FAERS",
    registry_order: Some(57), probe: Probe::Custom(probe),
    components: || crate::adverse_event_sources::components::COMPONENTS.get(..1).unwrap_or_default(),
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};

pub(crate) static MEDDRA: SourceDecl = SourceDecl {
    spec: &crate::decl::component_spec(biomcp_types::Source::Meddra, &super::SPEC, crate::adverse_event_sources::components::COMPONENTS[0].terms),
    display: "MedDRA", registry_order: None, probe: Probe::Component,
    components: || &[], auxiliary: &[], bundles: &[], guidance: Guidance { deadline: None, quota: None }, key: None,
};
