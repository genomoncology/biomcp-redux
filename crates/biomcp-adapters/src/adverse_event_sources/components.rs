//! Reviewed field ownership; no dictionaries, inferred mappings or fetch-and-discard.
use crate::registry::Component;
use biomcp_fetch::{Leg, RequestError, SourceError, Terms};
use biomcp_types::{Reason, Source, Use};
/// Exact native field paths owned by the MedDRA component.
pub const MEDDRA_FIELDS: &[&str] = &[
    "patient.reaction.reactionmeddrapt",
    "patient.reaction.reactionmeddraversionpt",
];
/// Exact native field paths conservatively owned by GMDN; no other fields are assigned.
/// This assignment does not establish that every native string is a GMDN term.
pub const GMDN_FIELDS: &[&str] = &["device.generic_name"];
/// One declaration per component, shared by health and source outcomes.
pub const COMPONENTS: &[Component] = &[
    Component {
        source: Source::Faers,
        native_source: Some(Source::Meddra),
        name: "meddra",
        terms: Terms {
            name: "MedDRA statement on data sharing; third-party terminology",
            url: "https://admin.meddra.org/sites/default/files/page/documents/000198_statement_on_meddra_data_sharing_1_1.pdf",
            usage: Use::Restricted,
        },
    },
    Component {
        source: Source::Maude,
        native_source: Some(Source::Gmdn),
        name: "gmdn",
        terms: Terms {
            name: "openFDA license: GMDN third-party terminology exceptions",
            url: "https://open.fda.gov/license/",
            usage: Use::Restricted,
        },
    },
];
/// Admit a FAERS reaction section or reaction-filtered search before endpoint/cache/replay access.
/// Ordinary search, card and drugs reads do not require this terminology component.
pub fn meddra(leg: &Leg<'_>) -> Result<(), RequestError> {
    admit(leg, Source::Meddra, Source::Faers)
}
/// Admit a MAUDE positional search or devices section before endpoint/cache/replay access.
/// Card and narrative reads do not require this terminology component.
pub fn gmdn(leg: &Leg<'_>) -> Result<(), RequestError> {
    admit(leg, Source::Gmdn, Source::Maude)
}
fn admit(leg: &Leg<'_>, component: Source, owner: Source) -> Result<(), RequestError> {
    if COMPONENTS
        .iter()
        .find(|c| c.native_source == Some(component))
        .is_none_or(|c| c.disabled_on_leg(leg))
    {
        Err(SourceError {
            source: owner,
            reason: Reason::Disabled,
            status: None,
            retry_after: None,
        }
        .into())
    } else {
        Ok(())
    }
}
