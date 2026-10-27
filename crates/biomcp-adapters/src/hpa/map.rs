use super::raw;
use crate::{Mapped, raw::Field};
use biodata::{EnsemblGeneId, Text};

use biomcp_types::{CancerPtpm, GeneExpression, Problem, RnaAbundance, Source, TissueNtpm};
use std::collections::BTreeMap;
pub(super) fn expression(raw: raw::Gene, id: &EnsemblGeneId) -> Mapped<GeneExpression> {
    let mut map = Mapper {
        id,
        problems: Vec::new(),
    };
    let value = GeneExpression {
        rna_tissue_specificity: crate::mapped::field_text(
            &mut map.problems,
            Source::Hpa,
            Text::new(id.base()).ok(),
            "rna_tissue_specificity",
            raw.tissue_specificity,
        ),
        rna_tissue_distribution: map
            .optional_text(raw.tissue_distribution, "rna_tissue_distribution"),
        rna_tissue_ntpm: map
            .numbers(raw.ntpm, "rna_tissue_ntpm")
            .into_iter()
            .map(|(tissue, ntpm)| TissueNtpm { tissue, ntpm })
            .collect(),
        rna_cancer_specificity: crate::mapped::field_text(
            &mut map.problems,
            Source::Hpa,
            Text::new(id.base()).ok(),
            "rna_cancer_specificity",
            raw.cancer_specificity,
        ),
        rna_cancer_ptpm: map
            .numbers(raw.ptpm, "rna_cancer_ptpm")
            .into_iter()
            .map(|(tissue, ptpm)| CancerPtpm { tissue, ptpm })
            .collect(),
        subcellular_main: map.list(raw.main, "subcellular_main"),
        subcellular_additional: map.list(raw.additional, "subcellular_additional"),
    };
    Mapped {
        value,
        problems: map.problems,
    }
}
struct Mapper<'a> {
    id: &'a EnsemblGeneId,
    problems: Vec<Problem>,
}
impl Mapper<'_> {
    fn list(&mut self, raw: Option<Field<Vec<Field<String>>>>, field: &str) -> Box<[Text]> {
        match raw {
            None => Box::default(),
            Some(Field::Value(values)) => values
                .into_iter()
                .filter_map(|value| {
                    crate::mapped::field_text(
                        &mut self.problems,
                        Source::Hpa,
                        Text::new(self.id.base()).ok(),
                        field,
                        Some(value),
                    )
                })
                .collect(),
            Some(_) => {
                crate::mapped::push_problem(
                    &mut self.problems,
                    Source::Hpa,
                    Text::new(self.id.base()).ok(),
                    field,
                );
                Box::default()
            }
        }
    }
    fn numbers(
        &mut self,
        raw: Option<Field<BTreeMap<String, Field<String>>>>,
        field: &str,
    ) -> Vec<(Text, RnaAbundance)> {
        let Some(raw) = raw else {
            return Vec::new();
        };
        let Field::Value(values) = raw else {
            crate::mapped::push_problem(
                &mut self.problems,
                Source::Hpa,
                Text::new(self.id.base()).ok(),
                field,
            );
            return Vec::new();
        };
        values
            .into_iter()
            .filter_map(|(name, value)| {
                if let Ok(tissue) = Text::new(&name)
                    && let Field::Value(value) = value
                    && let Ok(number) = value.parse::<f64>()
                    && number >= 0.0
                    && let Ok(number) = RnaAbundance::try_from(number)
                {
                    return Some((tissue, number));
                }
                crate::mapped::push_problem(
                    &mut self.problems,
                    Source::Hpa,
                    Text::new(self.id.base()).ok(),
                    field,
                );
                None
            })
            .collect()
    }
}
