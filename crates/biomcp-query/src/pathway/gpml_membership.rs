//! Join GPML references to HGNC and collapse repeated nodes before paging.
use crate::{Ctx, status};
use biodata::{EnsemblGeneId, Gene, GeneSymbol, HgncId, NcbiGeneId, PathwayMember, UniProtIsoform};
use biomcp_adapters::{hgnc, wikipathways::GeneXrefs};
use biomcp_fetch::{Deadline, Find, KeyKind};
use biomcp_types::{Problem, ProblemCode, Source, SourceOutcome};
use std::collections::{HashMap, HashSet};

#[derive(Clone, Eq, Hash, PartialEq)]
enum Key {
    Xref(String),
    Id(HgncId),
    Symbol(String),
}

pub(super) fn resolve(
    cx: &Ctx<'_>,
    rows: Vec<PathwayMember>,
    deadline: Deadline<'_>,
    fresh: bool,
    sources: &mut Vec<SourceOutcome>,
    problems: &mut Vec<Problem>,
    xrefs: impl FnOnce() -> Option<GeneXrefs>,
) -> (Vec<PathwayMember>, u64) {
    let mut members = rows;
    // Rows without a direct key can still join through the publisher export.
    let keys = members.iter().map(|row| key(row).unwrap_or(None)).collect();
    join(cx, &mut members, keys, deadline, fresh, sources, problems);
    if members.iter().any(can_join)
        && let Some(xrefs) = xrefs()
    {
        let keys = members
            .iter()
            .map(|row| {
                can_join(row)
                    .then(|| xrefs.entrez(row))
                    .flatten()
                    .map(|id| Key::Xref(format!("entrez:{id}")))
            })
            .collect();
        join(cx, &mut members, keys, deadline, fresh, sources, problems);
    }
    let mut by_gene = HashMap::<HgncId, usize>::new();
    let mut distinct: Vec<PathwayMember> = Vec::new();
    let mut unmapped = 0;
    for row in members {
        let Some(id) = row.gene else {
            unmapped += 1;
            report_unmapped(row, problems);
            continue;
        };
        let index = *by_gene.entry(id).or_insert(distinct.len());
        if let Some(first) = distinct.get_mut(index) {
            first.physical_context_count += row.physical_context_count;
        } else {
            distinct.push(row);
        }
    }
    (distinct, unmapped)
}

fn can_join(row: &PathwayMember) -> bool {
    row.gene.is_none() && row.node_id.is_some() && row.reference.namespace.as_deref().is_some()
}

fn join(
    cx: &Ctx<'_>,
    members: &mut [PathwayMember],
    keys: Vec<Option<Key>>,
    deadline: Deadline<'_>,
    fresh: bool,
    sources: &mut Vec<SourceOutcome>,
    problems: &mut Vec<Problem>,
) {
    let matches = lookup(
        cx,
        &keys.iter().flatten().cloned().collect(),
        deadline,
        fresh,
        sources,
        problems,
    );
    for (row, key) in members.iter_mut().zip(keys) {
        if let Some(key) = key {
            row.gene = matches.get(&key).copied().flatten();
        }
    }
}

fn lookup(
    cx: &Ctx<'_>,
    keys: &HashSet<Key>,
    deadline: Deadline<'_>,
    fresh: bool,
    sources: &mut Vec<SourceOutcome>,
    problems: &mut Vec<Problem>,
) -> HashMap<Key, Option<HgncId>> {
    let mut matches = HashMap::new();
    if keys.is_empty() {
        return matches;
    }
    let leg = cx.fetch.leg(&hgnc::SPEC, deadline).fresh(fresh);
    let mut error = None;
    for kind in [KeyKind::Xref, KeyKind::Id, KeyKind::Symbol] {
        let values: Vec<_> = keys.iter().filter_map(|key| key.selector(kind)).collect();
        if values.is_empty() {
            continue;
        }
        let names: Vec<_> = values.iter().map(String::as_str).collect();
        match hgnc::lookup(&leg, Find::AnyExact(kind, &names)) {
            Ok(mapped) => {
                problems.extend(mapped.problems);
                for gene in mapped.value {
                    index(gene, &mut matches);
                }
            }
            Err(failure) => {
                error.get_or_insert(failure);
            }
        }
    }
    let incoming = status::source(Source::Hgnc, leg.finish(), error);
    if let Some(existing) = sources
        .iter_mut()
        .find(|source| source.source == Source::Hgnc)
    {
        status::retain_source_state(existing, &incoming);
    } else {
        sources.push(incoming);
    }
    matches
}

fn index(gene: Gene, matches: &mut HashMap<Key, Option<HgncId>>) {
    let id = gene.hgnc_id;
    let keys = [Key::Id(id), Key::Symbol(gene.symbol.to_string())]
        .into_iter()
        .chain(gene.entrez_id.map(|id| Key::Xref(format!("entrez:{id}"))))
        .chain(
            gene.ensembl_id
                .map(|id| Key::Xref(format!("ensembl:{}", id.base()))),
        )
        .chain(
            gene.uniprot
                .into_iter()
                .map(|id| Key::Xref(format!("uniprot:{id}"))),
        );
    for key in keys {
        matches
            .entry(key)
            .and_modify(|matched| {
                if *matched != Some(id) {
                    *matched = None;
                }
            })
            .or_insert(Some(id));
    }
}

fn report_unmapped(row: PathwayMember, problems: &mut Vec<Problem>) {
    problems.push(Problem {
        source: Source::WikiPathways,
        record: row.node_id.or(row.name).or(Some(row.reference.id)),
        field: "genes.gene".into(),
        code: ProblemCode::ValueUnknown,
    });
}

impl Key {
    fn selector(&self, kind: KeyKind) -> Option<String> {
        match (self, kind) {
            (Self::Id(id), KeyKind::Id) => Some(id.to_string()),
            (Self::Xref(key), KeyKind::Xref) | (Self::Symbol(key), KeyKind::Symbol) => {
                Some(key.clone())
            }
            _ => None,
        }
    }
}

fn key(row: &PathwayMember) -> Result<Option<Key>, biodata::Error> {
    let value = row.reference.id.as_str();
    let key = match row.reference.namespace.as_deref() {
        Some("Entrez Gene") => {
            let id = value
                .parse::<NcbiGeneId>()
                .map_err(|_| biodata::Error::NcbiGeneIdInvalid)?;
            Key::Xref(format!("entrez:{id}"))
        }
        Some("Ensembl") => {
            let id: EnsemblGeneId = value.parse()?;
            Key::Xref(format!("ensembl:{}", id.base()))
        }
        Some("UniProt" | "Uniprot" | "UniProtKB") => {
            let id: UniProtIsoform = value.parse()?;
            let base = id
                .as_str()
                .split_once('-')
                .map_or(id.as_str(), |(base, _)| base);
            Key::Xref(format!("uniprot:{base}"))
        }
        Some("HGNC") => match hgnc_id(value) {
            Some(id) => Key::Id(id),
            None => Key::Symbol(value.parse::<GeneSymbol>()?.to_string()),
        },
        _ => return Ok(None),
    };
    Ok(Some(key))
}

fn hgnc_id(value: &str) -> Option<biodata::HgncId> {
    if let Ok(id) = value.parse() {
        return Some(id);
    }
    let number = value.parse::<u32>().ok()?;
    if number.to_string() != value {
        return None;
    }
    biodata::HgncId::new(number).ok()
}
