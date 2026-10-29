//! Lossless mapping of the retained native mechanism and indication facts.
use super::{Indication, Mechanism, Reference};
use biodata::{
    Attribution, DrugDiseaseReference, DrugIdentity, DrugIndicationClaim, DrugNativeReference,
    DrugNativeTarget, DrugStage, DrugTargetRelation, Error, Text,
};
/// Keep a native target record without inventing a gene mapping.
pub fn targets(
    drug: &DrugIdentity,
    rows: Vec<Mechanism>,
) -> Result<Box<[DrugTargetRelation]>, Error> {
    rows.into_iter()
        .map(|r| {
            let target = r
                .target_chembl_id
                .map(|id| {
                    Ok(DrugNativeTarget {
                        source: "chembl".parse()?,
                        id: id.parse()?,
                        label: None,
                        url: None,
                    })
                })
                .transpose()?;
            DrugTargetRelation::new(
                drug.clone(),
                None,
                target,
                r.action_type
                    .unwrap_or_else(|| "mechanism".into())
                    .parse()?,
                Some(Text::from_source(&r.mechanism_of_action)?),
                references(r.mechanism_refs)?,
                attribution(Some(r.mec_id.to_string()))?,
            )
        })
        .collect()
}
/// Keep native disease IDs, labels, stage field and references independently.
pub fn indications(
    drug: &DrugIdentity,
    rows: Vec<Indication>,
) -> Result<Box<[DrugIndicationClaim]>, Error> {
    rows.into_iter()
        .map(|r| {
            let mut diseases = Vec::new();
            for (namespace, id, label) in [
                ("EFO", r.efo_id, r.efo_term.clone()),
                ("MeSH", r.mesh_id, r.mesh_heading.clone()),
            ] {
                if let Some(id) = id {
                    diseases.push(DrugDiseaseReference {
                        namespace: namespace.parse()?,
                        id: id.parse()?,
                        label: label.map(|s| Text::from_source(&s)).transpose()?,
                    });
                }
            }
            Ok(DrugIndicationClaim {
                drug: drug.clone(),
                indication: Text::from_source(
                    &r.efo_term.or(r.mesh_heading).ok_or(Error::TextEmpty)?,
                )?,
                disease_references: diseases.into(),
                stage: r.max_phase_for_ind.map(|value| DrugStage {
                    field: "max_phase_for_ind".into(),
                    value: value.into(),
                }),
                references: references(r.indication_refs)?,
                population: None,
                combination: Box::default(),
                attribution: attribution(Some(r.drugind_id.to_string()))?,
            })
        })
        .collect()
}
fn references(rows: Vec<Reference>) -> Result<Box<[DrugNativeReference]>, Error> {
    rows.into_iter()
        .map(|r| {
            Ok(DrugNativeReference {
                ref_type: r.ref_type.parse()?,
                ref_id: Some(r.ref_id.parse()?),
                ref_url: Some(r.ref_url.parse()?),
            })
        })
        .collect()
}
fn attribution(record: Option<String>) -> Result<Attribution, Error> {
    Ok(Attribution {
        source: Text::new("chembl")?,
        record: record.map(|s| s.parse()).transpose()?,
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
        url: None,
    })
}
