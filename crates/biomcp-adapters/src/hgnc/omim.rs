//! Exact OMIM gene-entry lookups in the HGNC catalog.
use biodata::{Curie, HgncId};
use biomcp_fetch::{
    BadReply, Freshness, Input, InputError, Leg, PathSegment, Read, Reply, RequestError, SourceSpec,
};
use serde::Deserialize;

/// HGNC REST API for exact OMIM gene-entry lookups.
pub static OMIM_SPEC: SourceSpec = SourceSpec {
    base: "https://rest.genenames.org",
    deadline: std::time::Duration::from_secs(4),
    ..super::SPEC
};
const FETCH: biomcp_fetch::Endpoint = OMIM_SPEC.get("/fetch/omim_id").fresh(Freshness::Days(7));

#[derive(Debug, Deserialize)]
struct Document {
    response: Hits,
}

#[derive(Debug, Deserialize)]
struct Hits {
    #[serde(rename = "numFound")]
    total: usize,
    docs: Vec<Gene>,
}

#[derive(Debug, Deserialize)]
struct Gene {
    hgnc_id: HgncId,
    status: String,
    omim_id: Vec<String>,
}

impl Reply for Document {
    fn check(&self) -> Result<(), BadReply> {
        if self.response.total == self.response.docs.len() {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

/// Find approved HGNC genes that state this exact OMIM gene identifier.
pub fn omim_genes(leg: &Leg<'_>, id: &Curie) -> Result<Vec<HgncId>, RequestError> {
    let number = id.as_str().strip_prefix("OMIM:").ok_or(InputError::Path)?;
    let endpoint = FETCH.clone().with_segments(&[PathSegment::new(number)?]);
    let result = leg
        .call(
            &endpoint,
            Input::Params(&[]),
            Read::<Document>::json().check(&|data| {
                if data.response.docs.iter().all(|gene| {
                    gene.status == "Approved" && gene.omim_id.iter().any(|value| value == number)
                }) {
                    Ok(())
                } else {
                    Err(BadReply)
                }
            }),
        )?
        .value;
    Ok(result
        .response
        .docs
        .into_iter()
        .map(|gene| gene.hgnc_id)
        .collect())
}
