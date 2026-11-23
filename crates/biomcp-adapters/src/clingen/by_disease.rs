//! Read native ClinGen validity assertions for a disease without touching dosage.
use crate::curation::{CurationLookup, CurationMatches};
use crate::{Mapped, bundle_mapping, mapped::Problems, raw::Field};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{DiseaseCuration, DiseaseCurationGene, Source};

/// Read each ClinGen validity row matching a native disease identity or title prefix.
pub fn by_disease(
    leg: &Leg<'_>,
    lookup: CurationLookup<'_>,
) -> Result<CurationMatches, RequestError> {
    let key = lookup.key();
    let (rows, snapshot) = leg.rows_with_info(&super::VALIDITY, lookup.find(&key))?;
    let mut refusals = bundle_mapping::bundle_refusals(&snapshot);
    let mut problems = Problems {
        source: Source::ClinGen,
        record: None,
        list: bundle_mapping::bundle_problems(Source::ClinGen, &snapshot),
    };
    let value = rows
        .into_iter()
        .filter(|row| {
            let (id, name) = row.disease_keys();
            lookup.matches(id, name, &key)
        })
        .filter_map(|row| {
            let gene_symbol = row.symbol.as_ref().and_then(Field::as_ref).cloned();
            match row.checked(&mut problems) {
                Some((gene_id, assertion)) => Some(DiseaseCuration::ClinGen {
                    gene: DiseaseCurationGene {
                        gene_id,
                        gene_symbol,
                    },
                    assertion,
                }),
                None => {
                    refusals = crate::Refusal::Refused;
                    None
                }
            }
        })
        .collect();
    Ok(CurationMatches {
        rows: Mapped {
            value,
            refusals,
            problems: problems.list,
        },
        snapshot,
    })
}
