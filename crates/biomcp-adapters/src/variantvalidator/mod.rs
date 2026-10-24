//! Explicit VariantValidator GET reads; warning envelopes never resolve identity.
use crate::quotas;
use biodata::{Build, Representation, Source, SourceRecord, Text, Variant, VariantHeader};
use biomcp_fetch::{
    Arg, BadReply, Endpoint, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec,
};
use serde::Deserialize;
use std::{collections::BTreeMap, time::Duration};
/// Service terms are independent of the implementation's software license.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "VariantValidator service output terms require review",
        url: "https://variantvalidator.org/",
        usage: biodata::Use::Restricted,
    },
    source: Source::VariantValidator,
    base: "https://rest.variantvalidator.org/VariantValidator",
    key: KeyUse::None,
    quota: &quotas::VARIANT_VALIDATOR,
    deadline: Duration::from_secs(15),
};
const VALIDATE: Endpoint = SPEC.get("/variantvalidator").json_rejections();
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
    #[serde(rename = "metadata")]
    _metadata: Metadata,
    #[serde(flatten)]
    variants: BTreeMap<String, RawVariant>,
}
#[derive(Deserialize)]
struct Metadata {
    #[serde(rename = "variantvalidator_version")]
    _version: Text,
}
#[derive(Deserialize)]
struct RawVariant {
    submitted_variant: Text,
    hgvs_transcript_variant: String,
    #[serde(default)]
    primary_assembly_loci: BTreeMap<String, Locus>,
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
        if self.variants.is_empty() || self.variants.len() > 1000 {
            return Err(BadReply);
        }
        if self.flag.as_str() == "warning"
            || self
                .variants
                .values()
                .all(|v| !v.hgvs_transcript_variant.is_empty())
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
/// Preserve caller assembly, unchanged input and every service-returned identity.
pub fn resolve(leg: &Leg<'_>, input: &str, build: Build) -> Result<Resolution, RequestError> {
    let build = build.to_string();
    let endpoint = VALIDATE.clone().with_segments(&[
        PathSegment::new(&build)?,
        PathSegment::new(input)?,
        PathSegment::new("all")?,
    ]);
    let raw = leg.json::<Response>(
        &endpoint,
        &[("content-type", Arg::Text("application/json"))],
    )?;
    if raw
        .variants
        .values()
        .any(|v| v.submitted_variant.as_str() != input)
    {
        return Err(bad());
    }
    if raw.flag.as_str() == "warning" {
        return Ok(Resolution {
            rejected: true,
            candidates: Vec::new(),
        });
    }
    let mut candidates = Vec::new();
    for (_, v) in raw.variants {
        let key = Text::new(&v.hgvs_transcript_variant).map_err(|_| bad())?;
        let mut representations = Vec::new();
        for (assembly, locus) in v.primary_assembly_loci {
            let expression =
                biodata::Expression::hgvs(locus.hgvs_genomic_description).map_err(|_| bad())?;
            let source_build = match assembly.as_str() {
                "grch37" | "hg19" => Some(Build::Grch37),
                "grch38" | "hg38" => Some(Build::Grch38),
                _ => None,
            };
            representations.push(Representation {
                source: Source::VariantValidator,
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
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
