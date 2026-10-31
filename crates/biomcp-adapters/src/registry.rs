//! Implemented source declarations are the sole registry for terms and source switches.

mod component;
mod groups;
pub use groups::disable_groups;
mod names;
mod probe;
pub use names::display_name;
pub use probe::probe;
/// Canonical publisher record links and explicit uncitable ID kinds.
pub mod records;
pub use component::Component;

use biomcp_fetch::SourceSpec;
use biomcp_types::{Source, Use};
use std::collections::BTreeSet;
/// Every implemented source, independent of quota-provider identities.
pub const SOURCES: &[&SourceSpec] = &[
    &crate::cellosaurus::SPEC,
    &crate::cpic::SPEC,
    &crate::clinpgx::SPEC,
    &crate::gtr::SPEC,
    &crate::who_ivd::SPEC,
    &crate::who_pq::SPEC,
    &crate::fda_cdx::SPEC,
    &crate::fda_pma::SPEC,
    &crate::fda_510k::SPEC,
    &crate::fda_device_recall::SPEC,
    &crate::gwas::SPEC,
    &crate::myvariant::SPEC,
    &crate::allele_registry::SPEC,
    &crate::mutalyzer::SPEC,
    &crate::variantvalidator::SPEC,
    &crate::ctgov::SPEC,
    &crate::hgnc::SPEC,
    &crate::mygene::SPEC,
    &crate::oncokb::SPEC,
    &crate::alphagenome::SPEC,
    &crate::gnomad::SPEC,
    &crate::uniprot::SPEC,
    &crate::interpro::SPEC,
    &crate::complex_portal::SPEC,
    &crate::reactome::SPEC,
    &crate::wikipathways::SPEC,
    &crate::civic::SPEC,
    &crate::hotspots::SPEC,
    &crate::clinvar::SPEC,
    &crate::erepo::SPEC,
    &crate::clingen::SPEC,
    &crate::gencc::SPEC,
    &crate::hpa::SPEC,
    &crate::pubmed::SPEC,
    &crate::europepmc::SPEC,
    &crate::litvar::SPEC,
    &crate::litsense::SPEC,
    &crate::pubtator::SPEC,
    &crate::pmc_oa::SPEC,
    &crate::icite::SPEC,
    &crate::semantic_scholar::SPEC,
    &crate::orcid::SPEC,
    &crate::nci::SPEC,
    &crate::cbioportal::SPEC,
    &crate::mychem::SPEC,
    &crate::chembl::SPEC,
    &crate::opentargets::SPEC,
    &crate::openfda_label::SPEC,
    &crate::drugsfda::SPEC,
    &crate::fda_drug_recall::SPEC,
    &crate::ema::SPEC,
    &crate::ddinter::SPEC,
    &crate::ols::SPEC,
    &crate::mydisease::SPEC,
    &crate::monarch::SPEC,
    &crate::hpo::SPEC,
    &crate::seer::SPEC,
    &crate::faers::SPEC,
    &crate::maude::SPEC,
    &crate::vaers::SPEC,
];
/// Additional request declarations share a source identity with its primary API.
pub const AUXILIARY: &[&SourceSpec] = &[
    &crate::semantic_scholar::recommend::RECOMMEND_SPEC,
    &crate::semantic_scholar::pdf::PUBLISHER,
];
/// Enumerate every request declaration, including auxiliary APIs and publisher policy.
pub fn requests() -> impl Iterator<Item = &'static SourceSpec> {
    SOURCES.iter().chain(AUXILIARY).copied()
}
/// Declared credential policy shared by hosted admission and key discovery.
/// Multiple APIs using one credential must retain the strongest declared tier.
pub fn key_tier(name: biomcp_fetch::SecretName) -> biomcp_types::KeyTier {
    if requests().any(|spec| {
        matches!(spec.key,
        biomcp_fetch::KeyUse::Optional(c) | biomcp_fetch::KeyUse::Required(c)
            if c.secret == name && c.tier == biomcp_types::KeyTier::NoFallback)
    }) {
        biomcp_types::KeyTier::NoFallback
    } else {
        biomcp_types::KeyTier::Fallback
    }
}
/// Independently labeled fields from every implemented aggregator.
pub fn components() -> impl Iterator<Item = &'static Component> {
    crate::myvariant::components::all()
        .iter()
        .chain(crate::drug_sources::components::COMPONENTS)
        .chain(crate::disease_sources::components::COMPONENTS)
        .chain(crate::adverse_event_sources::components::COMPONENTS)
}
/// Resolve one supported terms group.
pub fn group(name: &str) -> Option<Use> {
    groups::USES
        .iter()
        .copied()
        .find(|usage| usage.as_str() == name)
}
/// Preserve group restrictions for independently labeled aggregator components.
pub fn disabled_uses(names: &[String]) -> BTreeSet<Use> {
    names.iter().filter_map(|name| group(name)).collect()
}
/// Expand source switches while component declarations keep aggregators available.
pub fn disabled(names: &[String]) -> Result<BTreeSet<Source>, biodata::Error> {
    let mut disabled = BTreeSet::new();
    for name in names {
        if let Some(usage) = group(name) {
            disabled.extend(groups::sources(usage));
        } else {
            let source = SOURCES
                .iter()
                .find(|s| s.source.to_string() == *name)
                .map(|s| s.source)
                .or_else(|| {
                    components()
                        .filter_map(|c| c.native_source)
                        .find(|s| s.to_string() == *name)
                })
                .ok_or(biodata::Error::SourceUnknown)?;
            disabled.insert(source);
        }
    }
    Ok(disabled)
}

/// Publisher terms for a direct service or its independently switchable component.
pub fn terms(source: Source) -> Option<biomcp_fetch::Terms> {
    SOURCES
        .iter()
        .find(|spec| spec.source == source)
        .map(|spec| spec.terms)
        .or_else(|| {
            components()
                .find(|component| component.native_source == Some(source))
                .map(|component| component.terms)
        })
}

/// Construct every outcome with the same publisher declaration used by health.
#[expect(
    clippy::expect_used,
    reason = "Every Source has one declaration; registry coverage is checked outside-in"
)]
pub fn outcome(
    source: Source,
    state: biomcp_types::SourceState,
    reason: Option<biomcp_types::Reason>,
) -> biomcp_types::SourceOutcome {
    let terms = terms(source).expect("source has a registered publisher declaration");
    biomcp_types::SourceOutcome {
        source,
        usage: terms.usage,
        terms: terms.name.into(),
        terms_url: terms.url.into(),
        state,
        reason,
        term: None,
        count: None,
        retrieved_count: None,
        notices: Vec::new(),
    }
}

/// Reapply current publisher labels when an outcome is restored from a snapshot.
pub fn restore_outcome(mut saved: biomcp_types::SourceOutcome) -> biomcp_types::SourceOutcome {
    let current = outcome(saved.source, saved.state, saved.reason);
    saved.usage = current.usage;
    saved.terms = current.terms;
    saved.terms_url = current.terms_url;
    saved
}

/// Source-owned local set metadata, separate from the online publisher probe.
pub fn bundle_state(
    spec: &'static SourceSpec,
    leg: &biomcp_fetch::Leg<'_>,
) -> Result<Option<biomcp_types::LocalBundleState>, biomcp_fetch::RequestError> {
    let state = match spec.source {
        Source::Ema => crate::ema::bundle_state(leg)?,
        Source::Ddinter => crate::ddinter::bundle_state(leg)?,
        Source::Cbioportal => crate::cbioportal::bundle_state(leg)?,
        Source::WikiPathways => crate::wikipathways::bundle_state(leg)?,
        Source::Hgnc => crate::hgnc::bundle_state(leg)?,
        Source::ClinGen => crate::clingen::bundle_state(leg)?,
        Source::Gencc => crate::gencc::bundle_state(leg)?,
        Source::Gtr => crate::gtr::bundle_state(leg)?,
        Source::WhoIvd => crate::who_ivd::bundle_state(leg)?,
        Source::WhoPq => crate::who_pq::bundle_state(leg)?,
        Source::FdaCdx => crate::fda_cdx::bundle_state(leg)?,
        _ => return Ok(None),
    };
    Ok(Some(state))
}
