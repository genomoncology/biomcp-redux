use super::{Resolution, raw::Allele};
use biodata::{
    Build, GenomicInterval, Source, Span0, Text, VariantHeader, VariantIdentity,
    VariantRepresentation,
};
pub(super) fn allele(raw: Allele) -> Result<Resolution, biomcp_fetch::BadReply> {
    if let Some(error) = raw.error_type {
        return Ok(Resolution::Rejected(error));
    }
    let key = raw.id.and_then(|s| {
        s.as_str()
            .strip_prefix("http://reg.genome.network/allele/")
            .and_then(|v| Text::new(v).ok())
    });
    let key = key.ok_or(biomcp_fetch::BadReply)?;
    let mut aliases = Vec::new();
    for rs in raw.external.rs {
        if let Ok(id) = Text::new(&format!("rs{}", rs.rs)) {
            aliases.push(id);
        }
    }
    for cv in raw.external.clinvar {
        if let Ok(id) = Text::new(&format!("ClinVar:{}", cv.id)) {
            aliases.push(id);
        }
    }
    let mut representations = Vec::new();
    for g in raw.genomic {
        let build = g
            .assembly
            .as_ref()
            .and_then(|a| a.as_str().parse().ok())
            .unwrap_or(Build::Unstated);
        for notation in g.hgvs {
            for c in &g.coordinates {
                let coordinate = g
                    .chromosome
                    .clone()
                    .zip(Span0::new(c.start, c.end).ok())
                    .map(|(contig, span)| GenomicInterval {
                        build,
                        contig,
                        span,
                        strand: None,
                    });
                representations.push(VariantRepresentation {
                    source: Source::AlleleRegistry,
                    notation: notation.clone(),
                    reference: g.reference.clone(),
                    assembly: g.assembly.clone(),
                    coordinate,
                    reference_allele: Some(c.reference.clone()),
                    alternate_allele: Some(c.allele.clone()),
                });
            }
        }
    }
    Ok(Resolution::Found(VariantHeader {
        identity: VariantIdentity {
            source: Source::AlleleRegistry,
            key,
            aliases: aliases.into(),
            representations: representations.into(),
        },
        facts: Box::default(),
    }))
}
