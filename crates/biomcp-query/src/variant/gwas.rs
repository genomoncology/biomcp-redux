//! Source-local GWAS Catalog pages, with no inference from catalog absence.
use crate::{Ctx, status};
use biodata::VariantId;
use biomcp_adapters::gwas;
use biomcp_fetch::{Deadline, RequestError};
use biomcp_types::{
    GwasAssociation, GwasEffectAllele, PageInfo, Problem, ProblemCode, Reason, Section, Source,
    VariantGet, VariantGwas, VariantHeader,
};
/// Ask only the current catalog and preserve source selection and paging.
pub(super) fn section(
    cx: &Ctx<'_>,
    query: &VariantGet,
    candidates: &[VariantHeader],
    deadline: Deadline<'_>,
) -> super::sections::Result<VariantGwas> {
    let Some(id) = rs_id(query, candidates) else {
        return (
            Section::Unavailable {
                source: Source::Gwas,
                reason: Reason::NoJoinId,
            },
            Vec::new(),
            vec![Problem {
                source: Source::Gwas,
                record: None,
                field: "rs_id".into(),
                code: ProblemCode::ValueUnknown,
            }],
            None,
        );
    };
    let leg = cx.fetch.leg(&gwas::SPEC, deadline).fresh(cx.fresh);
    let size = query.section_limit();
    let offset = query.section_offset();
    // A catalog page cannot start between provider page boundaries.
    let result = gwas::associations(&leg, &id, offset / size, size);
    let error = result.as_ref().err().copied();
    let source = status::source(Source::Gwas, leg.finish(), error);
    let value = match result {
        Ok(page) if page.total == 0 => Section::Empty {
            source: Source::Gwas,
            record: id
                .to_string()
                .parse()
                .ok()
                .map(|id| biomcp_types::SourceRecord { id, url: None }),
        },
        Ok(page) => {
            let rows = page
                .rows
                .into_iter()
                .skip((offset % size) as usize)
                .collect::<Vec<_>>();
            let returned = rows.len() as u32;
            let continuation = continuation(query, &id, offset, returned, page.total);
            Section::Data {
                source: Source::Gwas,
                data: VariantGwas {
                    rs_id: id,
                    associations: rows.into_iter().map(map).collect(),
                    page: PageInfo {
                        offset,
                        returned,
                        total: page.total,
                        pool: None,
                        truncated: None,
                    },
                    continuation,
                },
            }
        }
        Err(e) => Section::unavailable(
            Source::Gwas,
            match e {
                RequestError::Source(e) => e.reason,
                _ => Reason::Unavailable,
            },
        ),
    };
    (
        value,
        vec![source],
        Vec::new(),
        error.and_then(status::local),
    )
}
fn rs_id(query: &VariantGet, candidates: &[VariantHeader]) -> Option<VariantId> {
    if let Ok(id @ VariantId::Rs(_)) = query.input().as_str().parse() {
        return Some(id);
    }
    let mut ids = Vec::new();
    for id in candidates
        .iter()
        .flat_map(|r| &r.identity.ids)
        .filter(|id| matches!(id, VariantId::Rs(_)))
    {
        if !ids.contains(id) {
            ids.push(id.clone());
        }
    }
    if ids.len() == 1 { ids.pop() } else { None }
}
fn map(row: gwas::Association) -> GwasAssociation {
    let exact_p_value = row.exact_p_value();
    GwasAssociation {
        id: row.association_id,
        study: row.accession_id,
        traits: row.reported_trait.into(),
        ontology_traits: row
            .efo_traits
            .into_iter()
            .map(|t| biomcp_types::GwasTrait {
                id: t.efo_id,
                label: t.efo_trait,
            })
            .collect(),
        alleles: row
            .snp_allele
            .into_iter()
            .map(|a| GwasEffectAllele {
                rs_id: a.rs_id,
                allele: a.effect_allele,
            })
            .collect(),
        p_value: row.p_value,
        exact_p_value,
        author: optional_text(row.author.clone()),
        mapped_genes: row.mapped_gene.clone().unwrap_or_default().into(),
        beta: optional_text(row.beta),
        odds_ratio: row.or_per_copy_num,
        risk_frequency: optional_text(row.risk_frequency),
        interval: optional_text(row.range),
        publication: optional_text(row.pubmed_id),
    }
}

fn optional_text(value: Option<String>) -> Option<biodata::Text> {
    value.and_then(|v| v.parse().ok())
}

fn continuation(
    query: &VariantGet,
    id: &VariantId,
    offset: u32,
    returned: u32,
    total: u64,
) -> Option<VariantGet> {
    let next = offset
        .checked_add(returned)
        .filter(|n| u64::from(*n) < total)?;
    Some(
        query
            .for_candidate(id.to_string().parse().ok()?)
            .ok()?
            .at_section_offset(next)
            .with_sections(Box::new([biomcp_types::VariantSection::Gwas])),
    )
}
