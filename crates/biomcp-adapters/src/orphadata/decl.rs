//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::bundle::SPEC,
    display: "Orphadata",
    discovery_aliases: &[],
    registry_order: Some(60),
    probe: Probe::Get(super::HEALTH_PATH, &[]),
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
    help: "Synchronize Orphanet prevalence. Use --xrefs to synchronize its OMIM mappings.",
    name: "orphadata",
    source: biomcp_types::Source::Orphadata,
    order: 11,
    sync: |fetch| crate::decl::BundleReply::Standard(super::sync(fetch), false),
    state: super::bundle_state,
};
impl crate::decl::Bundle {
    /// Synchronize the Orphadata epidemiology files.
    pub const ORPHADATA: Self = Self(&BUNDLE);
}
pub(crate) static XREFS: crate::decl::BundleDecl = crate::decl::BundleDecl {
    help: "Synchronize Orphanet’s published OMIM mappings.",
    name: super::mappings::NAME,
    source: biomcp_types::Source::Orphadata,
    order: 13,
    sync: |fetch| crate::decl::BundleReply::Standard(super::mappings::sync(fetch), false),
    state: super::mappings::state,
};
impl crate::decl::Bundle {
    /// Synchronize the Orphadata cross-reference file.
    pub const ORPHADATA_XREFS: Self = Self(&XREFS);
}
