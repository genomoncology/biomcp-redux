//! Source declarations and access used by the command interfaces.
pub use biomcp_adapters::guidance::{WONDER_QUOTA, deadline};
pub use biomcp_adapters::registry::records::{RecordKind, declaration, record_url};
pub use biomcp_adapters::registry::{
    SOURCES, components, disable_groups, disabled, disabled_uses, display_name, group, key_tier,
    outcome, requests, restore_outcome, terms,
};

use crate::Bundle;
pub use biomcp_adapters::{Mapped, Refusal};
/// Synchronize the two WikiPathways catalogs with their native receipt body.
pub fn sync_wikipathways(
    fetch: &biomcp_fetch::Fetch,
) -> Result<Mapped<biomcp_types::WikiPathwaysSync>, biomcp_fetch::RequestError> {
    biomcp_adapters::wikipathways::sync(fetch)
}
/// Synchronize one source bundle, retaining whether its last admitted catalog is stale.
pub fn sync(
    fetch: &biomcp_fetch::Fetch,
    bundle: Bundle,
) -> Option<Result<(Mapped<biomcp_types::SyncResult>, bool), biomcp_fetch::RequestError>> {
    Some(
        match bundle {
            Bundle::WikiPathways => return None,
            Bundle::CbioportalStudies => {
                return Some(biomcp_adapters::cbioportal::sync_studies(fetch));
            }
            Bundle::FdaCdx => biomcp_adapters::fda_cdx::sync(fetch),
            Bundle::Gtr => biomcp_adapters::gtr::sync(fetch),
            Bundle::WhoIvd => biomcp_adapters::who_ivd::sync(fetch),
            Bundle::WhoPq => biomcp_adapters::who_pq::sync(fetch),
            Bundle::Ddinter => biomcp_adapters::ddinter::sync(fetch),
            Bundle::Ema => biomcp_adapters::ema::sync(fetch),
            Bundle::ClinGen => biomcp_adapters::clingen::sync(fetch),
            Bundle::Hgnc => biomcp_adapters::hgnc::sync_all(fetch),
            Bundle::Gencc => biomcp_adapters::gencc::sync(fetch),
        }
        .map(|mapped| (mapped, false)),
    )
}

/// Configuration actions for source transport failures.
pub fn network(sources: &[biomcp_types::SourceOutcome]) -> Vec<crate::next::Step> {
    crate::next::network(sources)
}

/// Label publisher terms for an independently disclosed aggregator component.
pub use biomcp_adapters::myvariant::components::label as component_label;
