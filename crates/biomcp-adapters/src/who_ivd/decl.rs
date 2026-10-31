//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "WHO prequalified diagnostic",
    registry_order: Some(4), probe: Probe::Get(crate::who_ivd::HEALTH_PATH, &[]),
    components: || &[],
    auxiliary: &[],
    bundles: &[&BUNDLE],
    guidance: Guidance { deadline: None, quota: None },
    key: None,
};
pub(crate) static BUNDLE: crate::decl::BundleDecl = crate::decl::BundleDecl {
    name: "who-ivd", source: biomcp_types::Source::WhoIvd, order: 9,
    sync: |fetch| super::sync(fetch).map(|mapped| crate::decl::BundleReply::Standard(mapped, false)), state: super::bundle_state,
};
#[allow(non_upper_case_globals)]
impl crate::decl::Bundle {
    /// Synchronize the WHO prequalified diagnostic local catalog.
    pub const WhoIvd: Self = Self(&BUNDLE);
}
