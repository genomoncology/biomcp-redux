//! Only the reviewed source label and subject mappings occur here.
use super::narrative::push as narrative;
use super::variant::{Annotation, Query, TumorType};
use crate::Mapped;
use crate::mapped::push_problem;
use biodata::{
    Attribution, Coding, Concept, OncokbLevelCode, OncokbOncogenicityCode, Proposition, Statement,
    Term, Text, VariantRef,
};
use biomcp_types::{OncokbVariantRecord as Record, Problem, Source};
/// Project native fields without attaching grouped implications to the queried allele.
pub fn records(row: &Annotation) -> Mapped<Vec<Record>> {
    let mut out = Mapped {
        value: Vec::new(),
        problems: Vec::new(),
    };
    if let Some(label) = &row.oncogenic {
        let coding = match label.as_str() {
            "Oncogenic" => Some(OncokbOncogenicityCode::Oncogenic),
            "Likely Oncogenic" => Some(OncokbOncogenicityCode::LikelyOncogenic),
            "Likely Neutral" => Some(OncokbOncogenicityCode::LikelyNeutral),
            "Inconclusive" => Some(OncokbOncogenicityCode::Inconclusive),
            "Resistance" => Some(OncokbOncogenicityCode::Resistance),
            "Unknown" => Some(OncokbOncogenicityCode::Unknown),
            _ => {
                push_problem(&mut out.problems, Source::Oncokb, None, "oncogenic");
                None
            }
        }
        .map(Coding::OncokbOncogenicity);
        if let Ok(label) = Text::from_source(label) {
            classification(
                &mut out.value,
                row,
                "oncogenic",
                Term {
                    label,
                    codings: coding.into_iter().collect(),
                },
                Proposition::Oncogenicity,
                Vec::new(),
            );
        }
    }
    if let Some(effect) = &row.mutation_effect {
        if let Some(label) = &effect.known_effect {
            let citations = effect
                .citations
                .as_ref()
                .map(|c| pmids(values(c.pmids.as_deref()), &mut out.problems))
                .unwrap_or_default();
            if let Ok(label) = Text::from_source(label) {
                classification(
                    &mut out.value,
                    row,
                    "mutationEffect.knownEffect",
                    Term {
                        label,
                        codings: Box::default(),
                    },
                    Proposition::FunctionalImpact,
                    citations,
                );
            }
        }
        if let Some(citations) = &effect.citations {
            out.value
                .extend(
                    values(citations.abstracts.as_deref())
                        .iter()
                        .map(|r| Record::Citation {
                            field: "mutationEffect.citations".into(),
                            text: r.abstract_content.as_deref().map(Into::into),
                            link: r.link.clone(),
                        }),
                );
        }
        narrative(
            &mut out.value,
            "mutationEffect.description",
            effect.description.as_deref(),
        );
    }
    super::narrative::annotation(row, &mut out.value);
    implications(row, &mut out);
    out
}
fn implications(row: &Annotation, out: &mut Mapped<Vec<Record>>) {
    for r in values(row.treatments.as_deref()) {
        let drugs = r
            .drugs
            .as_deref()
            .unwrap_or_default()
            .iter()
            .filter_map(|d| {
                Text::from_source(&d.drug_name).ok().map(|label| Concept {
                    id: d
                        .ncit_code
                        .as_ref()
                        .and_then(|s| format!("NCIT:{s}").parse().ok()),
                    label,
                })
            })
            .collect();
        out.value.push(Record::Implication {
            field: "treatments".into(),
            level: level(&r.level, &mut out.problems),
            tumor_type: r.level_associated_cancer_type.as_ref().and_then(concept),
            drugs,
            alterations: texts(values(r.alterations.as_deref()), false),
            statement_unmapped: true,
            statement: None,
            citations: pmids(values(r.pmids.as_deref()), &mut out.problems).into(),
            abstracts: abstracts(values(r.abstracts.as_deref())),
            description: r.description.as_deref().map(Into::into),
            excluded_tumor_types: values(r.level_excluded_cancer_types.as_deref())
                .iter()
                .filter_map(concept)
                .collect(),
            approved_indications: texts(values(r.approved_indications.as_deref()), true),
            fda_level: r
                .fda_level
                .as_deref()
                .and_then(|s| Text::new(s).ok())
                .map(|label| Term {
                    label,
                    codings: Box::default(),
                }),
        });
    }
    for (field, rows) in [
        (
            "diagnosticImplications",
            values(row.diagnostic_implications.as_deref()),
        ),
        (
            "prognosticImplications",
            values(row.prognostic_implications.as_deref()),
        ),
    ] {
        for r in rows {
            out.value.push(Record::Implication {
                field: field.into(),
                level: level(&r.level_of_evidence, &mut out.problems),
                tumor_type: r.tumor_type.as_ref().and_then(concept),
                drugs: Box::default(),
                alterations: texts(values(r.alterations.as_deref()), false),
                statement_unmapped: true,
                statement: None,
                citations: pmids(values(r.pmids.as_deref()), &mut out.problems).into(),
                abstracts: abstracts(values(r.abstracts.as_deref())),
                description: r.description.as_deref().map(Into::into),
                excluded_tumor_types: Box::default(),
                approved_indications: Box::default(),
                fda_level: None,
            });
        }
    }
}

fn classification(
    out: &mut Vec<Record>,
    row: &Annotation,
    field: &str,
    classification: Term,
    proposition: Proposition,
    citations: Vec<biodata::Pmid>,
) {
    if let Some(subject) = subject(&row.query) {
        out.push(Record::Statement {
            statement: Box::new(Statement {
                subject,
                proposition,
                predicate: None,
                direction: None,
                classification,
                strength: None,
                condition: None,
                therapies: Box::default(),
                allele_origin: None,
                gene_context: row
                    .query
                    .hugo_symbol
                    .clone()
                    .and_then(|g| g.try_into().ok()),
                criteria: Box::default(),
                citations: citations.into(),
                attribution: Attribution {
                    source: Source::Oncokb.text(),
                    record: None,
                    version: None,
                    release: row.data_version.as_deref().and_then(|s| Text::new(s).ok()),
                    submitter: None,
                    evaluated: None,
                    url: None,
                },
            }),
        });
    } else {
        out.push(Record::UnmappedLabel {
            field: field.into(),
            classification,
            citations: citations.into(),
            statement_unmapped: true,
        });
    }
}
fn subject(query: &Query) -> Option<VariantRef> {
    Some(VariantRef::protein(
        query.hugo_symbol.clone()?.try_into().ok()?,
        query.alteration.clone()?,
    ))
}
fn texts(values: &[String], prose: bool) -> Box<[Text]> {
    let admit = if prose { Text::from_source } else { Text::new };
    values.iter().filter_map(|s| admit(s).ok()).collect()
}

fn concept(row: &TumorType) -> Option<Concept> {
    Some(Concept {
        id: row
            .code
            .as_ref()
            .and_then(|s| format!("ONCOTREE:{s}").parse().ok()),
        label: Text::from_source(row.name.as_deref().or(row.code.as_deref())?).ok()?,
    })
}
fn pmids(values: &[String], problems: &mut Vec<Problem>) -> Vec<biodata::Pmid> {
    values
        .iter()
        .filter_map(|s| match s.parse() {
            Ok(v) => Some(v),
            Err(_) => {
                push_problem(problems, Source::Oncokb, None, "pmids");
                None
            }
        })
        .collect()
}
fn level(value: &Text, problems: &mut Vec<Problem>) -> Term {
    let code = match value.as_str() {
        "LEVEL_1" => Some(OncokbLevelCode::Level1),
        "LEVEL_2" => Some(OncokbLevelCode::Level2),
        "LEVEL_3A" => Some(OncokbLevelCode::Level3a),
        "LEVEL_3B" => Some(OncokbLevelCode::Level3b),
        "LEVEL_4" => Some(OncokbLevelCode::Level4),
        "LEVEL_R1" => Some(OncokbLevelCode::LevelR1),
        "LEVEL_R2" => Some(OncokbLevelCode::LevelR2),
        "LEVEL_Dx1" => Some(OncokbLevelCode::LevelDx1),
        "LEVEL_Dx2" => Some(OncokbLevelCode::LevelDx2),
        "LEVEL_Dx3" => Some(OncokbLevelCode::LevelDx3),
        "LEVEL_Px1" => Some(OncokbLevelCode::LevelPx1),
        "LEVEL_Px2" => Some(OncokbLevelCode::LevelPx2),
        "LEVEL_Px3" => Some(OncokbLevelCode::LevelPx3),
        _ => {
            push_problem(problems, Source::Oncokb, None, "level");
            None
        }
    };
    Term {
        label: value.clone(),
        codings: code.map(Coding::OncokbLevel).into_iter().collect(),
    }
}
fn abstracts(rows: &[super::variant::Abstract]) -> Box<[biomcp_types::OncokbCitation]> {
    rows.iter()
        .map(|row| biomcp_types::OncokbCitation {
            text: row.abstract_content.as_deref().map(Into::into),
            link: row.link.clone(),
        })
        .collect()
}

fn values<T>(rows: Option<&[T]>) -> &[T] {
    rows.unwrap_or_default()
}
