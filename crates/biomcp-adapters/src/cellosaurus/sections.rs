//! Complete checked cards share one cache identity across independent section pages.
use super::{Card, raw};
use biodata::*;
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::*;
/// Read the retained complete projection with primary and qualified identity checks.
pub fn complete(
    leg: &Leg<'_>,
    accession: &CellosaurusAccession,
    source: Option<&CellLineSourceId>,
) -> Result<Card, RequestError> {
    super::fetch_card(leg, accession, source, true)
}
pub(super) fn xref(row: raw::Xref) -> CellLineXref {
    CellLineXref {
        database: row.database,
        accession: row.accession,
        label: row.label,
        url: row.url,
        category: row.category,
        iri: row.iri,
        discontinued: row.discontinued,
    }
}
fn variation(row: raw::Variation) -> CellLineVariation {
    let (genes, references): (Vec<_>, Vec<_>) = row
        .xref_list
        .unwrap_or_default()
        .into_vec()
        .into_iter()
        .map(xref)
        .partition(|r| r.database.as_str() == "HGNC");
    CellLineVariation {
        variation_type: row.variation_type,
        mutation_type: row.mutation_type,
        description: row.mutation_description,
        note: row.variation_note,
        zygosity: row.zygosity_type,
        genes: genes
            .into_iter()
            .map(|reference| CellLineGene {
                hgnc: reference.accession.as_str().parse().ok(),
                reference,
            })
            .collect(),
        references: references.into(),
        provenance: row
            .source_list
            .unwrap_or_default()
            .into_vec()
            .into_iter()
            .map(|r| CellLineProvenance {
                reference: r.reference.map(|r| CellLineReference {
                    resource_internal_ref: r.resource_internal_ref,
                }),
                institution: r.institution,
                value: r.value,
                xref: r.xref.map(xref),
            })
            .collect(),
    }
}
pub(super) fn card(mut row: raw::Record) -> Result<CellLineCard, RequestError> {
    let accession = row.accession().map_err(super::bad)?.clone();
    let names = row
        .name_list
        .iter()
        .map(|r| CellLineName {
            kind: r.kind.clone(),
            value: r.value.clone(),
        })
        .collect();
    let species = row.species_list.clone().map(|v| v.0).unwrap_or_default().into_vec();
    let diseases = row.disease_list.clone().map(|v| v.0).unwrap_or_default().into_vec();
    let warnings = super::map::warnings(row.comment_list.as_deref().unwrap_or_default());
    let xrefs = row
        .xref_list
        .take()
        .unwrap_or_default()
        .into_vec()
        .into_iter()
        .map(xref)
        .collect();
    let variants = row
        .sequence_variation_list
        .take()
        .unwrap_or_default()
        .into_vec()
        .into_iter()
        .map(variation)
        .collect();
    let mut card = super::map::card(row)?;
    let data = |data| Section::Data {
        source: Source::Cellosaurus,
        data,
    };
    card.sections.names = Some(data(CellLineNames {
        accession: accession.clone(),
        items: super::map::capped(names, usize::MAX),
        offset: 0,
        release: None,
        fetched: None,
    }));
    card.sections.identity = Some(Section::Data {
        source: Source::Cellosaurus,
        data: CellLineIdentityPage {
            accession: accession.clone(),
            species: super::map::capped(species, usize::MAX),
            diseases: super::map::capped(diseases, usize::MAX),
            warnings: super::map::capped(warnings, usize::MAX),
            offset: 0,
            release: None,
            fetched: None,
        },
    });
    card.sections.xrefs = Some(Section::Data {
        source: Source::Cellosaurus,
        data: CellLineXrefs {
            accession: accession.clone(),
            items: super::map::capped(xrefs, usize::MAX),
            offset: 0,
            release: None,
            fetched: None,
        },
    });
    card.sections.variants = Some(Section::Data {
        source: Source::Cellosaurus,
        data: CellLineVariants {
            accession,
            items: super::map::capped(variants, usize::MAX),
            offset: 0,
            release: None,
            fetched: None,
        },
    });
    Ok(card)
}
