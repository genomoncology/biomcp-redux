//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "GenCC",
    registry_order: Some(31), probe: Probe::Get(crate::gencc::HEALTH_PATH, &[]),
    components: || &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    name: "gencc", source: biomcp_types::Source::Gencc, order: 2,
    sync: |fetch| super::sync(fetch).map(|mapped| crate::decl::BundleReply::Standard(mapped, false)), state: super::bundle_state,
};
#[allow(non_upper_case_globals)]
impl crate::decl::Bundle {
    /// Synchronize the GenCC local catalog.
    pub const Gencc: Self = Self(&BUNDLE);
}
