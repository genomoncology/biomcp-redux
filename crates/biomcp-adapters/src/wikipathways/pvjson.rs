//! Gene cross-references supplied by the publisher for the same GPML revision.
use super::{SPEC, gpml_raw::MAX_BYTES};
use biodata::{NcbiGeneId, PathwayMember, WikiPathwaysId};
use biomcp_fetch::{BadReply, Freshness, Input, Leg, Read, Reply, RequestError};
use serde::Deserialize;
use std::collections::HashMap;

/// The publisher's cross-references, indexed by GPML node ID.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GeneXrefs {
    pathway: Pathway,
    entities_by_id: HashMap<String, Node>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Pathway {
    id: String,
    data_source_version: String,
    organism: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Node {
    gpml_element_name: String,
    wp_type: Option<String>,
    xref_data_source: Option<String>,
    xref_identifier: Option<String>,
    #[serde(rename = "type", default)]
    types: Vec<String>,
}

impl Reply for GeneXrefs {
    fn check(&self) -> Result<(), BadReply> {
        if self.pathway.organism == "Homo sapiens" {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}

impl GeneXrefs {
    /// Join on node identity and the exact native reference. Ambiguous mappings stay unresolved.
    pub fn entrez(&self, row: &PathwayMember) -> Option<NcbiGeneId> {
        let node = self.entities_by_id.get(row.node_id.as_ref()?.as_str())?;
        if node.gpml_element_name != "DataNode"
            || !matches!(
                node.wp_type.as_deref(),
                Some("GeneProduct" | "Protein" | "Rna")
            )
            || node.xref_data_source.as_deref()? != row.reference.namespace.as_deref()?
            || node.xref_identifier.as_deref()? != row.reference.id.as_str()
        {
            return None;
        }
        let mut ids = node
            .types
            .iter()
            .filter_map(|kind| kind.strip_prefix("Entrez Gene:"));
        let first = ids.next()?;
        if ids.any(|other| other != first) {
            return None;
        }
        first.parse().ok()
    }
}

/// Read one bounded, revision-matched pathway export.
pub fn gene_xrefs(
    leg: &Leg<'_>,
    id: &WikiPathwaysId,
    revision: &str,
) -> Result<GeneXrefs, RequestError> {
    let endpoint = SPEC
        .get("/wikipathways-assets/pathways")
        .with_path(format!("/wikipathways-assets/pathways/{id}/{id}.json"))?
        .once()
        .bytes(MAX_BYTES)
        .fresh(Freshness::Days(7));
    let id = id.to_string();
    let check = |value: &GeneXrefs| {
        let pathway = &value.pathway;
        if pathway.id != id || pathway.data_source_version != revision {
            return Err(BadReply);
        }
        Ok(())
    };
    Ok(leg
        .call(&endpoint, Input::Params(&[]), Read::json().check(&check))?
        .value)
}
