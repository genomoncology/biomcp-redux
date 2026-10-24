use super::raw::{Genes, Hit, OneOrMany};
use crate::Mapped;
use biodata::{
    Build, Expression, ExpressionSyntax, Problem, ProblemCode, Representation, Source,
    SourceRecord, Text, Variant, VariantHeader, VariantId,
};
pub(super) fn candidates(
    hits: Vec<Hit>,
    query: &str,
    build: Option<Build>,
) -> Mapped<Vec<VariantHeader>> {
    let mut value = Vec::new();
    let mut problems = Vec::new();
    for hit in hits {
        let aliases = hit
            .dbsnp
            .as_ref()
            .and_then(|d| d.rsid.as_ref())
            .map(|v| match v {
                OneOrMany::One(v) => vec![v.clone()],
                OneOrMany::Many(v) => v.clone(),
            })
            .unwrap_or_default();
        if !hit.matches(query) || value.iter().any(|v: &VariantHeader| v.record.id == hit.id) {
            problems.push(problem(hit.id, "identity"));
            continue;
        }
        let mut ids = Vec::new();
        for alias in aliases {
            match alias.as_str().parse::<VariantId>() {
                Ok(id) => ids.push(id),
                Err(_) => problems.push(problem(hit.id.clone(), "ids")),
            }
        }
        if let Some(cv) = hit.clinvar
            && cv.variant_id > 0
        {
            ids.push(VariantId::ClinVarVariation(cv.variant_id));
        }
        let mut genes = Vec::new();
        if let Some(source_genes) = hit.dbsnp.and_then(|d| d.gene) {
            let source_genes = match source_genes {
                Genes::One(v) => vec![v],
                Genes::Many(v) => v,
            };
            for gene in source_genes {
                if let Some(symbol) = gene.symbol {
                    match symbol.as_str().parse() {
                        Ok(g) => genes.push(g),
                        Err(_) => problems.push(problem(hit.id.clone(), "genes")),
                    }
                }
            }
        }
        if build.is_none() {
            problems.push(problem(hit.id.clone(), "representations.build"));
        }
        let expression = Expression::new(ExpressionSyntax::HgvsG, hit.id.clone(), None);
        let representations = match expression {
            Ok(expression) => vec![Representation {
                source: Source::MyVariant,
                expression,
                build,
                allele: None,
            }],
            Err(_) => {
                problems.push(problem(hit.id.clone(), "representations"));
                continue;
            }
        };
        value.push(VariantHeader {
            identity: Variant {
                ids: ids.into(),
                representations: representations.into(),
                genes: genes.into(),
            },
            source: Source::MyVariant,
            record: SourceRecord {
                id: hit.id,
                url: None,
            },
            myvariant: None,
        });
    }
    Mapped { value, problems }
}
fn problem(record: Text, field: &str) -> Problem {
    Problem {
        source: Source::MyVariant,
        record: Some(record),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    }
}
