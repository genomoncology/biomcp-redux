//! Preserve native trial records while disclosing rejected fields.
use super::{Trial, trial_raw as raw};
use crate::mapped::Problems;
impl From<raw::Trial> for Trial {
    fn from(row: raw::Trial) -> Self {
        let mut p = Problems {
            source: biomcp_types::Source::Nci,
            record: crate::mapped::record(row.nct_id),
            list: Vec::new(),
        };
        let eligibility = p
            .field("eligibility", row.eligibility)
            .map(|v| v.map(&mut p));
        let diseases = p
            .fields("diseases", row.diseases)
            .into_vec()
            .into_iter()
            .map(|v| disease(v, &mut p))
            .collect();
        let outcomes = p
            .fields("outcome_measures", row.outcome_measures)
            .into_vec()
            .into_iter()
            .map(|v| outcome(v, &mut p))
            .collect();
        let arms = arms(row.arms, &mut p);
        Self {
            nci_id: row.nci_id,
            nct_id: row.nct_id,
            brief_title: p.source_field("brief_title", row.brief_title),
            current_trial_status: p.field("current_trial_status", row.current_trial_status),
            phase: p.field("phase", row.phase),
            brief_summary: p.field("brief_summary", row.brief_summary),
            diseases,
            eligibility,
            arms,
            outcome_measures: outcomes,
            biomarkers: row.biomarkers,
            sites: row.sites,
            problems: p.list,
        }
    }
}
fn disease(row: raw::Disease, p: &mut Problems) -> biomcp_types::NciDisease {
    biomcp_types::NciDisease {
        name: p.source_field("diseases.name", row.name),
        nci_thesaurus_concept_id: row.nci_thesaurus_concept_id,
        inclusion_indicator: p.field("diseases.inclusion_indicator", row.inclusion_indicator),
        is_lead_disease: p.field("diseases.is_lead_disease", row.is_lead_disease),
        kind: p.fields("diseases.kind", row.kind),
        parents: p.fields("diseases.parents", row.parents),
        synonyms: p.fields("diseases.synonyms", row.synonyms),
    }
}
fn intervention(row: raw::Intervention, p: &mut Problems) -> biomcp_types::NciIntervention {
    biomcp_types::NciIntervention {
        name: p.source_field("arms.interventions.name", row.name),
        description: p.field("arms.interventions.description", row.description),
        nci_thesaurus_concept_id: row.nci_thesaurus_concept_id,
        r#type: p.field("arms.interventions.type", row.r#type),
        category: p.field("arms.interventions.category", row.category),
        inclusion_indicator: p.field(
            "arms.interventions.inclusion_indicator",
            row.inclusion_indicator,
        ),
        parents: p.fields("arms.interventions.parents", row.parents),
        synonyms: p.fields("arms.interventions.synonyms", row.synonyms),
    }
}
fn outcome(row: raw::Outcome, p: &mut Problems) -> biomcp_types::NciOutcome {
    biomcp_types::NciOutcome {
        name: p.field("outcome_measures.name", row.name),
        description: p.field("outcome_measures.description", row.description),
        timeframe: p.field("outcome_measures.timeframe", row.timeframe),
        type_code: p.field("outcome_measures.type_code", row.type_code),
    }
}
fn arms(
    rows: Option<crate::raw::Field<Vec<crate::raw::Field<raw::Arm>>>>,
    p: &mut Problems,
) -> Box<[biomcp_types::NciArmRecord]> {
    let mut result = Vec::new();
    for (index, row) in p.fields("arms", rows).into_vec().into_iter().enumerate() {
        let arm_index = index as u32;
        result.push(biomcp_types::NciArmRecord::Arm {
            arm_index,
            data: biomcp_types::NciArm {
                name: p.field("arms.name", row.name),
                description: p.field("arms.description", row.description),
                r#type: p.field("arms.type", row.r#type),
            },
        });
        for (index, row) in p
            .fields("arms.interventions", row.interventions)
            .into_vec()
            .into_iter()
            .enumerate()
        {
            result.push(biomcp_types::NciArmRecord::Intervention {
                arm_index,
                index: index as u32,
                data: intervention(row, p),
            });
        }
    }
    result.into()
}
