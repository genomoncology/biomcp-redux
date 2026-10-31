//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "CDC WONDER VAERS",
    registry_order: Some(59),
    probe: Probe::Get(crate::vaers::HEALTH_PATH, &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: Some(
            "CDC WONDER can take longer than BioMCP's local command deadline. Use the interactive form at \
            https://wonder.cdc.gov/vaers.html for a longer query. See https://wonder.cdc.gov/wonder/help/faq.html.",
        ),
        quota: Some(crate::guidance::WONDER_QUOTA),
    },
    key: None,
};
