//! Source facts shared by discovery, health and commands.
use crate::decl::{Guidance, Probe, SourceDecl};
fn probe(leg: &biomcp_fetch::Leg<'_>) -> Result<(), biomcp_fetch::RequestError> {
    crate::disease_sources::components::monarch(leg)?;
    (Probe::Get(
        "/v3/api/association",
        &[
            ("object", Arg::Text("MONDO:0018875")),
            ("subject_category", Arg::Text("biolink:Gene")),
            ("limit", Arg::Num(1)),
        ],
    ))
    .run(&super::SPEC, leg)
}
use biomcp_fetch::Arg;
pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &super::SPEC,
    display: "Monarch",
    registry_order: Some(54),
    probe: Probe::Custom(probe),
    components: &[COMPONENT_8, COMPONENT_9],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: None,
    },
    key: None,
};

pub(crate) const COMPONENT_8: crate::registry::Component =
    crate::disease_sources::components::component(
        biomcp_types::Source::Monarch,
        biomcp_types::Source::Hpo,
        "hpo",
        crate::disease_sources::components::HPO,
    );

pub(crate) const COMPONENT_9: crate::registry::Component =
    crate::disease_sources::components::component(
        biomcp_types::Source::Monarch,
        biomcp_types::Source::Orphanet,
        "orphanet",
        crate::disease_sources::components::ORPHANET,
    );
