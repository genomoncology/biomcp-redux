//! Bounded health requests use the registered publisher policy.
use biomcp_fetch::SourceSpec;
use biomcp_types::Source;
/// One bounded source probe through the checked transport.
pub fn probe(
    spec: &'static SourceSpec,
    leg: &biomcp_fetch::Leg<'_>,
) -> Result<(), biomcp_fetch::RequestError> {
    request_probe(spec, leg)
}
/// One bounded read-only endpoint, never a data synchronization.
fn request_probe(
    spec: &'static SourceSpec,
    leg: &biomcp_fetch::Leg<'_>,
) -> Result<(), biomcp_fetch::RequestError> {
    if spec.source == Source::Hgnc {
        return crate::hgnc::probe(leg);
    }
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
    if let Some(probe) = article_probe(source) {
        return Some(probe);
    }
    if let Some(probe) = pgx_probe(source) {
        return Some(probe);
    }
    if let Some(probe) = protein_probe(source) {
        return Some(probe);
    }
    Some(match source {
        Source::FdaDrugRecall => ("/drug/enforcement.json", vec![("limit", Arg::Num(1))]),
        Source::FdaDeviceRecall => ("/device/recall.json", vec![("limit", Arg::Num(1))]),
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
        Source::MyGene => ("/metadata", vec![]),
        Source::Reactome => ("/data/database/version", vec![]),
        Source::WikiPathways => ("/help.html", vec![]),
        Source::Hpa => ("/ENSG00000157764.json", vec![]),
        Source::ClinGen => (crate::clingen::HEALTH_PATH, vec![]),
        Source::Gencc => (crate::gencc::HEALTH_PATH, vec![]),
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

fn article_probe(source: Source) -> Option<(&'static str, ProbeParams)> {
    use biomcp_fetch::Arg;
    Some(match source {
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
        _ => return None,
    })
}
