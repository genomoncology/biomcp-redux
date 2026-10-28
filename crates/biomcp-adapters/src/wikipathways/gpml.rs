//! One publisher-documented per-pathway GPML request, with no catalog fallback.
use super::{
    SPEC,
    gpml_raw::{Document, MAX_BYTES},
};
use crate::Mapped;
use crate::mapped::problem;
use biodata::{PathwayMember, PathwaySourceRef, Text, WikiPathwaysId};
use biomcp_fetch::{Freshness, Leg, RequestError};
use biomcp_types::Source;
use std::collections::BTreeSet;

/// Complete GeneProduct nodes and observed source node counts, before paging.
#[derive(Debug)]
pub struct Members {
    /// Whole native GeneProduct data nodes, including repeated references.
    pub items: Vec<PathwayMember>,
    /// GPML's own full WP identifier and r revision, never the catalog's date.
    pub version: Text,
    /// GPML's source-stated pathway name.
    pub name: Text,
    /// All DataNodes, including excluded Metabolite and Pathway nodes.
    pub data_nodes: u64,
}
fn invalid() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::WikiPathways,
        reason: biomcp_types::Reason::BadReply,
        status: Some(200),
        retry_after: None,
    }
    .into()
}

/// Fetch exactly the publisher's documented GPML locator and validate identity before caching.
pub fn members(leg: &Leg<'_>, id: &WikiPathwaysId) -> Result<Mapped<Members>, RequestError> {
    let endpoint = SPEC
        .get("/wikipathways-assets/pathways")
        .with_path(format!("/wikipathways-assets/pathways/{id}/{id}.gpml"))?
        .once()
        .application_octet_stream()
        .bytes(MAX_BYTES)
        .fresh(Freshness::Days(7));
    let (document, _) =
        leg.xml_received_checked::<Document>(&endpoint, &[], |value| value.matches(id))?;
    let mut problems = Vec::new();
    let mut items = Vec::new();
    let mut ids = BTreeSet::new();
    let data_nodes = document.nodes.len() as u64;
    for node in document.nodes {
        if let Some(node_id) = &node.id
            && !ids.insert(node_id.clone())
        {
            return Err(invalid());
        }
        if node.kind.as_deref() != Some("GeneProduct") {
            if node.namespace.as_deref().is_none_or(str::is_empty) {
                problems.push(problem(
                    Source::WikiPathways,
                    id.as_str().parse().ok(),
                    &format!(
                        "data_nodes.{}.xref",
                        node.kind.as_deref().unwrap_or("unknown")
                    ),
                ));
            }
            continue;
        }
        if node.namespace.as_deref().is_none_or(str::is_empty)
            || node.value.as_deref().is_none_or(str::is_empty)
        {
            problems.push(problem(
                Source::WikiPathways,
                id.as_str().parse().ok(),
                "genes.reference",
            ));
        }
        items.push(PathwayMember {
            reference: PathwaySourceRef {
                namespace: node.namespace.map(Into::into),
                value: node
                    .value
                    .ok_or_else(invalid)?
                    .parse()
                    .map_err(|_| invalid())?,
            },
            node_id: node
                .id
                .map(|s| s.parse().map_err(|_| invalid()))
                .transpose()?,
            name: node
                .label
                .map(|s| s.parse().map_err(|_| invalid()))
                .transpose()?,
            source_url: None,
            gene: None,
            schema_class: Some("GeneProduct".parse().map_err(|_| invalid())?),
            physical_context_count: 1,
            participants: None,
        });
    }
    Ok(Mapped {
        value: Members {
            items,
            version: document.version.parse().map_err(|_| invalid())?,
            name: document.name.parse().map_err(|_| invalid())?,
            data_nodes,
        },
        problems,
    })
}
