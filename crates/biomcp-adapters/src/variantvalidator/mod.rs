//! Explicit VariantValidator GET reads; warning envelopes never resolve identity.
use crate::quotas;
use biodata::{Build, Representation, Text, Variant};

use biomcp_fetch::{
    Arg, BadReply, Endpoint, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use biomcp_types::{Source, SourceRecord, VariantHeader};
use serde::Deserialize;
mod normalization;
use std::{collections::BTreeMap, time::Duration};
/// Service terms are independent of the implementation's software license.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "VariantValidator service output terms require review",
        url: "https://variantvalidator.org/help/about/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::VariantValidator,
    base: "https://rest.variantvalidator.org/VariantValidator",
    key: KeyUse::None,
    quota: &quotas::VARIANT_VALIDATOR,
    deadline: Duration::from_secs(15),
};
const VALIDATE: Endpoint = SPEC
    .get("/variantvalidator")
    .json_rejections()
    .fresh(biomcp_fetch::Freshness::Days(1));
/// Explicit service response, without a local consensus or preferred transcript.
#[derive(Debug)]
pub struct Resolution {
    /// Warning flag refuses resolution even on HTTP 200.
    pub rejected: bool,
    /// Every source identity returned by the selected service.
    pub candidates: Vec<VariantHeader>,
}
#[derive(Deserialize)]
struct Response {
    flag: Text,
    metadata: BTreeMap<String, String>,
    #[serde(flatten, deserialize_with = "ordered_variants")]
    variants: Vec<(String, RawVariant)>,
}
#[derive(Deserialize)]
struct RawVariant {
    #[serde(default)]
    validation_warnings: Vec<String>,
    hgvs_lrg_transcript_variant: Option<String>,
    hgvs_lrg_variant: Option<String>,
    hgvs_refseqgene_variant: Option<String>,
    hgvs_predicted_protein_consequence: Option<BTreeMap<String, String>>,
    alt_genomic_loci: Option<Vec<BTreeMap<String, Locus>>>,
    submitted_variant: String,
    hgvs_transcript_variant: String,
    primary_assembly_loci: Option<BTreeMap<String, Locus>>,
}
#[derive(Deserialize)]
struct Locus {
    hgvs_genomic_description: Text,
}
impl Reply for Response {
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if (200..300).contains(&status)
            || matches!(status, 400 | 422) && self.flag.as_str() == "warning"
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
    fn check(&self) -> Result<(), BadReply> {
        if self.variants.is_empty()
            || self.variants.len() > 1000
            || !self.metadata.contains_key("variantvalidator_version")
        {
            return Err(BadReply);
        }
        if self.flag.as_str() == "warning"
            || self
                .variants
                .iter()
                .map(|(_, value)| value)
                .all(|v| !v.hgvs_transcript_variant.is_empty())
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
/// Preserve caller assembly, unchanged input and every service-returned identity.
fn fetch(leg: &Leg<'_>, input: &str, build: Build) -> Result<Response, RequestError> {
    let build = build.to_string();
    let endpoint = VALIDATE.clone().with_segments(&[
        PathSegment::new(&build)?,
        PathSegment::new(input)?,
        PathSegment::new("all")?,
    ]);
    let raw = leg.json_checked_admission::<Response>(
        &endpoint,
        &[("content-type", Arg::Text("application/json"))],
        |reply| {
            if reply
                .variants
                .iter()
                .all(|(_, variant)| variant.submitted_variant.as_str() == input)
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
        |reply| reply.flag.as_str() != "warning",
    )?;
    Ok(raw)
}
/// Preserve caller assembly, unchanged input and every service-returned identity.
pub fn resolve(leg: &Leg<'_>, input: &str, build: Build) -> Result<Resolution, RequestError> {
    let raw = fetch(leg, input, build)?;
    if raw.flag.as_str() == "warning" {
        return Ok(Resolution {
            rejected: true,
            candidates: Vec::new(),
        });
    }
    let mut candidates = Vec::new();
    for (_, v) in raw.variants {
        let key = Text::new(&v.hgvs_transcript_variant).map_err(|_| bad())?;
        let expression = biodata::Expression::hgvs(key.clone()).map_err(|_| bad())?;
        let mut representations = vec![Representation {
            source: Source::VariantValidator.text(),
            expression,
            build: None,
            allele: None,
        }];
        for (assembly, locus) in v.primary_assembly_loci.into_iter().flatten() {
            let expression =
                biodata::Expression::hgvs(locus.hgvs_genomic_description).map_err(|_| bad())?;
            let source_build = match assembly.as_str() {
                "grch37" | "hg19" => Some(Build::GRCh37),
                "grch38" | "hg38" => Some(Build::GRCh38),
                _ => None,
            };
            representations.push(Representation {
                source: Source::VariantValidator.text(),
                expression,
                build: source_build,
                allele: None,
            });
        }
        candidates.push(VariantHeader {
            identity: Variant {
                ids: Box::default(),
                representations: representations.into(),
                genes: Box::default(),
            },
            source: Source::VariantValidator,
            record: SourceRecord { id: key, url: None },
            myvariant: None,
            requested: None,
            sections: None,
            component_terms: Box::default(),
        });
    }
    Ok(Resolution {
        rejected: false,
        candidates,
    })
}
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::VariantValidator,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}

fn ordered_variants<'de, D: serde::Deserializer<'de>>(
    de: D,
) -> Result<Vec<(String, RawVariant)>, D::Error> {
    struct Ordered;
    impl<'de> serde::de::Visitor<'de> for Ordered {
        type Value = Vec<(String, RawVariant)>;
        fn expecting(&self, out: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            out.write_str("source candidate map")
        }
        fn visit_map<M: serde::de::MapAccess<'de>>(
            self,
            mut map: M,
        ) -> Result<Self::Value, M::Error> {
            let mut rows = Vec::new();
            while let Some(row) = map.next_entry()? {
                rows.push(row);
            }
            Ok(rows)
        }
    }
    de.deserialize_map(Ordered)
}

/// Native service descriptions, warnings and version metadata.
pub use normalization::normalize;
