//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "DDInter",
    registry_order: Some(51), probe: Probe::Get("/terms/", &[]),
    components: || &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    name: "ddinter", source: biomcp_types::Source::Ddinter, order: 6,
    sync: |fetch| super::sync(fetch).map(|mapped| crate::decl::BundleReply::Standard(mapped, false)), state: super::bundle_state,
};
#[allow(non_upper_case_globals)]
impl crate::decl::Bundle {
    /// Synchronize the DDInter local catalog.
    pub const Ddinter: Self = Self(&BUNDLE);
}
