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
    annotations: Option<Field<Annotations>>,
}
#[derive(Deserialize)]
struct Annotations {
    mane_plus_clinical: Option<Field<bool>>,
}
impl Reply for Genes {
    fn check(&self) -> Result<(), BadReply> {
        if self.0.is_empty() || self.0.len() > 10 {
            return Err(BadReply);
        }
        Ok(())
    }
}

/// The gene-symbol description and the gene's other NM_ transcripts to try it on.
#[derive(Debug)]
pub struct FitQuery {
    /// `SYMBOL:c.change`, which VariantValidator validates on each selected transcript.
    query: String,
    /// The latest version of each other NM_ transcript: MANE Plus Clinical first, then by
    /// accession number, so older short accessions such as NM_172114 come early.
    pub transcripts: Vec<String>,
}

/// The input's gene and its other RefSeq transcripts, from VariantValidator.
/// Only a RefSeq NM_ c. description is tried; anything else, or no other transcript, is none.
pub fn fit_query(
    leg: &Leg<'_>,
    input: &str,
    build: Build,
) -> Result<Mapped<Option<FitQuery>>, RequestError> {
    let mut problems = Problems {
        source: Source::VariantValidator,
        record: None,
        list: Vec::new(),
    };
    let mut found = None;
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
        found = gene(genes, accession, &mut problems).map(|(symbol, transcripts)| FitQuery {
            query: format!("{symbol}:{body}"),
            transcripts,
        });
    }
    Ok(Mapped {
        refusals: crate::Refusal::None,
        value: found,
        problems: problems.list,
    })
}

/// The transcripts in `group` where VariantValidator accepts the same c. change.
pub fn transcript_fits(
    leg: &Leg<'_>,
    fit: &FitQuery,
    group: &[String],
    build: Build,
) -> Result<Mapped<Vec<TranscriptFit>>, RequestError> {
    let mut problems = Problems {
        source: Source::VariantValidator,
        record: None,
        list: Vec::new(),
    };
    let mut refusals = crate::Refusal::None;
    let mut fits = Vec::new();
    let mut raw = fetch_on(leg, &fit.query, build, &group.join("|"))?;
    // The transcripts where the base does not fit come back as warning records.
    raw.variants
        .retain(|(key, _)| !key.starts_with("validation_warning"));
    if raw.flag.as_str() != "warning" {
        let key = build.to_string().to_ascii_lowercase();
        for (_, row) in raw.records(&fit.query, &mut problems, &mut refusals) {
            let genomic = problems
                .field("primary_assembly_loci", row.primary_assembly_loci)
                .and_then(|mut loci| loci.remove(&key))
                .and_then(|locus| problems.field(&key, Some(locus)))
                .and_then(|locus| problems.field(&key, Some(locus.hgvs_genomic_description)))
                .map(|value| value.to_string().into());
            fits.push(TranscriptFit {
                hgvs: row.hgvs_transcript_variant.into(),
                genomic,
            });
        }
    }
    Ok(Mapped {
        refusals,
        value: fits,
        problems: problems.list,
    })
}

/// Whether `fit_query` would ask anything: a RefSeq NM_ c. description.
pub fn fits_apply(input: &str) -> bool {
    parts(input).is_some()
}

fn parts(input: &str) -> Option<(&str, &str)> {
    let (reference, body) = input.split_once(':')?;
    let accession = reference
        .split_once('(')
        .map_or(reference, |(outer, _)| outer);
    (accession.starts_with("NM_") && body.starts_with("c.")).then_some((accession, body))
}

/// The gene symbol and the latest version of each of its other NM_ transcripts, in the
/// order of [`FitQuery::transcripts`].
fn gene(genes: Genes, input: &str, problems: &mut Problems) -> Option<(Text, Vec<String>)> {
    let gene = genes.0.into_iter().find_map(Field::value)?;
    let symbol = problems.field("current_symbol", gene.current_symbol)?;
    if symbol.as_str().parse::<biodata::GeneSymbol>().is_err() {
        problems.required::<()>("current_symbol", None);
        return None;
    }
    let own = input.split_once('.').map_or(input, |(base, _)| base);
    // Each accession's latest version, and whether any version is MANE Plus Clinical.
    let mut latest: BTreeMap<String, (u32, bool)> = BTreeMap::new();
    for row in problems.fields("transcripts", gene.transcripts) {
        let Some(reference) = problems.field("transcripts.reference", Some(row.reference)) else {
            continue;
        };
        let Some((base, version)) = reference.as_str().split_once('.') else {
            continue;
        };
        let Some(version) =
            problems.optional("transcripts.reference", Some(version.parse::<u32>()))
        else {
            continue;
        };
        let plus = problems
            .field("transcripts.annotations", row.annotations)
            .and_then(|annotations| {
                problems.field(
                    "transcripts.annotations.mane_plus_clinical",
                    annotations.mane_plus_clinical,
                )
            })
            .unwrap_or(false);
        if base.starts_with("NM_") && base != own {
            let entry = latest.entry(base.to_owned()).or_insert((version, plus));
            *entry = (entry.0.max(version), entry.1 || plus);
        }
    }
    let mut ordered: Vec<(bool, u64, String)> = latest
        .into_iter()
        .map(|(base, (version, plus))| {
            let number = base
                .trim_start_matches("NM_")
                .parse::<u64>()
                .unwrap_or(u64::MAX);
            (!plus, number, format!("{base}.{version}"))
        })
        .collect();
    ordered.sort();
    let transcripts: Vec<String> = ordered.into_iter().map(|(_, _, name)| name).collect();
    (!transcripts.is_empty()).then_some((symbol, transcripts))
}
