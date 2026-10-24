//! Implemented source declarations are the sole registry for terms and source switches.
use biodata::{Source, Use};
use biomcp_fetch::SourceSpec;
use std::collections::BTreeSet;
/// Every implemented source, independent of quota-provider identities.
pub const SOURCES: &[&SourceSpec] = &[
    &crate::clinvar::SPEC,
    &crate::erepo::SPEC,
    &crate::myvariant::SPEC,
    &crate::gwas::SPEC,
    &crate::allele_registry::SPEC,
    &crate::mutalyzer::SPEC,
    &crate::variantvalidator::SPEC,
    &crate::ctgov::SPEC,
    &crate::hgnc::SPEC,
    &crate::mygene::SPEC,
    &crate::oncokb::SPEC,
    &crate::gnomad::SPEC,
    &crate::uniprot::SPEC,
    &crate::reactome::SPEC,
    &crate::civic::SPEC,
    &crate::hotspots::SPEC,
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
    &crate::nci::SPEC,
];
/// Preserve terms groups for component policy after source-name expansion.
pub fn disabled_uses(names: &[String]) -> BTreeSet<Use> {
    names.iter().filter_map(|name| group(name)).collect()
}
fn group(name: &str) -> Option<Use> {
    match name {
        "noncommercial" => Some(Use::Noncommercial),
        "restricted" => Some(Use::Restricted),
        _ => None,
    }
}
/// Expand checked source names and the two user-selectable terms groups.
pub fn disabled(names: &[String]) -> Result<BTreeSet<Source>, biodata::Error> {
    let mut disabled = BTreeSet::new();
    for name in names {
        if let Some(group) = group(name) {
            disabled.extend(
                SOURCES
                    .iter()
                    .filter(|s| {
                        s.terms.usage == group
                            && !crate::myvariant::components::COMPONENTS
                                .iter()
                                .any(|component| component.source == s.source)
                    })
                    .map(|s| s.source),
            );
        } else {
            let source = SOURCES
                .iter()
                .find(|s| s.source.to_string() == *name)
                .ok_or(biodata::Error::SourceUnknown)?;
            disabled.insert(source.source);
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
    if matches!(spec.source, Source::Gnomad | Source::Civic) {
        let path = if spec.source == Source::Gnomad {
            "/api"
        } else {
            "/api/graphql"
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
            reason: biodata::Reason::Rejected,
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
    Some(match source {
        Source::Gwas => (
            "/associations",
            vec![("rs_id", Arg::Text("rs113488022")), ("size", Arg::Num(1))],
        ),
        Source::MyVariant => ("/metadata", vec![]),
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
        Source::UniProt => (
            "/uniprotkb/P15056.json",
            vec![("fields", Arg::Text("accession"))],
        ),
        Source::Reactome => ("/data/database/version", vec![]),
        Source::Hpa => ("/ENSG00000157764.json", vec![]),
        Source::CancerHotspots => ("/api/hotspots/single/byGene/BRAF", vec![]),
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
        _ => return None,
    })
}

#[derive(serde::Serialize)]
struct ProbeQuery {
    query: &'static str,
}
