//! Read native GenCC submissions by disease without merging contributor assertions.
use crate::curation::{CurationMatches, Lookup};
use crate::{Mapped, bundle_mapping, raw::Field};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{DiseaseCuration, DiseaseCurationGene, DiseaseId, Source};

/// Read each stored GenCC submission whose native disease ID or title matches.
pub fn by_disease(leg: &Leg<'_>, id: &DiseaseId) -> Result<CurationMatches, RequestError> {
    let lookup = Lookup::new(id);
    let (rows, snapshot) = leg.stored_rows(&super::SUBMISSIONS, Lookup::find(&lookup.key()))?;
    let mut refusals = bundle_mapping::bundle_refusals(&snapshot);
    let mut problems = bundle_mapping::bundle_problems(Source::Gencc, &snapshot);
    let value = rows
        .into_iter()
        .filter(|row| {
            let (id, name) = row.disease_keys();
            lookup.matches(id, name)
        })
        .filter_map(|row| {
            let gene_symbol = row.gene_symbol.as_ref().and_then(Field::as_ref).cloned();
            let mapped = row.into_record(None);
            refusals.include(mapped.refusals);
            problems.extend(mapped.problems);
            mapped
                .value
                .map(|(gene_id, assertion)| DiseaseCuration::Gencc {
                    gene: DiseaseCurationGene {
                        gene_id,
                        gene_symbol,
                    },
                    assertion,
                })
        })
        .collect();
    Ok(CurationMatches {
        rows: Mapped {
            value,
            refusals,
            problems,
        },
        snapshot,
    })
}
