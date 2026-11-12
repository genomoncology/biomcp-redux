//! Other alleles join only when CIViC states the candidate's protein change on its transcript.
use super::{GENES, variant::failure, variant_raw as raw};
use crate::raw::Field;
use biomcp_fetch::{BadReply, Input, Leg, Read, Reply, RequestError};
use biomcp_types::{Reason, VariantHeader};
use serde::{Deserialize, Serialize};

const QUERY: &str = "query($name:String!){variants(name:$name,first:50){totalCount \
    pageInfo{hasNextPage endCursor} nodes{id name link feature{id name} \
    ... on GeneVariant{alleleRegistryId hgvsDescriptions}}}}";

#[derive(Serialize)]
struct Request<'a> {
    query: &'static str,
    variables: Variables<'a>,
}
#[derive(Serialize)]
struct Variables<'a> {
    name: &'a str,
}
#[derive(Deserialize)]
struct Response {
    data: Data,
    errors: Option<Vec<serde::de::IgnoredAny>>,
}
#[derive(Deserialize)]
struct Data {
    variants: raw::Connection<raw::Member>,
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .errors
            .as_ref()
            .is_some_and(|errors| !errors.is_empty())
        {
            return Err(BadReply);
        }
        self.data.variants.check()
    }
}
/// One protein change stated by the candidate's own transcript annotation.
#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct Change<'a> {
    gene: &'a str,
    transcript: &'a str,
    name: String,
    protein: String,
}

/// CIViC allele registry IDs, other than `allele`, whose variants state the same change.
pub(super) fn alleles(
    leg: &Leg<'_>,
    candidate: &VariantHeader,
    allele: &str,
) -> Result<Vec<String>, RequestError> {
    let mut changes: Vec<Change<'_>> = candidate
        .transcripts
        .iter()
        .filter_map(|row| {
            let forms = biomcp_types::protein_query_forms(row.hgvs_p.as_ref()?.as_str()).ok()?;
            let [name, _, _, protein] = forms;
            if name.ends_with('=') {
                return None;
            }
            Some(Change {
                gene: row.gene.as_ref()?.as_str(),
                transcript: accession(row.transcript.as_str()),
                name,
                protein,
            })
        })
        .collect();
    changes.sort();
    changes.dedup();
    let mut found = Vec::new();
    for change in &changes {
        let body = Request {
            query: QUERY,
            variables: Variables { name: &change.name },
        };
        let reply: Response = leg
            .call(&GENES, Input::json(&[], &body), Read::json())?
            .value;
        if reply.data.variants.page_info.has_next_page {
            return Err(failure(Reason::TooLarge));
        }
        for node in &reply.data.variants.nodes {
            if let Some(id) = states(node, change).filter(|id| *id != allele) {
                found.push(id.to_owned());
            }
        }
    }
    found.sort();
    found.dedup();
    Ok(found)
}
/// The CIViC allele when the variant's gene, transcript and protein notation all agree.
fn states<'a>(node: &'a raw::Member, change: &Change<'_>) -> Option<&'a str> {
    let gene = node.feature.as_ref()?.as_ref()?.name.as_ref()?.as_ref()?;
    let forms = node.hgvs_descriptions.as_ref()?.as_ref()?;
    let forms = forms
        .iter()
        .filter_map(Field::as_ref)
        .map(|form| form.as_str());
    let (mut transcript, mut protein) = (false, false);
    for form in forms {
        let Some((reference, body)) = form.split_once(':') else {
            continue;
        };
        transcript |= accession(reference) == change.transcript;
        protein |= body == change.protein;
    }
    if gene.as_str() != change.gene || !transcript || !protein {
        return None;
    }
    node.allele_registry_id
        .as_ref()?
        .as_ref()
        .map(|id| id.as_str())
}
fn accession(reference: &str) -> &str {
    reference
        .split_once('.')
        .map_or(reference, |(base, _)| base)
}
