//! FDA website identity and pacing shared by the quarterly index and reports.
use crate::decl::{Guidance, Probe, SourceDecl};
use biomcp_fetch::{KeyUse, SourceSpec};
use biomcp_types::Source;
use std::time::Duration;

pub(super) const INDEX: &str = concat!(
    "/drugs/fda-adverse-event-monitoring-system-aems/",
    "new-safety-information-or-potential-signals-serious-risks-identified-fda-adverse-event-monitoring"
);

/// FDA website policy: “They are in the public domain and may be republished, reprinted and otherwise used freely”.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: crate::fda_dsc::SPEC.user_agent,
    source: Source::FdaSafetySignals,
    base: crate::fda_dsc::SITE,
    terms: crate::fda_dsc::SPEC.terms,
    quota: &crate::quotas::FDA_DSC,
    key: KeyUse::None,
    deadline: Duration::from_secs(8),
    command_budget: None,
};

pub(crate) static DECL: SourceDecl = SourceDecl {
    spec: &SPEC,
    display: "FDA Potential Safety Signals",
    discovery_aliases: &["FDA quarterly safety signals"],
    registry_order: Some(67),
    probe: Probe::Get(INDEX, &[]),
    components: &[],
    auxiliary: &[],
    bundles: &[],
    guidance: Guidance {
        deadline: None,
        quota: Some(
            "FDA web lists share a 30-second request interval as requested in FDA's robots.txt.",
        ),
    },
    key: None,
};
