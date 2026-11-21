//! Exact source namespace joins; display names never participate in identity.
use super::relation::{Selection, position};
use crate::{Ctx, Outcome, status};
use biodata::{PathwayId, PathwayMember, UniProtAcc, UniProtIsoform};
use biomcp_adapters::{hgnc, reactome};
use biomcp_fetch::{Deadline, Find, KeyKind};
use biomcp_types::{
    PathwayGeneCount, PathwayGeneCountScope, PathwayGenes, PathwayGenesQuery, PoolInfo, Problem,
    ProblemCode, ReplyBody, Source, SourceOutcome,
};
use std::collections::{HashMap, HashSet};
fn resolve(
    cx: &Ctx<'_>,
    rows: &mut [PathwayMember],
    deadline: Deadline<'_>,
    sources: &mut Vec<SourceOutcome>,
    problems: &mut Vec<Problem>,
) -> PathwayGeneCount {
    let accessions: HashSet<_> = rows.iter().filter_map(accession).collect();
    let keys: Vec<_> = accessions
        .iter()
        .map(|id| format!("uniprot:{id}"))
        .collect();
    let names: Vec<_> = keys.iter().map(String::as_str).collect();
    let mut by_accession = HashMap::new();
    if !names.is_empty() {
        let leg = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(cx.fresh);
        let mapped = hgnc::lookup(&leg, Find::AnyExact(KeyKind::Xref, &names));
        sources.push(status::source(
            Source::Hgnc,
            leg.finish(),
            mapped.as_ref().err().copied(),
        ));
        if let Ok(mapped) = mapped {
            for gene in mapped.value {
                for id in gene.uniprot {
                    by_accession
                        .entry(id)
                        .and_modify(|matched| {
                            if *matched != Some(gene.hgnc_id) {
                                *matched = None;
                            }
                        })
                        .or_insert(Some(gene.hgnc_id));
                }
            }
            problems.extend(mapped.problems);
        }
    }
    let mut distinct = HashSet::new();
    let mut unmapped = 0;
    for row in rows {
        row.gene = accession(row).and_then(|id| by_accession.get(&id).copied().flatten());
        if let Some(gene) = row.gene {
            distinct.insert(gene);
        } else if row
            .reference
            .namespace
            .as_deref()
            .is_some_and(reactome::gene_product)
        {
            unmapped += 1;
        }
    }
    PathwayGeneCount {
        scope: PathwayGeneCountScope::HgncGeneProducts,
        distinct: distinct.len() as u64,
        unmapped,
    }
}

fn report_unmapped(rows: &[PathwayMember], problems: &mut Vec<Problem>) {
    for row in rows.iter().filter(|row| row.gene.is_none()) {
        problems.push(Problem {
            source: Source::Reactome,
            record: format!(
                "{}:{}",
                row.reference.namespace.as_deref().unwrap_or(""),
                row.reference.id
            )
            .parse()
            .ok(),
            field: "genes.gene".into(),
            code: ProblemCode::ValueUnknown,
        });
    }
}

fn accession(row: &PathwayMember) -> Option<UniProtAcc> {
    if row.reference.namespace.as_deref() != Some("uniprot") {
        return None;
    }
    let stated = UniProtIsoform::new(row.reference.id.as_str())?;
    let base = stated
        .as_str()
        .split_once('-')
        .map_or(stated.as_str(), |(base, _)| base);
    UniProtAcc::new(base).ok()
}

pub(super) fn present(
    cx: &Ctx<'_>,
    q: &Selection<'_>,
    query: &PathwayGenesQuery,
    deadline: Deadline<'_>,
    mut body: PathwayGenes,
    pool: Option<PoolInfo>,
    out: &mut Outcome,
) {
    if query.gene_products() {
        keep_gene_products(&mut body);
    }
    let reactome = matches!(q.id, PathwayId::Reactome(_));
    if reactome {
        body.gene_count = Some(resolve(
            cx,
            &mut body.items,
            deadline,
            &mut out.sources,
            &mut out.problems,
        ));
    }
    body.items = body
        .items
        .iter()
        .skip(q.offset as usize)
        .take(q.limit as usize)
        .cloned()
        .collect();
    body.page = position(body.page, q, body.items.len(), pool);
    // A pool saved before 1022 lacks the top-level total; the page always has it.
    body.total = body.page.total;
    if !q.details {
        for row in &mut body.items {
            row.participants = None;
        }
    }
    if reactome {
        report_unmapped(&body.items, &mut out.problems);
    } else {
        super::gpml_membership::resolve(
            cx,
            &mut body.items,
            deadline,
            &mut out.sources,
            &mut out.problems,
        );
    }
    super::relation_steps::genes(&body, query, &mut out.next, &mut out.problems);
    // The pathway card was read first, so an empty relation is a complete answer, not an unknown pathway.
    crate::status::item(out, true, false, false);
    out.body = body.with_query(query.clone());
}
/// Leave out non-gene Reactome references from the retained relation and count them by
/// namespace, so the total and paging count kept rows. WikiPathways rows are GeneProduct
/// nodes already, so nothing is left out there (1045).
fn keep_gene_products(body: &mut biomcp_types::PathwayGenes) {
    let mut left_out = biomcp_types::PathwayLeftOut {
        rows: 0,
        namespaces: Box::default(),
    };
    if matches!(body.header.id, PathwayId::Reactome(_)) {
        let mut namespaces: Vec<biomcp_types::PathwayNamespaceCount> = Vec::new();
        let mut kept = Vec::new();
        for row in std::mem::take(&mut body.items) {
            // Reactome always states a namespace; a row without one cannot be judged, so it stays.
            let biodata::IdentifierNamespace::Stated(namespace) = &row.reference.namespace else {
                kept.push(row);
                continue;
            };
            if reactome::gene_product(namespace.as_str()) {
                kept.push(row);
                continue;
            }
            left_out.rows += 1;
            match namespaces.iter_mut().find(|c| &c.namespace == namespace) {
                Some(count) => count.rows += 1,
                None => namespaces.push(biomcp_types::PathwayNamespaceCount {
                    namespace: namespace.clone(),
                    rows: 1,
                }),
            }
        }
        body.items = kept.into();
        left_out.namespaces = namespaces.into();
    }
    body.page.total = body.items.len() as u64;
    body.left_out = Some(left_out);
}
