//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC, display: "NCI Clinical Trials Search",
    registry_order: Some(42), probe: Probe::Get(
            "/trials",
            &[("size", Arg::Num(1)), ("include", Arg::Text("none"))],
        ),
    components: || &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance { deadline: None, quota: None },
    key: Some(crate::decl::KeyDecl { name: biomcp_fetch::SecretName::NciApiKey, word: "nci", order: 5 }),
};
