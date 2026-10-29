use super::{
    Resolution,
    raw::{Allele as RawAllele, Genomic},
};
use crate::Mapped;
use crate::mapped::problem;
use biodata::{
    Allele, Build, Expression, Representation, SequenceLocation, Span0, Text, Variant, VariantId,
};

use biomcp_types::{Problem, Source, SourceRecord, VariantHeader};
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
                        Err(_) => problems.push(problem(
                            Source::AlleleRegistry,
                            Some(record.clone()),
                            "representations.expression",
                        )),
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
    Ok(Resolution::Found(Box::new(Mapped { value, problems })))
}
fn genomic(g: Genomic, record: &Text, out: &mut Vec<Representation>, problems: &mut Vec<Problem>) {
    let mut fields = crate::mapped::Problems {
        source: Source::AlleleRegistry,
        record: Some(record.clone()),
        list: Vec::new(),
    };
    let assembly = fields.field("representations.build", g.assembly);
    let build = fields.optional(
        "representations.build",
        assembly.map(|a| a.as_str().parse::<Build>()),
    );
    let chromosome = fields.field("representations.chromosome", g.chromosome);
    problems.extend(fields.list);
    for notation in g.hgvs {
        let expression = match Expression::hgvs(notation.clone()) {
            Ok(v) => v,
            Err(_) => {
                problems.push(problem(
                    Source::AlleleRegistry,
                    Some(record.clone()),
                    "representations.expression",
                ));
                continue;
            }
        };
        let sequence = notation
            .as_str()
            .parse::<biodata::HgvsText>()
            .and_then(|h| h.accession());
        let agrees = chromosome.as_ref().is_none_or(|given| {
            build
                .zip(sequence.as_ref().ok())
                .is_none_or(|(build, accession)| {
                    biodata::Chromosome::from_accession(*accession, build)
                        .is_ok_and(|chromosome| chromosome.name() == given.as_str())
                })
        });
        if !agrees {
            problems.push(problem(
                Source::AlleleRegistry,
                Some(record.clone()),
                "representations.chromosome",
            ));
        }
        let allele = if g.coordinates.len() == 1 {
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
            problems.push(problem(
                Source::AlleleRegistry,
                Some(record.clone()),
                "representations.allele",
            ));
        }
        out.push(Representation {
            source: Source::AlleleRegistry.text(),
            expression,
            build,
            allele: allele.filter(|_| agrees),
        });
    }
}
