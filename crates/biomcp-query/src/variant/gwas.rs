//! Source-local GWAS Catalog pages, with no inference from catalog absence.
use crate::{Ctx, status};
use biodata::{
    GwasAssociation, GwasEffectAllele, PageInfo, Problem, ProblemCode, Reason, Section, Source,
    VariantGet, VariantGwas, VariantHeader, VariantId,
};
use biomcp_adapters::gwas;
use biomcp_fetch::{Deadline, RequestError};
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
                .map(|id| biodata::SourceRecord { id, url: None }),
        },
        Ok(page) => {
            let rows = page
                .rows
                .into_iter()
                .skip((offset % size) as usize)
                .collect::<Vec<_>>();
            let returned = rows.len() as u32;
            let continuation = offset
                .checked_add(returned)
                .filter(|next| u64::from(*next) < page.total)
                .map(|next| query.at_section_offset(next));
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
    GwasAssociation {
        id: row.association_id,
        study: row.accession_id,
        traits: row.reported_trait.into(),
        ontology_traits: row
            .efo_traits
            .into_iter()
            .map(|t| biodata::GwasTrait {
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
        beta: row.beta,
        odds_ratio: row.or_per_copy_num,
        risk_frequency: row.risk_frequency,
        interval: row.range,
        publication: row.pubmed_id,
    }
}
