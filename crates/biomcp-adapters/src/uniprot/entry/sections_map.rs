use super::sections_raw as raw;
use biodata::{
    Attribution, Bound, ProteinAudit, ProteinCoordinateConvention, ProteinFeature, ProteinFunction,
    ProteinIsoform, ProteinSequenceAvailability, ProteinStructureRef, Term, Text, UncertainSpan0,
    UniProtFeature, UniProtIsoform,
};
use biomcp_types::{Problem, ProblemCode, Source};
pub(super) fn evidence(rows: Vec<raw::Evidence>) -> Box<[Text]> {
    rows.into_iter()
        .filter_map(|e| {
            let mut text = e.evidence_code.to_string();
            if let Some(source) = e.source {
                text.push_str(&format!("|{source}"));
            }
            if let Some(id) = e.id {
                text.push_str(&format!("|{id}"));
            }
            Text::from_source(&text).ok()
        })
        .collect()
}
enum Side {
    Start,
    End,
}
fn bound(v: raw::Boundary, side: Side) -> Option<Bound> {
    if matches!(v.modifier, raw::Modifier::Unknown | raw::Modifier::Unsure) {
        return Bound::new(Some(1), None).ok();
    }
    let n = v.value?;
    if n == 0 {
        return None;
    }
    match v.modifier {
        raw::Modifier::Exact => Bound::new(Some(n), Some(n)).ok(),
        raw::Modifier::Outside if matches!(side, Side::Start) => Bound::new(None, Some(n)).ok(),
        raw::Modifier::Outside => Bound::new(Some(n), None).ok(),
        _ => None,
    }
}
pub(super) fn features(
    rows: Vec<raw::Feature>,
    id: &UniProtIsoform,
    audit: &ProteinAudit,
    problems: &mut Vec<Problem>,
) -> Vec<ProteinFeature> {
    rows.into_iter()
        .filter_map(|r| {
            let span = bound(r.location.start, Side::Start)
                .zip(bound(r.location.end, Side::End))
                .and_then(|(start, end)| UncertainSpan0::from_inclusive(start, end).ok());
            let Some(span) = span else {
                problems.push(Problem {
                    source: Source::UniProt,
                    record: Text::new(id.as_str()).ok(),
                    field: "features.location".into(),
                    code: ProblemCode::ValueUnknown,
                });
                return None;
            };
            let alternative = r.alternative_sequence;
            Some(ProteinFeature {
                feature: UniProtFeature {
                    protein: r.location.sequence.unwrap_or_else(|| id.clone()),
                    span,
                    kind: Term {
                        label: r.kind,
                        codings: Box::default(),
                    },
                    description: r.description.and_then(|v| Text::from_source(&v).ok()),
                    id: r.feature_id.and_then(|v| Text::new(&v).ok()),
                    original: alternative
                        .as_ref()
                        .and_then(|a| a.original_sequence.clone()),
                    alternative: alternative
                        .map(|a| a.alternative_sequences.unwrap_or_default().into())
                        .unwrap_or_default(),
                    evidence: evidence(r.evidences.unwrap_or_default()),
                },
                attribution: Attribution {
                    source: Text::new("uniprot").ok()?,
                    record: Text::new(id.as_str()).ok(),
                    version: audit
                        .entry_version
                        .and_then(|v| Text::new(&v.to_string()).ok()),
                    release: audit.release.clone(),
                    submitter: None,
                    evaluated: audit.last_annotation_update,
                    url: Text::new(&format!("https://www.uniprot.org/uniprotkb/{id}/entry")).ok(),
                },
                source_coordinates: ProteinCoordinateConvention::OneBasedInclusive,
            })
        })
        .collect()
}
pub(super) fn comments(rows: Vec<raw::Comment>) -> (Vec<ProteinIsoform>, Vec<ProteinFunction>) {
    let mut isoforms = Vec::new();
    let mut function = Vec::new();
    for row in rows {
        if row.comment_type == "ALTERNATIVE PRODUCTS" {
            isoforms.extend(row.isoforms.unwrap_or_default().into_iter().map(|r| {
                ProteinIsoform {
                    ids: r.isoform_ids.unwrap_or_default().into(),
                    name: r.name.value,
                    synonyms: r
                        .synonyms
                        .unwrap_or_default()
                        .into_iter()
                        .map(|v| v.value)
                        .collect(),
                    sequence_status: r.isoform_sequence_status,
                    sequence_ids: r.sequence_ids.unwrap_or_default().into(),
                    direct_sequence: ProteinSequenceAvailability::NotChecked,
                }
            }));
        } else if row.comment_type == "FUNCTION" {
            function.extend(
                row.texts
                    .unwrap_or_default()
                    .into_iter()
                    .map(|p| ProteinFunction {
                        molecule: row.molecule.clone(),
                        text: p.value,
                        evidence: evidence(p.evidences.unwrap_or_default()),
                    }),
            );
        }
    }
    (isoforms, function)
}
pub(super) fn structures(rows: Vec<raw::CrossReference>) -> Vec<ProteinStructureRef> {
    rows.into_iter()
        .filter(|r| r.database.as_str() == "PDB")
        .filter_map(|r| {
            Some(ProteinStructureRef {
                url: Text::new(&format!("https://www.rcsb.org/structure/{}", r.id)).ok()?,
                database: r.database,
                id: r.id,
                properties: r.properties.unwrap_or_default().into(),
            })
        })
        .collect()
}
