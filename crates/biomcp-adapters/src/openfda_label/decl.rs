//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "openFDA drug label",
    registry_order: Some(47),
    probe: Probe::Get("", &[("limit", Arg::Num(1))]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: Some(crate::decl::KeyDecl {
        name: biomcp_fetch::SecretName::OpenfdaApiKey,
        suffix: "",
        word: "openfda",
        order: 6,
    }),
};
