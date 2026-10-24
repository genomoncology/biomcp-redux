use super::raw::{Hit, Genes};
use crate::Mapped;
use biodata::{Problem, ProblemCode, Source, Text, VariantFact, VariantHeader, VariantIdentity};
pub(super) fn candidates(hits: Vec<Hit>, query: &str) -> Mapped<Vec<VariantHeader>> {
    let mut value = Vec::new();
    let mut problems = Vec::new();
    for hit in hits {
        let aliases: Vec<Text> = hit.dbsnp.as_ref().and_then(|d| d.rsid.as_ref()).map(|v| match v {
            super::raw::OneOrMany::One(v) => vec![v.clone()], super::raw::OneOrMany::Many(v) => v.clone()
        }).unwrap_or_default();
        let matches = if query.starts_with("rs") { aliases.iter().any(|a| a.as_str() == query) }
            else if let Some(id) = query.strip_prefix("clinvar.variant_id:") { hit.clinvar.as_ref().is_some_and(|c| c.variant_id.to_string() == id) }
            else { hit.id.as_str() == query };
        if !matches || value.iter().any(|v: &VariantHeader| v.identity.key == hit.id) {
            problems.push(Problem { source: Source::MyVariant, record: Some(hit.id), field: "identity".into(), code: ProblemCode::ValueUnknown });
            continue;
        }
        let mut facts = Vec::new();
        if let Some(genes) = hit.dbsnp.and_then(|d| d.gene) {
            let genes = match genes { Genes::One(v) => vec![v], Genes::Many(v) => v };
            for gene in genes { if let Some(symbol) = gene.symbol { fact(&mut facts, "dbsnp", "gene.symbol", symbol); } }
        }
        if let Some(clinvar) = hit.clinvar { if let Ok(id) = Text::new(&clinvar.variant_id.to_string()) { fact(&mut facts, "clinvar", "variant_id", id); } }
        if let Some(ids) = hit.cosmic.and_then(|c| c.cosmic_id) { for id in ids.values() { fact(&mut facts, "cosmic", "cosmic_id", id); } }
        value.push(VariantHeader { identity: VariantIdentity { source: Source::MyVariant, key: hit.id, aliases: aliases.into(), representations: Box::default() }, facts: facts.into() });
    }
    Mapped { value, problems }
}
fn fact(facts: &mut Vec<VariantFact>, source: &str, field: &str, value: Text) {
    if let (Ok(source), Ok(field)) = (Text::new(source), Text::new(field)) { facts.push(VariantFact { source, field, value }); }
}
