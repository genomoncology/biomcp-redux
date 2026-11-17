//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::read::SPEC,
    display: "STRING",
    discovery_aliases: &[],
    registry_order: Some(61),
    probe: Probe::Get(
        super::read::PARTNERS,
        &[
            ("identifiers", Arg::Text("Q14191")),
            ("species", Arg::Num(9606)),
            ("limit", Arg::Num(1)),
            ("caller_identity", Arg::Text(super::read::CALLER)),
        ],
    ),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};
