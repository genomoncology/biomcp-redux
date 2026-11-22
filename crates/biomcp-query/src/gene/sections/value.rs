//! Apply a source section to a resolved gene card.
use super::SectionValue;
use biomcp_types::{GeneItem, Section, SourceState};

impl SectionValue {
    pub(super) fn unavailable(&self) -> bool {
        match self {
            Self::Protein(value) => matches!(value, Section::Unavailable { .. }),
            Self::Civic(value) => matches!(value, Section::Unavailable { .. }),
            Self::ClinGen(Section::Data { data, .. }) => {
                [&data.validity_status, &data.dosage_status]
                    .into_iter()
                    .any(|outcome| {
                        matches!(
                            outcome.state,
                            SourceState::Failed | SourceState::Stale | SourceState::Syncing
                        )
                    })
            }
            Self::ClinGen(value) => matches!(value, Section::Unavailable { .. }),
            Self::Gencc(value) => matches!(value, Section::Unavailable { .. }),
            Self::ClinVar(value) => matches!(value, Section::Unavailable { .. }),
            Self::Constraint(value) => matches!(value, Section::Unavailable { .. }),
            Self::Expression(value) => matches!(value, Section::Unavailable { .. }),
            Self::Pathways(value) => matches!(value, Section::Unavailable { .. }),
            Self::Oncokb(value, _, _) => matches!(value, Section::Unavailable { .. }),
        }
    }
    pub(super) fn apply(&self, item: &mut GeneItem) {
        if let Self::Oncokb(_, card, partial) = self {
            item.oncokb = card.clone();
            if *partial {
                item.status = crate::status::item_status(true, false, true);
            }
        }
        if let Some(sections) = &mut item.sections {
            match self {
                Self::Protein(value) => sections.protein = Some(value.clone()),
                Self::Civic(value) => sections.civic = Some(value.clone()),
                Self::ClinGen(value) => sections.clingen = Some(value.clone()),
                Self::Gencc(value) => sections.gencc = Some(value.clone()),
                Self::ClinVar(value) => sections.clinvar = Some(value.clone()),
                Self::Constraint(value) => sections.constraint = Some(value.clone()),
                Self::Expression(value) => sections.expression = Some(value.clone()),
                Self::Pathways(value) => sections.pathways = Some(value.clone()),
                Self::Oncokb(value, _, _) if sections.oncokb.is_some() => {
                    sections.oncokb = Some(value.clone())
                }
                Self::Oncokb(..) => {}
            }
        }
    }
}
