//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "OncoKB",
    registry_order: Some(18),
    probe: Probe::Custom(|leg| leg.probe(&crate::oncokb::INFO, &[])),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: Some(crate::decl::KeyDecl {
        name: biomcp_fetch::SecretName::OncokbApiKey,
        suffix: "",
        word: "oncokb",
        order: 3,
    }),
};
