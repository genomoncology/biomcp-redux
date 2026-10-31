//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "AlphaGenome",
    registry_order: Some(19), probe: Probe::Unavailable,
    components: || &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: Some(crate::decl::KeyDecl { name: biomcp_fetch::SecretName::AlphaGenomeApiKey, word: "alphagenome", order: 4 }),
};
