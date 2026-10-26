//! Each requested collection retains whole source rows and its independent total.
use super::*;
use biomcp_types::*;
fn page<T>(rows: &mut Capped<T>, query: &CellLineGet, default: u32) {
    rows.items = std::mem::take(&mut rows.items)
        .into_vec()
        .into_iter()
        .skip(query.section_offset() as usize)
        .take(query.section_limit().unwrap_or(default) as usize)
        .collect();
}
pub(super) fn select(query: &CellLineGet, card: &mut CellLineCard) {
    let section = query.section();
    if section != Some(CellLineSection::Names) {
        card.sections.names = None;
    }
    if section != Some(CellLineSection::Identity) {
        card.sections.identity = None;
    }
    if section != Some(CellLineSection::Xrefs) {
        card.sections.xrefs = None;
    }
    if section != Some(CellLineSection::Variants) {
        card.sections.variants = None;
    }
    if let Some(Section::Data { data, .. }) = &mut card.sections.names {
        page(&mut data.items, query, 5);
        data.offset = query.section_offset();
    }
    if let Some(Section::Data { data, .. }) = &mut card.sections.identity {
        page(&mut data.species, query, 5);
        page(&mut data.diseases, query, 5);
        page(&mut data.warnings, query, 5);
        data.offset = query.section_offset();
    }
    if let Some(Section::Data { data, .. }) = &mut card.sections.xrefs {
        page(&mut data.items, query, 5);
        data.offset = query.section_offset();
    }
    if let Some(Section::Data { data, .. }) = &mut card.sections.variants {
        page(&mut data.items, query, 3);
        data.offset = query.section_offset();
    }
    card.more = card
        .more
        .iter()
        .copied()
        .filter(|s| Some(*s) != section)
        .collect();
}
pub(super) fn actions(query: &CellLineGet, card: &CellLineCard, out: &mut Outcome) {
    let total = match query.section() {
        Some(CellLineSection::Names) => card.sections.names.as_ref().and_then(|s| match s {
            Section::Data { data, .. } => Some(data.items.total),
            _ => None,
        }),
        Some(CellLineSection::Xrefs) => card.sections.xrefs.as_ref().and_then(|s| match s {
            Section::Data { data, .. } => Some(data.items.total),
            _ => None,
        }),
        Some(CellLineSection::Variants) => card.sections.variants.as_ref().and_then(|s| match s {
            Section::Data { data, .. } => Some(data.items.total),
            _ => None,
        }),
        Some(CellLineSection::Identity) => card.sections.identity.as_ref().and_then(|s| match s {
            Section::Data { data, .. } => Some(
                data.species
                    .total
                    .max(data.diseases.total)
                    .max(data.warnings.total),
            ),
            _ => None,
        }),
        _ => None,
    };
    let limit =
        query
            .section_limit()
            .unwrap_or(if query.section() == Some(CellLineSection::Variants) {
                3
            } else {
                5
            });
    if let Some(offset) = query.section_offset().checked_add(limit)
        && total.is_some_and(|total| u64::from(offset) < total)
    {
        out.next.push((
            StepKind::Page,
            run(Request::GetCellLine(query.at_section_offset(offset))),
        ));
    }
    if let Some(Section::Data { data, .. }) = &card.sections.variants {
        pivots(data, out);
    }
}
fn pivots(data: &CellLineVariants, out: &mut Outcome) {
    let mut requests = Vec::new();
    for row in &data.items.items {
        for gene in &row.genes {
            if let Some(id) = &gene.hgnc
                && let Ok(query) = GeneGet::new(vec![GeneInput::HgncId(id.clone())].into())
            {
                requests.push(Request::GetGene(query));
            }
        }
        for source in &row.provenance {
            if let Some(reference) = &source.reference
                && let Some(value) = reference
                    .resource_internal_ref
                    .as_str()
                    .strip_prefix("PubMed=")
                && let Ok(pmid) = value.parse()
                && let Ok(query) = ArticleGet::new(vec![ArticleInput::Pmid(pmid)].into())
            {
                requests.push(Request::GetArticle(query));
            }
        }
    }
    let mut seen = std::collections::BTreeSet::new();
    for request in requests {
        if out.next.len() < 5 && serde_json::to_string(&request).is_ok_and(|key| seen.insert(key)) {
            out.next.push((StepKind::Pivot, run(request)));
        }
    }
}
