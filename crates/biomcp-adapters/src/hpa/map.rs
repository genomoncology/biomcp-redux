use super::raw;
use crate::{Mapped, raw::Field};
use biodata::{
    CancerPtpm, EnsemblGeneId, Expression, Problem, ProblemCode, RnaAbundance, Source, Text,
    TissueNtpm,
};
use std::collections::BTreeMap;
pub(super) fn expression(raw: raw::Gene, id: &EnsemblGeneId) -> Mapped<Expression> {
    let mut map = Mapper {
        id,
        problems: Vec::new(),
    };
    let value = Expression {
        rna_tissue_specificity: map.text(raw.tissue_specificity, "RNA tissue specificity"),
        rna_tissue_distribution: map.text(raw.tissue_distribution, "RNA tissue distribution"),
        rna_tissue_ntpm: map
            .numbers(raw.ntpm, "RNA tissue specific nTPM")
            .into_iter()
            .map(|(tissue, ntpm)| TissueNtpm { tissue, ntpm })
            .collect(),
        rna_cancer_specificity: map.text(raw.cancer_specificity, "RNA cancer specificity"),
        rna_cancer_ptpm: map
            .numbers(raw.ptpm, "RNA cancer specific pTPM")
            .into_iter()
            .map(|(tissue, ptpm)| CancerPtpm { tissue, ptpm })
            .collect(),
        subcellular_main: map.list(raw.main, "Subcellular main location"),
        subcellular_additional: map.list(raw.additional, "Subcellular additional location"),
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
    fn problem(&mut self, field: &str) {
        self.problems.push(Problem {
            source: Source::Hpa,
            record: Text::new(self.id.base()).ok(),
            field: field.into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    fn text(&mut self, raw: Option<Field<String>>, field: &str) -> Option<Text> {
        let raw = raw?;
        if let Field::Value(value) = &raw {
            if value.is_empty() {
                return None;
            }
            if let Ok(text) = Text::new(value) {
                return Some(text);
            }
        }
        self.problem(field);
        None
    }
    fn list(&mut self, raw: Option<Field<Vec<Field<String>>>>, field: &str) -> Box<[Text]> {
        match raw {
            None => Box::default(),
            Some(Field::Value(values)) => values
                .into_iter()
                .filter_map(|value| self.text(Some(value), field))
                .collect(),
            Some(_) => {
                self.problem(field);
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
            self.problem(field);
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
                self.problem(field);
                None
            })
            .collect()
    }
}
