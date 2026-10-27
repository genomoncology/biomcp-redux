//! Implemented source declarations are the sole registry for terms and source switches.

mod component;
mod groups;
pub use groups::disable_groups;
mod names;
pub use names::display_name;
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

/// One bounded read-only endpoint for each implemented source; never a data synchronization.
/// Paths stay relative to the source declaration, and replies bypass the data cache.
pub fn probe(
    spec: &'static SourceSpec,
    leg: &biomcp_fetch::Leg<'_>,
) -> Result<(), biomcp_fetch::RequestError> {
    if spec.source == Source::Faers {
        crate::adverse_event_sources::components::meddra(leg)?;
    }
    if spec.source == Source::Maude {
        crate::adverse_event_sources::components::gmdn(leg)?;
    }
    if spec.source == Source::Monarch {
        crate::disease_sources::components::monarch(leg)?;
    }
    if spec.source == Source::LitVar {
        return leg.probe(
            &crate::litvar::AUTOCOMPLETE,
            &[("query", biomcp_fetch::Arg::Text("rs121434592"))],
        );
    }
    if spec.source == Source::LitSense {
        return leg.probe(
            &crate::litsense::SENTENCES,
            &[
                (
                    "query",
                    biomcp_fetch::Arg::Text("AKT1 E17K capivasertib breast cancer"),
                ),
                ("rerank", biomcp_fetch::Arg::Text("true")),
            ],
        );
    }
    if spec.source == Source::Oncokb {
        return leg.probe(&crate::oncokb::INFO, &[]);
    }
    if matches!(
        spec.source,
        Source::Gnomad | Source::Civic | Source::Opentargets
    ) {
        let path = if spec.source == Source::Gnomad {
            "/api"
        } else if spec.source == Source::Civic {
            "/api/graphql"
        } else {
            "/graphql"
        };
        return leg.probe_json(
            &spec.post_json(path).idempotent(),
            &ProbeQuery {
                query: "{ __typename }",
            },
        );
    }
    let Some((path, params)) = get_probe(spec.source) else {
        return Err(biomcp_fetch::SourceError {
            source: spec.source,
            reason: biomcp_types::Reason::Rejected,
            status: None,
            retry_after: None,
        }
        .into());
    };
    leg.probe(&spec.get(path), &params)
}
type ProbeParams = Vec<(&'static str, biomcp_fetch::Arg<'static>)>;
fn get_probe(source: Source) -> Option<(&'static str, ProbeParams)> {
    use biomcp_fetch::Arg;
    if let Some(probe) = pgx_probe(source) {
        return Some(probe);
    }
    if let Some(probe) = protein_probe(source) {
        return Some(probe);
    }
    Some(match source {
        Source::Gtr => (crate::gtr::HEALTH_PATH, vec![]),
        Source::WhoIvd => (crate::who_ivd::HEALTH_PATH, vec![]),
        Source::WhoPq => (crate::who_pq::HEALTH_PATH, vec![]),
        Source::FdaCdx => (crate::fda_cdx::HEALTH_PATH, vec![]),
        Source::Vaers => (crate::vaers::HEALTH_PATH, vec![]),
        Source::Cellosaurus => ("/release-info", vec![("format", Arg::Text("json"))]),
        Source::Gwas => (
            "/associations",
            vec![("rs_id", Arg::Text("rs113488022")), ("size", Arg::Num(1))],
        ),
        Source::CancerHotspots => ("/api/hotspots/single/byGene/BRAF", vec![]),
        Source::Cbioportal => ("/studies", vec![("pageSize", Arg::Num(1))]),
        Source::MyVariant | Source::Mychem | Source::MyDisease => ("/metadata", vec![]),
        Source::Chembl => ("/status.json", vec![]),
        Source::OpenfdaLabel
        | Source::Drugsfda
        | Source::Faers
        | Source::Maude
        | Source::FdaPma
        | Source::Fda510k => ("", vec![("limit", Arg::Num(1))]),
        Source::Ema => ("/en/about-us/about-website/legal-notice", vec![]),
        Source::Ddinter => ("/terms/", vec![]),
        Source::AlleleRegistry => ("/allele/CA123643", vec![]),
        Source::Mutalyzer => ("/normalize/NM_004333.4:c.1799T%3EA", vec![]),
        Source::VariantValidator => (
            "/variantvalidator/GRCh38/LRG_321p1%3Ap.Arg273His/all",
            vec![("content-type", Arg::Text("application/json"))],
        ),
        Source::ClinicalTrialsGov => ("/version", vec![]),
        Source::Nci => (
            "/trials",
            vec![("size", Arg::Num(1)), ("include", Arg::Text("none"))],
        ),
        Source::Hgnc => ("/withdrawn.txt", vec![]),
        Source::MyGene => ("/metadata", vec![]),
        Source::Reactome => ("/data/database/version", vec![]),
        Source::WikiPathways => ("/help.html", vec![]),
        Source::Hpa => ("/ENSG00000157764.json", vec![]),
        Source::ClinGen => ("/kb/gene-validity/download", vec![]),
        Source::PubMed => (
            "/einfo.fcgi",
            vec![("db", Arg::Text("pubmed")), ("retmode", Arg::Text("json"))],
        ),
        Source::EuropePmc => (
            "/search",
            vec![
                ("query", Arg::Text("EXT_ID:17611497 AND SRC:MED")),
                ("format", Arg::Text("json")),
                ("pageSize", Arg::Num(1)),
            ],
        ),
        Source::PubTator => (
            "/publications/export/biocjson",
            vec![("pmids", Arg::Text("17611497"))],
        ),
        Source::PmcOa => (
            "/",
            vec![
                ("list-type", Arg::Num(2)),
                ("prefix", Arg::Text("PMC5606870.")),
                ("max-keys", Arg::Num(1)),
            ],
        ),
        Source::Orcid => ("/0000-0002-4299-3018/person", vec![]),
        Source::SemanticScholar => (
            "/paper/search",
            vec![
                ("query", Arg::Text("EGFR")),
                ("limit", Arg::Num(1)),
                ("fields", Arg::Text("externalIds,title,year")),
            ],
        ),
        Source::Icite => (
            "/pubs",
            vec![("pmids", Arg::Text("17611497")), ("fl", Arg::Text("pmid"))],
        ),
        _ => return disease_probe(source),
    })
}

fn disease_probe(source: Source) -> Option<(&'static str, ProbeParams)> {
    use biomcp_fetch::Arg;
    Some(match source {
        Source::Hpo => ("/api/hp/terms/HP:0001250", vec![]),
        Source::Ols => ("/api/ontologies", vec![("size", Arg::Num(1))]),
        Source::Monarch => (
            "/v3/api/association",
            vec![
                ("object", Arg::Text("MONDO:0018875")),
                ("subject_category", Arg::Text("biolink:Gene")),
                ("limit", Arg::Num(1)),
            ],
        ),
        Source::Seer => ("/get_var_formats.php", vec![]),
        _ => return assertion_probe(source),
    })
}

#[derive(serde::Serialize)]
struct ProbeQuery {
    query: &'static str,
}

fn assertion_probe(source: Source) -> Option<(&'static str, ProbeParams)> {
    use biomcp_fetch::Arg;
    Some(match source {
        Source::ClinVar => (
            "/einfo.fcgi",
            vec![("db", Arg::Text("clinvar")), ("retmode", Arg::Text("json"))],
        ),
        Source::Erepo => (
            "/evrepo/api/summary/classifications",
            vec![
                ("columns", Arg::Text("caId")),
                ("values", Arg::Text("CA000434")),
                ("matchTypes", Arg::Text("exact")),
                ("pgSize", Arg::Num(1)),
                ("pg", Arg::Num(1)),
            ],
        ),
        _ => return None,
    })
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
        fetched: None,
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

fn protein_probe(source: Source) -> Option<(&'static str, ProbeParams)> {
    use biomcp_fetch::Arg;
    Some(match source {
        Source::UniProt => (
            "/uniprotkb/P15056.json",
            vec![("fields", Arg::Text("accession"))],
        ),
        Source::InterPro => (
            "/entry/interpro/protein/uniprot/P00533/",
            vec![("page_size", Arg::Num(5))],
        ),
        Source::ComplexPortal => (
            "/search/P15056",
            vec![
                ("first", Arg::Num(0)),
                ("number", Arg::Num(5)),
                ("filters", Arg::Text("species_f:(\"Homo sapiens\")")),
            ],
        ),
        _ => return None,
    })
}

/// Source-owned local set metadata, separate from the online publisher probe.
pub fn bundle_state(
    spec: &'static SourceSpec,
    leg: &biomcp_fetch::Leg<'_>,
) -> Result<Option<biomcp_types::LocalBundleState>, biomcp_fetch::RequestError> {
    let state = match spec.source {
        Source::Gtr => crate::gtr::bundle_state(leg)?,
        Source::WhoIvd => crate::who_ivd::bundle_state(leg)?,
        Source::WhoPq => crate::who_pq::bundle_state(leg)?,
        Source::FdaCdx => crate::fda_cdx::bundle_state(leg)?,
        _ => return Ok(None),
    };
    Ok(Some(state))
}

fn pgx_probe(source: Source) -> Option<(&'static str, ProbeParams)> {
    use biomcp_fetch::Arg;
    Some(match source {
        Source::Cpic => (crate::cpic::GENE_PATH, vec![("limit", Arg::Num(1))]),
        Source::ClinPgx => (
            crate::clinpgx::CLINICAL_PATH,
            vec![
                ("location.genes.symbol", Arg::Text("DPYD")),
                ("view", Arg::Text("min")),
            ],
        ),
        _ => return None,
    })
}
