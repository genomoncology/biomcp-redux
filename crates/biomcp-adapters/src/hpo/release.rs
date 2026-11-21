//! Read the ontology, disease annotations and gene links from the same release tag.
use biodata::{Curie, HpId};
use biomcp_fetch::{
    BadReply, Bytes, Destinations, Freshness, Input, Leg, Read, RequestError, SourceSpec,
};
use biomcp_types::{HpoDiseaseMatch, HpoGeneLink, Reason, Source, Text};
use std::collections::{HashMap, HashSet};
use std::time::Duration;

use super::release_ontology as ontology;

const RELEASES: &str = "/obophenotype/human-phenotype-ontology/releases/download";
const GENE_HEADER: &str = "ncbi_gene_id\tgene_symbol\tassociation_type\tdisease_id\tsource";
const ANNOTATION_HEADER: &str = concat!(
    "database_id\tdisease_name\tqualifier\thpo_id\treference\tevidence",
    "\tonset\tfrequency\tsex\tmodifier\taspect\tbiocuration"
);

/// Transport policy for the HPO project's pinned GitHub release assets.
pub static SPEC: SourceSpec = SourceSpec {
    base: "https://github.com",
    destinations: Destinations::Additional(&["https://release-assets.githubusercontent.com"]),
    deadline: Duration::from_secs(60),
    command_budget: Some(Duration::from_secs(60)),
    ..super::SPEC
};

/// One slice of the complete intersection.
#[derive(Debug)]
pub struct Matches {
    /// Number of distinct matching diseases before paging.
    pub total: u64,
    /// Disease records with all positive evidence and available gene links.
    pub items: Box<[HpoDiseaseMatch]>,
    /// Terms without a current identity in this release.
    pub unknown: Vec<HpId>,
}

/// Match current HPO terms (including alternate IDs) against positive release annotations.
pub fn search(
    leg: &Leg<'_>,
    query: &biomcp_types::DiseaseSearch,
    tag: &str,
    requested: &[HpId],
) -> Result<Matches, RequestError> {
    let date = tag.strip_prefix('v').ok_or_else(bad_reply)?;
    let obo = read(leg, tag, "hp.obo", 24, &|body| {
        ontology::has_version(body, date)
    })?;
    let (descendants, unknown) = ontology::descendants(&obo, requested).map_err(|_| bad_reply())?;
    if !unknown.is_empty() {
        return Ok(Matches {
            total: 0,
            items: Box::default(),
            unknown,
        });
    }
    drop(obo);
    let hpoa = read(leg, tag, "phenotype.hpoa", 64, &|body| {
        has_annotations(body, date)
    })?;
    let candidates = annotate(&hpoa, &descendants, requested).map_err(|_| bad_reply())?;
    drop(hpoa);
    let total = candidates.len() as u64;
    let mut items: Vec<_> = candidates
        .into_iter()
        .skip(query.offset() as usize)
        .take(query.limit() as usize)
        .collect();
    if !items.is_empty() {
        let genes = read(leg, tag, "genes_to_disease.txt", 8, &|body| {
            body.lines().next() == Some(GENE_HEADER)
        })?;
        link_genes(&genes, &mut items).map_err(|_| bad_reply())?;
    }
    Ok(Matches {
        total,
        items: items.into(),
        unknown,
    })
}

fn read(
    leg: &Leg<'_>,
    tag: &str,
    file: &str,
    max_mib: u64,
    accept: &dyn Fn(&str) -> bool,
) -> Result<String, RequestError> {
    let url = format!("{}{RELEASES}/{tag}/{file}", SPEC.base);
    let endpoint = SPEC
        .download(&url)?
        .bytes(max_mib << 20)
        .fresh(Freshness::Days(30));
    let filter = |body: &[u8]| -> Result<Vec<u8>, BadReply> {
        let body = std::str::from_utf8(body).map_err(|_| BadReply)?;
        if !accept(body) {
            return Err(BadReply);
        }
        // The HPO annotation documentation requires separate permission for IEA rows.
        Ok(body
            .split_inclusive('\n')
            .filter(|line| line.starts_with('#') || line.split('\t').nth(5) != Some("IEA"))
            .flat_map(str::bytes)
            .collect())
    };
    let check = |bytes: &[u8]| {
        std::str::from_utf8(bytes)
            .ok()
            .filter(|body| accept(body))
            .map(|_| ())
            .ok_or(Reason::BadReply)
    };
    let mut codec = Read::<Bytes>::bytes().validate_body(&check);
    if file == "phenotype.hpoa" {
        codec = codec.filter_body(&filter);
    }
    let bytes = leg.call(&endpoint, Input::Params(&[]), codec)?.value;
    String::from_utf8(bytes.0).map_err(|_| bad_reply())
}

fn has_annotations(body: &str, date: &str) -> bool {
    let mut lines = body.lines().peekable();
    let mut matching_ontology = false;
    while let Some(line) = lines.next_if(|line| line.starts_with('#')) {
        if let Some(iri) = line.strip_prefix("#hpo-version:") {
            matching_ontology = iri
                .trim()
                .strip_prefix("http://purl.obolibrary.org/obo/hp/releases/")
                .or_else(|| {
                    iri.trim()
                        .strip_prefix("https://purl.obolibrary.org/obo/hp/releases/")
                })
                .and_then(|suffix| suffix.split_once('/'))
                .is_some_and(|(version, _)| version == date);
        }
    }
    matching_ontology && lines.next() == Some(ANNOTATION_HEADER)
}

fn positive(frequency: &str) -> bool {
    if frequency == "HP:0040285" {
        return false;
    }
    if let Some((numerator, _)) = frequency.split_once('/') {
        return numerator.trim().parse::<u64>() != Ok(0);
    }
    if let Some(percent) = frequency.strip_suffix('%') {
        return percent.trim().parse::<f64>() != Ok(0.0);
    }
    true
}

fn annotate(
    body: &str,
    descendants: &HashMap<HpId, Vec<usize>>,
    requested: &[HpId],
) -> Result<Vec<HpoDiseaseMatch>, BadReply> {
    let mut candidates = HashMap::<Curie, (Text, Vec<HashSet<HpId>>)>::new();
    for line in body.lines().filter(|line| !line.starts_with('#')).skip(1) {
        let mut fields = line.split('\t');
        let [Some(disease), Some(label), Some(qualifier), Some(term)] =
            std::array::from_fn(|_| fields.next())
        else {
            return Err(BadReply);
        };
        let frequency = fields.nth(3).ok_or(BadReply)?;
        if !qualifier.is_empty() || fields.nth(2) != Some("P") || !positive(frequency) {
            continue;
        }
        let term: HpId = term.parse().map_err(|_| BadReply)?;
        let Some(indices) = descendants.get(&term) else {
            continue;
        };
        let id = disease.parse().map_err(|_| BadReply)?;
        let label = Text::new(label).map_err(|_| BadReply)?;
        let (_, evidence) = candidates
            .entry(id)
            .or_insert_with(|| (label, vec![HashSet::new(); requested.len()]));
        for &index in indices {
            evidence[index].insert(term.clone());
        }
    }
    let mut matches: Vec<_> = candidates
        .into_iter()
        .filter_map(|(id, (label, evidence))| {
            if evidence.iter().any(HashSet::is_empty) {
                return None;
            }
            let matches = requested
                .iter()
                .cloned()
                .zip(evidence)
                .map(|(requested, annotated)| biomcp_types::HpoTermMatch {
                    requested,
                    annotated: {
                        let mut values: Vec<_> = annotated.into_iter().collect();
                        values.sort_unstable_by(|a, b| a.as_str().cmp(b.as_str()));
                        values.into()
                    },
                })
                .collect();
            Some(HpoDiseaseMatch {
                id,
                label,
                matches,
                gene_links: Box::default(),
            })
        })
        .collect();
    matches.sort_unstable_by(|a, b| a.id.as_str().cmp(b.id.as_str()));
    Ok(matches)
}

fn link_genes(body: &str, items: &mut [HpoDiseaseMatch]) -> Result<(), BadReply> {
    let mut links = HashMap::<_, Vec<_>>::new();
    for item in items.iter() {
        links.insert(item.id.clone(), Vec::new());
    }
    for line in body.lines().skip(1) {
        let mut fields = line.split('\t');
        let [
            Some(id),
            Some(symbol),
            Some(association_type),
            Some(disease),
            Some(source),
        ] = std::array::from_fn(|_| fields.next())
        else {
            return Err(BadReply);
        };
        let disease: Curie = disease.parse().map_err(|_| BadReply)?;
        if let Some(rows) = links.get_mut(&disease) {
            let link = HpoGeneLink {
                id: id.parse().map_err(|_| BadReply)?,
                symbol: Text::new(symbol).map_err(|_| BadReply)?,
                association_type: Text::new(association_type).map_err(|_| BadReply)?,
                source: Text::new(source).map_err(|_| BadReply)?,
            };
            if !rows.contains(&link) {
                rows.push(link);
            }
        }
    }
    for item in items {
        if let Some(rows) = links.remove(&item.id) {
            item.gene_links = rows.into();
        }
    }
    Ok(())
}

fn bad_reply() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Hpo,
        reason: Reason::BadReply,
        status: None,
        tls_cause: None,
        retry_after: None,
    }
    .into()
}
