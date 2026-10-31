//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "DDInter",
    registry_order: Some(51),
    probe: Probe::Get("/terms/", &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    help: crate::decl::bundle::REPLACE_HELP,
    name: "ddinter",
    source: biomcp_types::Source::Ddinter,
    order: 6,
    sync: |fetch| crate::decl::BundleReply::Standard(super::sync(fetch), false),
    state: super::bundle_state,
};
#[expect(
    non_upper_case_globals,
    reason = "Retain the established typed bundle selector names"
)]
impl crate::decl::Bundle {
    /// Synchronize the DDInter local catalog.
    pub const Ddinter: Self = Self(&BUNDLE);
}
