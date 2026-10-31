//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "WikiPathways",
    registry_order: Some(25), probe: Probe::Get("/help.html", &[]),
    components: || &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    name: "wikipathways", source: biomcp_types::Source::WikiPathways, order: 3,
    sync: |fetch| super::sync(fetch).map(crate::decl::BundleReply::WikiPathways), state: super::bundle_state,
};
#[allow(non_upper_case_globals)]
impl crate::decl::Bundle {
    /// Synchronize the WikiPathways local catalog.
    pub const WikiPathways: Self = Self(&BUNDLE);
}
