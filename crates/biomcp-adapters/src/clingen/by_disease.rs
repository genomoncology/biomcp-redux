//! Read native ClinGen validity assertions for a disease without touching dosage.
use crate::curation::{CurationMatches, Lookup};
use crate::{Mapped, bundle_mapping, mapped::Problems, raw::Field};
use biomcp_fetch::{Leg, RequestError};
use biomcp_types::{DiseaseCuration, DiseaseCurationGene, DiseaseId, Source};

/// Read each ClinGen validity row whose native MONDO ID or label matches.
pub fn by_disease(leg: &Leg<'_>, id: &DiseaseId) -> Result<CurationMatches, RequestError> {
    let lookup = Lookup::new(id);
    let (rows, snapshot) = leg.rows_with_info(&super::VALIDITY, Lookup::find(&lookup.key()))?;
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
            lookup.matches(id, name)
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
