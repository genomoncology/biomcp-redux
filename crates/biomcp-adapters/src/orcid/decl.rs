//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "ORCID",
    registry_order: Some(41),
    probe: Probe::Get("/0000-0002-4299-3018/person", &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: Some(crate::decl::KeyDecl {
        name: biomcp_fetch::SecretName::OrcidAccessToken,
        suffix: "",
        word: "orcid",
        order: 2,
    }),
};
