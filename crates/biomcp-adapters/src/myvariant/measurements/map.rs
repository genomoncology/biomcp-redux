use super::raw::*;
use crate::{
    Mapped,
    raw::{Field, OneOrMany},
};
use biodata::{
    Attribution, Build, PredictorContext, PredictorScore, Problem, ProblemCode, Source, Text,
    VariantPredictors, VariantRef,
};
use biomcp_fetch::RequestError;
struct Output {
    id: Text,
    subject: VariantRef,
    scores: Vec<PredictorScore>,
    context: Vec<PredictorContext>,
    problems: Vec<Problem>,
}
pub(super) fn project(
    record: Record,
    build: Build,
    conservation: bool,
    cadd: bool,
    dbnsfp: bool,
) -> Result<Mapped<VariantPredictors>, RequestError> {
    let (chromosome, suffix) = record
        .id
        .as_str()
        .split_once(':')
        .ok_or_else(super::super::leg_bad)?;
    let chromosome = chromosome
        .strip_prefix("chr")
        .ok_or_else(super::super::leg_bad)?
        .parse::<biodata::Chromosome>()
        .map_err(|_| super::super::leg_bad())?;
    let qualified = format!("{}:{suffix}", chromosome.accession(build));
    let subject = VariantRef::allele_hgvs(qualified.parse().map_err(|_| super::super::leg_bad())?);
    let mut out = Output {
        id: record.id,
        subject,
        scores: Vec::new(),
        context: Vec::new(),
        problems: Vec::new(),
    };
    if cadd && let Some(cadd) = record.cadd {
        out.number("cadd.phred", cadd.phred, None)?;
        out.number("cadd.rawscore", cadd.rawscore, None)?;
        if let Some(sift) = cadd.sift {
            out.number("cadd.sift.val", sift.val, sift.cat)?;
        }
    }
    if dbnsfp && let Some(mut db) = record.dbnsfp {
        out.context("dbnsfp.hgvsp", db.hgvsp.take())?;
        if let Some(ensembl) = db.ensembl.take() {
            out.context("dbnsfp.ensembl.transcriptid", ensembl.transcriptid)?;
        }
        if conservation {
            project_conservation(&mut out, db)?;
        } else {
            project_prediction(&mut out, db)?;
        }
    }
    Ok(Mapped {
        value: VariantPredictors {
            subject_qualification: biodata::SubjectQualification::AdapterDerivedReference,
            scores: out.scores.into(),
            components: Box::new([]),
            context: out.context.into(),
        },
        problems: out.problems,
    })
}
fn project_prediction(out: &mut Output, mut db: Dbnsfp) -> Result<(), RequestError> {
    if let Some(polyphen) = db.polyphen2.take() {
        for (name, score) in [
            ("dbnsfp.polyphen2.hdiv", polyphen.hdiv),
            ("dbnsfp.polyphen2.hvar", polyphen.hvar),
        ] {
            if let Some(score) = score {
                out.number(&format!("{name}.score"), score.score, score.pred)?;
            }
        }
    }
    for (name, score) in [
        ("dbnsfp.revel", db.revel),
        ("dbnsfp.mutationtaster", db.mutationtaster),
    ] {
        if let Some(score) = score {
            out.number(&format!("{name}.score"), score.score, score.pred)?;
        }
    }
    Ok(())
}
fn project_conservation(out: &mut Output, db: Dbnsfp) -> Result<(), RequestError> {
    if let Some(gerp) = db.gerp {
        for (name, value) in [
            ("nr", gerp.nr),
            ("rs", gerp.rs),
            ("rs_rankscore", gerp.rs_rankscore),
        ] {
            out.number(&format!("dbnsfp.gerp++.{name}"), value, None)?;
        }
    }
    for (name, groups) in [("phastcons", db.phastcons), ("phylop", db.phylop)] {
        if let Some(groups) = groups {
            for (group, score) in [
                ("100way_vertebrate", groups.vertebrate),
                ("17way_primate", groups.primate),
                ("470way_mammalian", groups.mammalian),
            ] {
                if let Some(score) = score {
                    out.number(&format!("dbnsfp.{name}.{group}.score"), score.score, None)?;
                    out.number(
                        &format!("dbnsfp.{name}.{group}.rankscore"),
                        score.rankscore,
                        None,
                    )?;
                }
            }
        }
    }
    if let Some(siphy) = db.siphy_29way {
        out.number(
            "dbnsfp.siphy_29way.logodds_score",
            siphy.logodds_score,
            None,
        )?;
        out.number(
            "dbnsfp.siphy_29way.logodds_rankscore",
            siphy.logodds_rankscore,
            None,
        )?;
        if let Some(pi) = siphy.pi {
            for (name, value) in [("a", pi.a), ("c", pi.c), ("g", pi.g), ("t", pi.t)] {
                out.number(&format!("dbnsfp.siphy_29way.pi.{name}"), value, None)?;
            }
        }
    }
    Ok(())
}
impl Output {
    fn problem(&mut self, field: &str) {
        self.problems.push(Problem {
            source: Source::MyVariant,
            record: Some(self.id.clone()),
            field: field.into(),
            code: ProblemCode::ValueUnknown,
        });
    }
    fn context(&mut self, path: &str, values: Texts) -> Result<(), RequestError> {
        match values {
            Some(Field::Value(values)) => self.context.push(PredictorContext {
                field: Text::new(path).map_err(|_| super::super::leg_bad())?,
                values: values.values().into(),
            }),
            Some(Field::Invalid(_)) => self.problem(path),
            None => {}
        }
        Ok(())
    }
    fn number(&mut self, path: &str, values: Numbers, labels: Texts) -> Result<(), RequestError> {
        let label = match &labels {
            Some(Field::Value(OneOrMany::One(label))) => Some(label.clone()),
            _ => None,
        };
        let label_path = path
            .strip_suffix(".val")
            .map(|prefix| format!("{prefix}.cat"))
            .or_else(|| {
                path.strip_suffix(".score")
                    .map(|prefix| format!("{prefix}.pred"))
            });
        if let Some(label_path) = label_path {
            if matches!(
                &labels,
                Some(Field::Value(OneOrMany::Many(_))) | Some(Field::Invalid(_))
            ) {
                self.context(&label_path, labels)?;
            }
        }
        let Some(values) = values else { return Ok(()) };
        let Field::Value(values) = values else {
            self.problem(path);
            return Ok(());
        };
        let array = matches!(&values, OneOrMany::Many(_));
        for (index, score) in values.values().into_iter().enumerate() {
            self.scores.push(PredictorScore {
                subject: self.subject.clone(),
                method: Text::new(&if array {
                    format!("{path}[{index}]")
                } else {
                    path.to_owned()
                })
                .map_err(|_| super::super::leg_bad())?,
                score,
                label: if array { None } else { label.clone() },
                attribution: Attribution {
                    source: Source::MyVariant,
                    record: Some(self.id.clone()),
                    version: None,
                    release: None,
                    submitter: None,
                    evaluated: None,
                    url: None,
                },
            });
        }
        Ok(())
    }
}
