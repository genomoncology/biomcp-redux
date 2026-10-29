//! Historical fragments retain their original spellings and unverified association.
use crate::{
    Mapped,
    mapped::Problems,
    raw::{Field, OneOrMany},
};
use biodata::{Build, FiniteNumber, Text};

use biomcp_fetch::{Arg, BadReply, Leg, PathSegment, Reply, RequestError};
use biomcp_types::{CosmicCoordinates, CosmicIdentity, CosmicRecord, Source, VariantCosmic};
use serde::Deserialize;
/// Read one exact aggregator record, without asserting its fragment's allele identity.
pub fn get(leg: &Leg<'_>, id: &str, build: Build) -> Result<Mapped<VariantCosmic>, RequestError> {
    let endpoint = super::VARIANT
        .clone()
        .with_segments(&[PathSegment::new(id)?]);
    let assembly = if build == Build::GRCh38 {
        "hg38"
    } else {
        "hg19"
    };
    let (raw, _) = leg.json_received_checked::<Record>(
        &endpoint,
        &[
            ("fields", Arg::Text("_id,cosmic")),
            ("assembly", Arg::Text(assembly)),
        ],
        |record| {
            if record.id.as_str() == id {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    let mut problems = Problems {
        source: Source::MyVariant,
        record: Some(raw.id.clone()),
        list: Vec::new(),
    };
    let mut records = Vec::new();
    let rows = problems.field("cosmic", raw.cosmic);
    for raw_row in rows.into_iter().flat_map(OneOrMany::values) {
        let Some(row) = problems.field("cosmic.cosmic_id", Some(raw_row)) else {
            continue;
        };
        let tumor_site = problems
            .field("cosmic.tumor_site", row.tumor_site)
            .into_iter()
            .flat_map(OneOrMany::values)
            .filter_map(|raw| problems.field("cosmic.tumor_site", Some(raw)))
            .collect();
        records.push(CosmicRecord {
            myvariant_record: raw.id.clone(),
            identity: CosmicIdentity::Unverified,
            cosmic_id: row.cosmic_id,
            reference: problems.field("cosmic.ref", row.reference),
            alternate: problems.field("cosmic.alt", row.alt),
            chromosome: problems.field("cosmic.chrom", row.chrom),
            hg19: problems.field("cosmic.hg19", row.hg19),
            mut_nt: problems.field("cosmic.mut_nt", row.mut_nt),
            mut_freq: problems.field("cosmic.mut_freq", row.mut_freq),
            tumor_site,
        });
    }
    let components = if records.is_empty() {
        Vec::new()
    } else {
        vec![super::metadata::cosmic(leg)?]
    };
    Ok(Mapped {
        value: VariantCosmic {
            total: records.len() as u64,
            next: None,
            records: records.into(),
            components: components.into(),
        },
        problems: problems.list,
    })
}
#[derive(Deserialize)]
struct Record {
    #[serde(rename = "_id")]
    id: Text,
    cosmic: Option<Field<OneOrMany<Field<Row>>>>,
}
impl Reply for Record {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .id
            .as_str()
            .parse::<biomcp_types::VariantInput>()
            .is_ok()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Deserialize)]
struct Row {
    cosmic_id: biodata::CosmicId,
    #[serde(rename = "ref")]
    reference: Option<Field<Text>>,
    alt: Option<Field<Text>>,
    chrom: Option<Field<Text>>,
    hg19: Option<Field<CosmicCoordinates>>,
    mut_nt: Option<Field<Text>>,
    mut_freq: Option<Field<FiniteNumber>>,
    tumor_site: Option<Field<OneOrMany<Field<Text>>>>,
}
