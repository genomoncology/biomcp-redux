//! Historical fragments retain their original spellings and unverified association.
use biodata::{Build, FiniteNumber, Text};

use biomcp_fetch::{Arg, BadReply, Leg, PathSegment, Reply, RequestError};
use biomcp_types::{CosmicCoordinates, CosmicIdentity, CosmicRecord, VariantCosmic};
use serde::Deserialize;
/// Read one exact aggregator record, without asserting its fragment's allele identity.
pub fn get(
    leg: &Leg<'_>,
    id: &str,
    build: Build,
) -> Result<crate::Mapped<VariantCosmic>, RequestError> {
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
    let mut records = Vec::new();
    for row in raw.cosmic.map(Rows::rows).unwrap_or_default() {
        records.push(CosmicRecord {
            myvariant_record: raw.id.clone(),
            identity: CosmicIdentity::Unverified,
            cosmic_id: row.cosmic_id,
            reference: row.reference,
            alternate: row.alt,
            chromosome: row.chrom,
            hg19: row.hg19,
            mut_nt: row.mut_nt,
            mut_freq: row.mut_freq,
            tumor_site: row.tumor_site.map(Sites::rows).unwrap_or_default().into(),
        });
    }
    let metadata = if records.is_empty() {
        crate::Mapped {
            value: Vec::new(),
            problems: Vec::new(),
        }
    } else {
        super::metadata::cosmic(leg)?
    };
    Ok(crate::Mapped {
        value: VariantCosmic {
            total: records.len() as u64,
            next: None,
            records: records.into(),
            components: metadata.value.into(),
        },
        problems: metadata.problems,
    })
}
#[derive(Deserialize)]
struct Record {
    #[serde(rename = "_id")]
    id: Text,
    cosmic: Option<Rows>,
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
#[serde(untagged)]
enum Rows {
    One(Row),
    Many(Vec<Row>),
}
impl Rows {
    fn rows(self) -> Vec<Row> {
        match self {
            Self::One(row) => vec![row],
            Self::Many(rows) => rows,
        }
    }
}
#[derive(Deserialize)]
struct Row {
    cosmic_id: biodata::CosmicId,
    #[serde(rename = "ref")]
    reference: Option<Text>,
    alt: Option<Text>,
    chrom: Option<Text>,
    hg19: Option<CosmicCoordinates>,
    mut_nt: Option<Text>,
    mut_freq: Option<FiniteNumber>,
    tumor_site: Option<Sites>,
}
#[derive(Deserialize)]
#[serde(untagged)]
enum Sites {
    One(Text),
    Many(Vec<Text>),
}
impl Sites {
    fn rows(self) -> Vec<Text> {
        match self {
            Self::One(row) => vec![row],
            Self::Many(rows) => rows,
        }
    }
}
