//! Read native GenCC submissions by disease without merging contributor assertions.
use crate::curation::{CurationLookup, CurationMatches};
use crate::{Mapped, bundle_mapping, raw::Field};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{DiseaseCuration, DiseaseCurationGene, Source};

/// Read each stored GenCC submission matching a native disease identity or title prefix.
pub fn by_disease(
    leg: &Leg<'_>,
    lookup: CurationLookup<'_>,
) -> Result<CurationMatches, RequestError> {
    let key = lookup.key();
    let (rows, snapshot) = leg.stored_rows(&super::SUBMISSIONS, lookup.find(&key))?;
    let mut refusals = bundle_mapping::bundle_refusals(&snapshot);
    let mut problems = bundle_mapping::bundle_problems(Source::Gencc, &snapshot);
    let value = rows
        .into_iter()
        .filter(|row| {
            let (id, name) = row.disease_keys();
            lookup.matches(id, name, &key)
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
