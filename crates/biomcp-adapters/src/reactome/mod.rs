//! Lowest-level human pathway mappings from Reactome.
use crate::{Mapped, quotas};
use biodata::{EnsemblGeneId, Gene, GeneSymbol, UniProtAcc};
use biomcp_fetch::{Input, Read};

use biomcp_fetch::{
    Arg, BadReply, Freshness, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use biomcp_types::{PathwayRef, Source};
use serde::Deserialize;
use std::time::Duration;
mod discovery;
mod map;
mod raw;
mod relation_raw;
mod relations;
pub use discovery::{Page, card, release, search, top_level};
pub use relations::{Members, events, gene_product, members};
/// Public Reactome mapping endpoint policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "CC0 1.0 database data",
        url: "https://reactome.org/license",
        usage: biomcp_types::Use::Open,
    },
    source: Source::Reactome,
    base: "https://reactome.org/ContentService",
    key: KeyUse::None,
    quota: &quotas::REACTOME,
    deadline: Duration::from_secs(8),
    command_budget: None,
};
#[derive(Deserialize)]
struct Rows(Vec<Row>);
impl Reply for Rows {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Row {
    st_id: String,
    display_name: Option<crate::raw::Field<String>>,
    species_name: String,
    schema_class: String,
}
/// An identifier accepted by the Reactome pathway mapping endpoint.
#[derive(Clone, PartialEq, Eq)]
pub enum MappingId {
    /// UniProt protein accession.
    UniProt(UniProtAcc),
    /// Ensembl stable gene identifier.
    Ensembl(EnsemblGeneId),
    /// Approved gene symbol, accepted by Reactome's Ensembl mapping route.
    Symbol(GeneSymbol),
}

impl MappingId {
    /// Keep the UniProt relation when available. Without it, Reactome may index
    /// different entities by the Ensembl ID and the approved gene symbol.
    pub fn for_gene(gene: &Gene) -> Vec<Self> {
        if !gene.uniprot.is_empty() {
            return gene.uniprot.iter().cloned().map(Self::UniProt).collect();
        }
        gene.ensembl_id
            .iter()
            .cloned()
            .map(Self::Ensembl)
            .chain([Self::Symbol(gene.symbol.clone())])
            .collect()
    }

    /// Reactome resource and exact identifier sent to its mapping endpoint.
    pub fn parts(&self) -> (&'static str, &str) {
        match self {
            Self::UniProt(id) => ("UniProt", id.as_str()),
            Self::Ensembl(id) => ("Ensembl", id.base()),
            Self::Symbol(symbol) => ("Ensembl", symbol.as_str()),
        }
    }
}

/// Fetch one Reactome mapping; a checked 404 is an empty mapping.
pub fn pathways(leg: &Leg<'_>, id: &MappingId) -> Result<Mapped<Vec<PathwayRef>>, RequestError> {
    let (resource, id) = id.parts();
    let endpoint = SPEC
        .get("/data/mapping")
        .with_segments(&[
            PathSegment::new(resource)?,
            PathSegment::new(id)?,
            PathSegment::new("pathways")?,
        ])
        .fresh(Freshness::Days(7));
    let rows = match leg
        .call(
            &endpoint,
            Input::Params(&[("species", Arg::Text("9606"))]),
            Read::<Rows>::json(),
        )
        .map(|received| received.value)
    {
        Ok(rows) => rows,
        Err(RequestError::Source(error)) if error.reason == biomcp_types::Reason::NotFound => {
            return Ok(Mapped {
                refusals: crate::Refusal::None,
                value: Vec::new(),
                problems: Vec::new(),
            });
        }
        Err(error) => return Err(error),
    };
    let mut value = Vec::new();
    let mut refusals = crate::Refusal::None;
    let mut problems = map::fields(id);
    for row in rows.0 {
        let identity = map::identity(&row.st_id, &row.species_name, &row.schema_class);
        let Some(id) = problems.optional("pathways", Some(identity)) else {
            refusals = crate::Refusal::Refused;
            continue;
        };
        let name = map::metadata(
            &mut problems,
            "pathways.name",
            row.display_name.as_ref(),
            false,
        );
        value.push(PathwayRef { id, name });
    }
    Ok(Mapped {
        refusals,
        value,
        problems: problems.list,
    })
}

/// Complete source policy and command declaration.
pub(crate) mod decl;
