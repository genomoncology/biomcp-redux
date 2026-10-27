use super::{
    Resolution,
    raw::{Allele as RawAllele, Genomic},
};
use crate::Mapped;
use biodata::{
    Allele, Build, Expression, Representation, SequenceLocation, Span0, Text, Variant, VariantId,
};

use biomcp_types::{Problem, ProblemCode, Source, SourceRecord, VariantHeader};
pub(super) fn allele(raw: RawAllele) -> Result<Resolution, biomcp_fetch::BadReply> {
    if let Some(error) = raw.error_type {
        return Ok(Resolution::Rejected(error));
    }
    let url = raw.id.ok_or(biomcp_fetch::BadReply)?;
    let key = url
        .as_str()
        .rsplit('/')
        .next()
        .ok_or(biomcp_fetch::BadReply)?;
    let id = key
        .parse::<VariantId>()
        .map_err(|_| biomcp_fetch::BadReply)?;
    let record = Text::new(key).map_err(|_| biomcp_fetch::BadReply)?;
    let mut ids = vec![id];
    if let Some(external) = raw.external {
        for rs in external.rs.into_iter().flatten() {
            if rs.rs > 0 {
                ids.push(VariantId::Rs(
                    biodata::RsId::new(rs.rs).map_err(|_| biomcp_fetch::BadReply)?,
                ));
            }
        }
        for cv in external.clinvar.into_iter().flatten() {
            if cv.id > 0 {
                ids.push(VariantId::ClinVarVariation(
                    biodata::ClinVarVariationId::new(cv.id).map_err(|_| biomcp_fetch::BadReply)?,
                ));
            }
        }
    }
    let mut problems = Vec::new();
    let mut representations = Vec::new();
    for g in raw.genomic.into_iter().flatten() {
        genomic(g, &record, &mut representations, &mut problems);
    }
    for t in raw.transcripts.into_iter().flatten() {
        if let Some(m) = t.mane {
            for forms in [m.nucleotide, m.protein].into_iter().flatten() {
                for form in [forms.ensembl, forms.refseq].into_iter().flatten() {
                    match Expression::hgvs(form.hgvs) {
                        Ok(expression) => representations.push(Representation {
                            source: Source::AlleleRegistry.text(),
                            expression,
                            build: None,
                            allele: None,
                        }),
                        Err(_) => problems.push(problem(&record, "representations.expression")),
                    }
                }
            }
        }
    }
    let value = VariantHeader {
        requested: None,
        identity: Variant {
            ids: ids.into(),
            representations: representations.into(),
            genes: Box::default(),
        },
        source: Source::AlleleRegistry,
        record: SourceRecord {
            id: record,
            url: Some(url),
        },
        transcripts: Box::default(),
        myvariant: None,
        sections: None,
        component_terms: Box::default(),
    };
    Ok(Resolution::Found(Mapped { value, problems }))
}
fn genomic(g: Genomic, record: &Text, out: &mut Vec<Representation>, problems: &mut Vec<Problem>) {
    let build = g
        .assembly
        .as_ref()
        .and_then(|a| a.as_str().parse::<Build>().ok());
    if g.assembly.is_some() && build.is_none() {
        problems.push(problem(record, "representations.build"));
    }
    for notation in g.hgvs {
        let expression = match Expression::hgvs(notation.clone()) {
            Ok(v) => v,
            Err(_) => {
                problems.push(problem(record, "representations.expression"));
                continue;
            }
        };
        let sequence = notation
            .as_str()
            .parse::<biodata::HgvsText>()
            .and_then(|h| h.accession());
        let agrees = g.chromosome.as_ref().is_none_or(|given| {
            build
                .zip(sequence.as_ref().ok())
                .is_some_and(|(build, accession)| {
                    biodata::Chromosome::from_accession(*accession, build)
                        .is_ok_and(|chromosome| chromosome.name() == given.as_str())
                })
        });
        if !agrees {
            problems.push(problem(record, "representations.chromosome"));
        }
        let allele = if agrees && g.coordinates.len() == 1 {
            g.coordinates.first().and_then(|c| {
                Allele::new(
                    SequenceLocation {
                        sequence: sequence.ok()?,
                        span: Span0::new(c.start, c.end).ok()?,
                    },
                    c.reference.parse().ok()?,
                    c.allele.parse().ok()?,
                )
                .ok()
            })
        } else {
            None
        };
        if allele.is_none() {
            problems.push(problem(record, "representations.allele"));
        }
        out.push(Representation {
            source: Source::AlleleRegistry.text(),
            expression,
            build,
            allele,
        });
    }
}
fn problem(record: &Text, field: &str) -> Problem {
    Problem {
        source: Source::AlleleRegistry,
        record: Some(record.clone()),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    }
}
