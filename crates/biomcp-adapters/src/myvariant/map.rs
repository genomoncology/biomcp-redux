use super::raw::{Genes, Hit, OneOrMany};
use crate::Mapped;
use crate::mapped::problem;
use biodata::{Build, Expression, ExpressionSyntax, Representation, Text, Variant, VariantId};

use biomcp_types::{Source, SourceRecord, VariantHeader};
pub(super) fn candidates(
    hits: Vec<Hit>,
    query: &str,
    build: Option<Build>,
    allow_clinvar: bool,
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
            problems.push(problem(Source::MyVariant, Some(hit.id), "identity"));
            continue;
        }
        let mut ids = Vec::new();
        for alias in aliases {
            match alias.as_str().parse::<VariantId>() {
                Ok(id) => ids.push(id),
                Err(_) => problems.push(problem(Source::MyVariant, Some(hit.id.clone()), "ids")),
            }
        }
        if allow_clinvar
            && let Some(cv) = hit.clinvar
            && let Ok(id) = biodata::ClinVarVariationId::new(cv.variant_id)
        {
            ids.push(VariantId::ClinVarVariation(id));
        }
        let component_terms = component_terms(&ids);
        let genes = genes(hit.dbsnp.and_then(|d| d.gene), &hit.id, &mut problems);
        if build.is_none() {
            problems.push(problem(
                Source::MyVariant,
                Some(hit.id.clone()),
                "representations.build",
            ));
        }
        let expression = Expression::new(ExpressionSyntax::HgvsG, hit.id.clone(), None);
        let representations = match expression {
            Ok(expression) => vec![Representation {
                source: Source::MyVariant.text(),
                expression,
                build,
                allele: None,
            }],
            Err(_) => {
                problems.push(problem(
                    Source::MyVariant,
                    Some(hit.id.clone()),
                    "representations",
                ));
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
            requested: None,
            sections: None,
            component_terms,
        });
    }
    Mapped { value, problems }
}

fn component_terms(ids: &[VariantId]) -> Box<[biomcp_types::ComponentLabel]> {
    if ids
        .iter()
        .any(|id| matches!(id, VariantId::ClinVarVariation(_)))
    {
        Text::new("clinvar")
            .ok()
            .zip(super::components::label(super::components::CLINVAR.terms).ok())
            .map(|(component, terms)| biomcp_types::ComponentLabel { component, terms })
            .into_iter()
            .collect()
    } else {
        Box::default()
    }
}

fn genes(
    raw: Option<Genes>,
    id: &Text,
    problems: &mut Vec<biomcp_types::Problem>,
) -> Vec<biodata::GeneRef> {
    let mut genes = Vec::new();
    if let Some(source_genes) = raw {
        let source_genes = match source_genes {
            Genes::One(v) => vec![v],
            Genes::Many(v) => v,
        };
        for gene in source_genes {
            if let Some(symbol) = gene.symbol {
                match biodata::GeneRef::new(symbol, None, None, None) {
                    Ok(g) => genes.push(g),
                    Err(_) => problems.push(problem(Source::MyVariant, Some(id.clone()), "genes")),
                }
            }
        }
    }
    genes
}
