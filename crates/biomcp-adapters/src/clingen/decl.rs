//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "ClinGen",
    registry_order: Some(30), probe: Probe::Get(crate::clingen::HEALTH_PATH, &[]),
    components: || &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    name: "clingen", source: biomcp_types::Source::ClinGen, order: 1,
    sync: |fetch| super::sync(fetch).map(|mapped| crate::decl::BundleReply::Standard(mapped, false)), state: super::bundle_state,
};
#[allow(non_upper_case_globals)]
impl crate::decl::Bundle {
    /// Synchronize the ClinGen local catalog.
    pub const ClinGen: Self = Self(&BUNDLE);
}
