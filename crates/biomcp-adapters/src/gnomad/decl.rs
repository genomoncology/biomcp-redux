//! Source facts shared by discovery, health and commands.
use super::release;
use crate::decl::{BundleDecl, BundleReply, Guidance, Probe, SourceDecl};
use biomcp_types::ConstraintRelease;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "gnomAD",
    discovery_aliases: &[],
    registry_order: Some(20),
    probe: Probe::GraphQl("/api"),
    components: &[],
    auxiliary: &[&super::release::SPEC],
    bundles: &[&V4_1, &V4_0],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

const HELP: &str = "Index a gnomAD constraint release for MANE Select reads.";
static V4_0: BundleDecl = BundleDecl {
    name: release::V4_0_NAME,
    help: HELP,
    source: biomcp_types::Source::Gnomad,
    order: 13,
    sync: |fetch| BundleReply::Standard(release::sync(fetch, ConstraintRelease::V4_0), false),
    state: release::state,
};
static V4_1: BundleDecl = BundleDecl {
    name: release::V4_1_NAME,
    help: HELP,
    source: biomcp_types::Source::Gnomad,
    order: 14,
    sync: |fetch| BundleReply::Standard(release::sync(fetch, ConstraintRelease::V4_1), false),
    state: release::state,
};
impl crate::decl::Bundle {
    /// Select the local table matching this gnomAD constraint release.
    pub fn gnomad_constraint(release: ConstraintRelease) -> Self {
        Self(match release {
            ConstraintRelease::V4_0 => &V4_0,
            ConstraintRelease::V4_1 => &V4_1,
        })
    }
}
