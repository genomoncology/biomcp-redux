//! Checked CIViC gene-level curation joined to HGNC Entrez identities.
mod claim_content;
mod claim_vocab;
mod drug;
mod drug_get;
mod drug_map;
mod drug_raw;
mod raw;
mod variant;
mod variant_claims;
mod variant_map;
mod variant_raw;
mod variant_subject;
use crate::{Mapped, quotas, raw::Field};
use biodata::{GeneSymbol, NcbiGeneId, Text};

use biomcp_fetch::{Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::{CivicGene, Section, Source, SourceRecord};
pub use drug::drug_search;
pub use drug_get::drug_civic;
use serde::Serialize;
use std::{collections::BTreeMap, time::Duration};
pub use variant::variant;
/// Public CIViC GraphQL source policy.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "CC0 1.0",
        url: "https://civicdb.org/releases/licensing",
        usage: biomcp_types::Use::Open,
    },
    source: Source::Civic,
    base: "https://civicdb.org",
    key: KeyUse::None,
    quota: &quotas::CIVIC,
    deadline: Duration::from_secs(8),
};
const GENES: Endpoint = SPEC
    .post_json("/api/graphql")
    .idempotent()
    .fresh(Freshness::Days(7));
#[derive(Serialize)]
struct Query {
    query: String,
    variables: BTreeMap<String, String>,
}
/// A requested identity and its independently mapped source section.
#[derive(Debug)]
pub struct GeneCivic {
    /// Submitted symbol that selected the GraphQL alias.
    pub symbol: GeneSymbol,
    /// Submitted Entrez join identity.
    pub id: NcbiGeneId,
    /// Complete section or its explicit absence/failure.
    pub civic: Mapped<Section<CivicGene>>,
}
/// Fetch up to twenty distinct symbol/Entrez pairs in one encoded aliased query.
pub fn genes(
    leg: &Leg<'_>,
    ids: &[(GeneSymbol, NcbiGeneId)],
) -> Result<Vec<GeneCivic>, RequestError> {
    let mut ids = ids.to_vec();
    ids.sort_unstable_by(|a, b| a.0.as_str().cmp(b.0.as_str()));
    ids.dedup();
    let mut variables = BTreeMap::new();
    let mut declarations = Vec::new();
    let mut aliases = Vec::new();
    for (index, (symbol, _)) in ids.iter().enumerate() {
        variables.insert(format!("id{index}"), symbol.to_string());
        declarations.push(format!("$id{index}: String!"));
        aliases.push(format!(
            "g{index}: gene(entrezSymbol: $id{index}) {{ id name entrezId description link variants {{ totalCount }} }}"
        ));
    }
    let body = Query {
        query: format!(
            "query({}) {{ {} }}",
            declarations.join(", "),
            aliases.join(" ")
        ),
        variables,
    };
    macro_rules! fetch {
        ($($n:literal),+) => { match ids.len() { $( $n => leg.post_json::<_, raw::Genes<$n>>(&GENES, &body).map(|raw| raw.data), )+ _ => return Err(bad()) } };
    }
    let raw = fetch!(
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20
    )?;
    Ok(ids
        .into_iter()
        .enumerate()
        .map(|(index, (symbol, id))| mapped(&raw, index, symbol, id))
        .collect())
}
fn mapped(raw: &raw::Aliases, index: usize, symbol: GeneSymbol, id: NcbiGeneId) -> GeneCivic {
    let gene = raw
        .0
        .iter()
        .find(|(name, _)| *name == format!("g{index}"))
        .and_then(|(_, gene)| gene.as_ref());
    let mut cx = crate::mapped::Problems {
        source: Source::Civic,
        record: None,
        list: Vec::new(),
    };
    let value = gene
        .and_then(|gene| map(gene, id, &mut cx))
        .unwrap_or(Section::Empty {
            source: Source::Civic,
            record: None,
        });
    GeneCivic {
        symbol,
        id,
        civic: Mapped {
            value,
            problems: cx.list,
        },
    }
}
fn map(
    gene: &raw::Gene,
    requested: NcbiGeneId,
    cx: &mut crate::mapped::Problems,
) -> Option<Section<CivicGene>> {
    let id = gene
        .id
        .as_ref()
        .and_then(Field::as_ref)
        .copied()
        .filter(|id| *id > 0 && *id <= i32::MAX as u64);
    cx.record = id.and_then(crate::mapped::record);
    let id = cx.required("id", id)?;
    let entrez = cx.optional(
        "entrezId",
        gene.entrez_id.as_ref().map(|value| {
            value
                .as_ref()
                .copied()
                .ok_or(())
                .and_then(|id| NcbiGeneId::new(id).map_err(|_| ()))
        }),
    );
    let entrez = cx.required("entrezId", entrez)?;
    if entrez != requested {
        cx.required::<()>("entrezId", None);
        return None;
    }
    let url = cx.optional(
        "link",
        gene.link.as_ref().map(|field| {
            field.as_ref().ok_or(()).and_then(|link| {
                if !link.starts_with("/features/") {
                    return Err(());
                }
                Text::new(&format!("https://civicdb.org{link}")).map_err(|_| ())
            })
        }),
    );
    let description = cx.optional(
        "description",
        gene.description.as_ref().map(|field| {
            field
                .as_ref()
                .ok_or(())
                .and_then(|text| Text::from_source(text).map_err(|_| ()))
        }),
    );
    let variants = cx.optional(
        "variants",
        gene.variants.as_ref().map(|field| field.as_ref().ok_or(())),
    );
    let variant_count = variants.and_then(|variants| {
        cx.optional(
            "variants.totalCount",
            variants
                .total_count
                .as_ref()
                .map(|field| field.as_ref().copied().ok_or(())),
        )
    });
    Some(if variant_count == Some(0) && description.is_none() {
        Section::Empty {
            source: Source::Civic,
            record: Some(SourceRecord {
                namespace: biodata::IdentifierNamespace::Unknown,
                id: cx.record.clone()?,
                url,
            }),
        }
    } else {
        Section::Data {
            source: Source::Civic,
            data: CivicGene {
                id: biodata::Identifier::new(crate::mapped::text("CIViC").ok()?, id),
                variant_count,
                url,
                description,
            },
        }
    })
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Civic,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
