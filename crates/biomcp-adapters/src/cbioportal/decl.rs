//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "cBioPortal",
    registry_order: Some(43), probe: Probe::Get("/studies", &[("pageSize", Arg::Num(1))]),
    components: || &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    name: "cbioportal-studies", source: biomcp_types::Source::Cbioportal, order: 4,
    sync: |fetch| super::sync_studies(fetch).map(|(mapped, stale)| crate::decl::BundleReply::Standard(mapped, stale)), state: super::bundle_state,
};
#[allow(non_upper_case_globals)]
impl crate::decl::Bundle {
    /// Synchronize the cBioPortal local catalog.
    pub const CbioportalStudies: Self = Self(&BUNDLE);
}
