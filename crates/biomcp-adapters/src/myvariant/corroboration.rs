//! Same-record allowed identifiers or a checked assembly-qualified reference confirm fallback rows.
use super::raw::Hit;
use crate::raw::{Field, OneOrMany};
use biodata::VariantId;
use biomcp_fetch::{Leg, RequestError};

pub(super) fn confirmed(hit: &Hit, leg: &Leg<'_>, reference: &Leg<'_>) -> Result<bool, RequestError> {
    let clinvar = !super::components::disabled_on_leg(leg, "clinvar")
        && hit.clinvar.as_ref().and_then(Field::as_ref)
            .and_then(|row| row.variant_id.as_ref())
            .and_then(|id| u32::try_from(*id).ok())
            .is_some_and(|id| biodata::ClinVarVariationId::new(id).is_ok());
    let dbsnp = hit.dbsnp.as_ref().and_then(Field::as_ref)
        .and_then(|row| row.rsid.as_ref()).and_then(Field::as_ref)
        .is_some_and(|ids| match ids {
            OneOrMany::One(id) => valid_rs(id),
            OneOrMany::Many(ids) => ids.iter().any(valid_rs),
        });
    if clinvar || dbsnp {
        return Ok(true);
    }
    let Some(build) = super::search_build::source_build(hit) else {
        return Ok(false);
    };
    // Failure to confirm is an exclusion, never a protein match. The shared leg records errors.
    crate::clinvar::reference_agrees(reference, hit.id.as_str(), build)
}
fn valid_rs(value: &Field<biodata::Text>) -> bool {
    value.as_ref().is_some_and(|id| matches!(id.as_str().parse::<VariantId>(), Ok(VariantId::Rs(_))))
}
