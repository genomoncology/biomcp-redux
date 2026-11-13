//! After a reference mismatch, ask VariantValidator which of the gene's RefSeq
//! transcripts carry the given reference base. VariantValidator itself validates
//! the gene-symbol description on each transcript; BioMCP keeps the ones it accepts.
use super::{Build, Leg, RequestError, Source, fetch_on};
use crate::{Mapped, mapped::Problems, raw::Field};
use biodata::Text;
use biomcp_fetch::{Arg, BadReply, Endpoint, Input, PathSegment, Read, Reply};
use biomcp_types::TranscriptFit;
use serde::Deserialize;
use std::collections::BTreeMap;

const TRANSCRIPTS: Endpoint = super::SPEC
    .get("/tools/gene2transcripts_v2")
    .fresh(biomcp_fetch::Freshness::Days(1));

#[derive(Deserialize)]
#[serde(transparent)]
struct Genes(Vec<Field<Gene>>);
#[derive(Deserialize)]
struct Gene {
    current_symbol: Option<Field<Text>>,
    transcripts: Option<Field<Vec<Field<Transcript>>>>,
}
#[derive(Deserialize)]
struct Transcript {
    reference: Field<Text>,
}
impl Reply for Genes {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.is_empty() || self.0.len() > 10 {
            return Err(BadReply);
        }
        Ok(())
    }
}

/// Transcripts of the input's gene where VariantValidator accepts the same c. change.
/// Only a RefSeq NM_ c. description is tried; anything else returns no fits.
pub fn transcript_fits(
    leg: &Leg<'_>,
    input: &str,
    build: Build,
) -> Result<Mapped<Vec<TranscriptFit>>, RequestError> {
    let mut problems = Problems {
        source: Source::VariantValidator,
        record: None,
        list: Vec::new(),
    };
    let mut refusals = crate::Refusal::None;
    let mut fits = Vec::new();
    if let Some((accession, body)) = parts(input) {
        let assembly = build.to_string();
        let endpoint = TRANSCRIPTS.clone().with_segments(&[
            PathSegment::new(accession)?,
            PathSegment::new("all")?,
            PathSegment::new("refseq")?,
            PathSegment::new(&assembly)?,
        ]);
        let genes = leg
            .call(
                &endpoint,
                Input::Params(&[("content-type", Arg::Text("application/json"))]),
                Read::<Genes>::json(),
            )?
            .value;
        if let Some((symbol, transcripts)) = gene(genes, accession, &mut problems) {
            let query = format!("{symbol}:{body}");
            let raw = fetch_on(leg, &query, build, &transcripts.join("|"))?;
            if raw.flag.as_str() != "warning" {
                let key = assembly.to_ascii_lowercase();
                for (_, row) in raw.records(&query, &mut problems, &mut refusals) {
                    let genomic = problems
                        .field("primary_assembly_loci", row.primary_assembly_loci)
                        .and_then(|mut loci| loci.remove(&key))
                        .and_then(|locus| problems.field(&key, Some(locus)))
                        .and_then(|locus| {
                            problems.field(&key, Some(locus.hgvs_genomic_description))
                        })
                        .map(|value| value.to_string().into());
                    fits.push(TranscriptFit {
                        hgvs: row.hgvs_transcript_variant.into(),
                        genomic,
                    });
                }
            }
        }
    }
    Ok(Mapped {
        refusals,
        value: fits,
        problems: problems.list,
    })
}

fn parts(input: &str) -> Option<(&str, &str)> {
    let (reference, body) = input.split_once(':')?;
    let accession = reference
        .split_once('(')
        .map_or(reference, |(outer, _)| outer);
    (accession.starts_with("NM_") && body.starts_with("c.")).then_some((accession, body))
}

/// The gene symbol and the latest version of each of its other NM_ transcripts.
fn gene(genes: Genes, input: &str, problems: &mut Problems) -> Option<(Text, Vec<String>)> {
    let gene = genes.0.into_iter().find_map(Field::value)?;
    let symbol = problems.field("current_symbol", gene.current_symbol)?;
    if symbol.as_str().parse::<biodata::GeneSymbol>().is_err() {
        problems.required::<()>("current_symbol", None);
        return None;
    }
    let own = input.split_once('.').map_or(input, |(base, _)| base);
    let mut latest: BTreeMap<String, u32> = BTreeMap::new();
    for row in problems.fields("transcripts", gene.transcripts) {
        let Some(reference) = problems.field("transcripts.reference", Some(row.reference)) else {
            continue;
        };
        let Some((base, version)) = reference.as_str().split_once('.') else {
            continue;
        };
        let Ok(version) = version.parse::<u32>() else {
            continue;
        };
        if base.starts_with("NM_") && base != own {
            let entry = latest.entry(base.to_owned()).or_insert(version);
            *entry = (*entry).max(version);
        }
    }
    let transcripts: Vec<String> = latest
        .into_iter()
        .map(|(base, version)| format!("{base}.{version}"))
        .collect();
    (!transcripts.is_empty()).then_some((symbol, transcripts))
}
