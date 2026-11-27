//! Read native GenCC submissions by disease without merging contributor assertions.
use crate::curation::{CurationLookup, CurationMatches};
use crate::{Mapped, bundle_mapping, raw::Field};
use biomcp_fetch::{Find, KeyKind, Leg, RequestError};
use biomcp_types::{DiseaseCuration, DiseaseCurationGene, DiseaseId, Source};

/// Read each stored GenCC submission matching a native disease identity or title prefix.
pub fn by_disease(
    leg: &Leg<'_>,
    lookup: CurationLookup<'_>,
) -> Result<CurationMatches, RequestError> {
    let key = lookup.key();
    let submitted = match lookup {
        CurationLookup::Exact(DiseaseId::Concept(id)) => submitted_disease_key(id.as_str()),
        _ => None,
    };
    // Retained bundles may predate the submitted disease index.
    let find = submitted.as_deref().map_or_else(
        || lookup.find(&key),
        |key| Find::ExactOrUnindexed(KeyKind::Alias, key),
    );
    let (rows, snapshot) = leg.stored_rows(&super::SUBMISSIONS, find)?;
    let mut refusals = bundle_mapping::bundle_refusals(&snapshot);
    let mut problems = bundle_mapping::bundle_problems(Source::Gencc, &snapshot);
    let value = rows
        .into_iter()
        .filter(|row| {
            if let Some(key) = &submitted {
                row.submitted_disease_keys()
                    .any(|candidate| candidate == *key)
            } else {
                let (id, name) = row.disease_keys();
                lookup.matches(id, name, &key)
            }
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

impl super::Submission {
    pub(super) fn submitted_disease_keys(&self) -> impl Iterator<Item = String> {
        [&self.submitted_as_disease_id, &self.disease_curie]
            .into_iter()
            .filter_map(|id| id.as_ref().and_then(Field::as_ref))
            .filter_map(|id| submitted_disease_key(id.as_str()))
    }
}

/// Whether the identifier belongs to an exact GenCC submitter disease namespace beyond MONDO.
pub fn has_submitted_disease_namespace(id: &str) -> bool {
    submitted_disease_key(id).is_some()
}

fn submitted_disease_key(id: &str) -> Option<String> {
    let (prefix, code) = id.split_once(':')?;
    if prefix.eq_ignore_ascii_case("OMIM")
        && !code.is_empty()
        && code.bytes().all(|byte| byte.is_ascii_digit())
    {
        return Some(format!("OMIM:{code}"));
    }
    crate::orphadata::orpha_code(id).map(|code| format!("ORPHA:{code}"))
}
