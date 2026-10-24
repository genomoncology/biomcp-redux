use super::raw::Pos;
use biodata::{
    Build, Contig, GenomicInterval, PartialDate, Span0, Strand, Summary, SummarySource, Text,
};

pub(super) fn summary(value: String) -> Option<Summary> {
    let parsed = value.strip_suffix("].").and_then(|text| {
        let (body, marker) = text.rsplit_once(" [provided by RefSeq, ")?;
        let (month, year) = marker.split_once(' ')?;
        let months = [
            "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
        ];
        let month = months.iter().position(|candidate| *candidate == month)? as u8 + 1;
        if year.len() != 4 || !year.bytes().all(|ch| ch.is_ascii_digit()) {
            return None;
        }
        let date = PartialDate::new(year.parse().ok()?, Some(month), None).ok()?;
        Some((body, date))
    });
    let (body, dated) = parsed.map_or((value.as_str(), None), |(body, date)| (body, Some(date)));
    Some(Summary {
        text: Text::new(body).ok()?,
        source: SummarySource::RefSeq,
        dated,
    })
}

pub(super) fn interval(value: Pos, build: Build) -> Option<GenomicInterval> {
    let start = value.start?.value()?;
    let end = value.end?.value()?;
    if start == 0 || end < start {
        return None;
    }
    let strand = match value.strand {
        None => None,
        Some(field) => Some(match field.value()? {
            1 => Strand::Forward,
            -1 => Strand::Reverse,
            _ => return None,
        }),
    };
    // Source-chain inference: the pinned MyGene parser copies Ensembl bounds.
    // Ensembl counts closed one-based positions; equal endpoints span one base.
    // https://github.com/biothings/mygene.info/blob/7515239530e9784f311623bd925c4ae3ba284415/src/hub/dataload/sources/ensembl/parser.py#L224
    // https://beta.ensembl.org/help/articles/accessing-ensembl-s-refget-services
    Some(GenomicInterval {
        build,
        contig: Contig::new(&value.chr?.value()?).ok()?,
        span: Span0::new(start - 1, end).ok()?,
        strand,
    })
}

use super::{
    GeneExtras,
    raw::{Genes, Hit},
};
use crate::{
    Mapped,
    raw::{Field, OneOrMany},
};
use biodata::{EntrezGeneId, HgncId, Problem, ProblemCode, Source};
use std::collections::BTreeMap;

fn problem(id: Option<EntrezGeneId>, field: &str) -> Problem {
    Problem {
        source: Source::MyGene,
        record: id.and_then(|id| Text::new(&id.to_string()).ok()),
        field: field.into(),
        code: ProblemCode::ValueUnknown,
    }
}

pub(super) fn extras(
    reply: Genes,
    ids: &[EntrezGeneId],
    problems: &mut Vec<Problem>,
) -> Vec<Mapped<GeneExtras>> {
    let mut hits: BTreeMap<EntrezGeneId, Vec<Hit>> = BTreeMap::new();
    for hit in reply.0 {
        match hit.query.parse::<EntrezGeneId>() {
            Ok(id) if ids.contains(&id) => hits.entry(id).or_default().push(hit),
            _ => problems.push(problem(None, "query")),
        }
    }
    hits.into_iter()
        .filter_map(|(id, mut hits)| {
            if hits.len() != 1 {
                problems.push(problem(Some(id), "query"));
                return None;
            }
            let hit = hits.pop()?;
            if hit.notfound == Some(true) {
                return None;
            }
            let returned = hit
                .entrezgene
                .and_then(Field::value)
                .and_then(|value| value.parse::<EntrezGeneId>().ok());
            if returned != Some(id) {
                problems.push(problem(Some(id), "entrez_id"));
                return None;
            }
            let hgnc = hit
                .hgnc
                .and_then(Field::value)
                .and_then(|value| format!("HGNC:{value}").parse::<HgncId>().ok());
            if hgnc.is_none() {
                problems.push(problem(Some(id), "hgnc_id"));
                return None;
            }
            let mut mapped = Mapped {
                value: GeneExtras {
                    query_id: id,
                    entrez_id: id,
                    hgnc_id: hgnc,
                    summary: None,
                    spans: Box::new([]),
                },
                problems: Vec::new(),
            };
            if let Some(field) = hit.summary {
                mapped.value.summary = field.value().and_then(summary);
                if mapped.value.summary.is_none() {
                    mapped.problems.push(problem(Some(id), "summary"));
                }
            }
            let mut spans = Vec::new();
            positions(
                hit.genomic_pos,
                Build::Grch38,
                id,
                &mut spans,
                &mut mapped.problems,
            );
            positions(
                hit.genomic_pos_hg19,
                Build::Grch37,
                id,
                &mut spans,
                &mut mapped.problems,
            );
            mapped.value.spans = spans.into_boxed_slice();
            Some(mapped)
        })
        .collect()
}

fn positions(
    field: Option<Field<OneOrMany<Pos>>>,
    build: Build,
    id: EntrezGeneId,
    spans: &mut Vec<GenomicInterval>,
    problems: &mut Vec<Problem>,
) {
    let Some(field) = field else {
        return;
    };
    let name = match build {
        Build::Grch38 => "genomic_pos",
        _ => "genomic_pos_hg19",
    };
    match field.value() {
        None => problems.push(problem(Some(id), name)),
        Some(values) => {
            for value in values.values() {
                match interval(value, build) {
                    Some(value) => spans.push(value),
                    None => problems.push(problem(Some(id), name)),
                }
            }
        }
    }
}
