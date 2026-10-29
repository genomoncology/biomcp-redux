//! Native source stages and complete mechanism members, without regulator inference.
use super::{Indication, Mechanism};
use biodata::{
    Attribution, DrugDiseaseReference, DrugIdentity, DrugIndicationClaim, DrugNativeTarget,
    DrugStage, DrugTargetRelation, Error, GeneRef, Text,
};
/// Preserve every native target member and both its supplied symbol and full name.
pub fn targets(
    drug: &DrugIdentity,
    rows: Vec<Mechanism>,
) -> Result<Box<[DrugTargetRelation]>, Error> {
    let mut mapped = Vec::new();
    for row in rows {
        if row.targets.is_empty() {
            return Err(Error::DrugTargetAbsent);
        }
        for target in row.targets {
            let gene = GeneRef::new(
                target.approved_symbol.parse()?,
                None,
                None,
                Some(target.id.parse()?),
            )?;
            let native = DrugNativeTarget {
                source: "opentargets".parse()?,
                id: target.id.parse()?,
                label: Some(Text::from_source(&target.approved_name)?),
                url: None,
            };
            mapped.push(DrugTargetRelation::new(
                drug.clone(),
                Some(gene),
                Some(native),
                row.action_type.as_deref().unwrap_or("mechanism").parse()?,
                Some(Text::from_source(&row.mechanism_of_action)?),
                Box::default(),
                attribution(drug)?,
            )?);
        }
    }
    Ok(mapped.into())
}
/// Retain the exact source disease ID, its label and the unchanged stage field/value.
pub fn indications(
    drug: &DrugIdentity,
    rows: Vec<Indication>,
) -> Result<Box<[DrugIndicationClaim]>, Error> {
    rows.into_iter()
        .map(|row| {
            let disease = DrugDiseaseReference {
                namespace: row
                    .disease
                    .id
                    .split('_')
                    .next()
                    .unwrap_or("opentargets")
                    .parse()?,
                id: row.disease.id.parse()?,
                label: Some(Text::from_source(&row.disease.name)?),
            };
            Ok(DrugIndicationClaim {
                drug: drug.clone(),
                indication: Text::from_source(&row.disease.name)?,
                disease_references: Box::new([disease]),
                stage: Some(DrugStage {
                    field: "maxClinicalStage".into(),
                    value: row.max_clinical_stage.into(),
                }),
                references: Box::default(),
                population: None,
                combination: Box::default(),
                attribution: attribution(drug)?,
            })
        })
        .collect()
}
fn attribution(drug: &DrugIdentity) -> Result<Attribution, Error> {
    Ok(Attribution {
        source: "opentargets".parse()?,
        record: Some(drug.id.to_string().parse()?),
        version: None,
        release: None,
        submitter: None,
        evaluated: None,
        url: None,
    })
}
