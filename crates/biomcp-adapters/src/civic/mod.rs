//! Checked CIViC gene-level curation joined to HGNC Entrez identities.
mod drug;
mod drug_get;
mod drug_map;
mod drug_raw;
mod raw;
mod variant;
mod variant_claims;
mod variant_map;
mod variant_raw;
use crate::{Mapped, quotas, raw::Field};
use biodata::{GeneSymbol, NcbiGeneId, Text};

use biomcp_fetch::{Endpoint, Freshness, KeyUse, Leg, RequestError, SourceSpec};
use biomcp_types::{CivicGene, Problem, ProblemCode, Section, Source, SourceRecord};
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
    let mut problems = Vec::new();
    let gene = raw
        .0
        .iter()
        .find(|(name, _)| *name == format!("g{index}"))
        .and_then(|(_, gene)| gene.as_ref());
    let section = match gene {
        None => Section::Empty {
            source: Source::Civic,
            record: None,
        },
        Some(gene) if gene.entrez_id != id => {
            problems.push(Problem {
                source: Source::Civic,
                record: Text::new(&id.to_string()).ok(),
                field: "entrez_id".into(),
                code: ProblemCode::ValueUnknown,
            });
            Section::Unavailable {
                source: Source::Civic,
                reason: biomcp_types::Reason::BadReply,
            }
        }
        Some(gene) => map(gene, &mut problems),
    };
    GeneCivic {
        symbol,
        id,
        civic: Mapped {
            value: section,
            problems,
        },
    }
}

fn map(gene: &raw::Gene, problems: &mut Vec<Problem>) -> Section<CivicGene> {
    let Ok(url) = Text::new(&format!("https://civicdb.org{}", gene.link)) else {
        return Section::Unavailable {
            source: Source::Civic,
            reason: biomcp_types::Reason::BadReply,
        };
    };
    let description = match &gene.description {
        Some(Field::Value(text)) => crate::mapped::field_text(
            problems, Source::Civic, crate::mapped::record(gene.id), "description",
            Some(Field::Value(text.clone())),
        ),
        Some(Field::Invalid(_)) => {
            problems.push(Problem {
                source: Source::Civic,
                record: Text::new(&gene.id.to_string()).ok(),
                field: "description".into(),
                code: ProblemCode::ValueUnknown,
            });
            None
        }
        None => None,
    };
    if gene.variants.total_count == 0 && description.is_none() {
        Section::Empty {
            source: Source::Civic,
            record: Some(SourceRecord {
                id: match Text::new(&gene.id.to_string()) {
                    Ok(id) => id,
                    Err(_) => {
                        return Section::Unavailable {
                            source: Source::Civic,
                            reason: biomcp_types::Reason::BadReply,
                        };
                    }
                },
                url: Some(url),
            }),
        }
    } else {
        Section::Data {
            source: Source::Civic,
            data: CivicGene {
                id: gene.id,
                variant_count: gene.variants.total_count,
                url,
                description,
            },
        }
    }
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
