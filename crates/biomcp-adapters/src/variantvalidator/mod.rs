//! Explicit VariantValidator GET reads; warning envelopes never resolve identity.
use crate::quotas;
use biodata::{Build, Source, Text, VariantHeader, VariantIdentity, VariantRepresentation};
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
    metadata: Metadata,
    #[serde(flatten)]
    variants: BTreeMap<String, Variant>,
}
#[derive(Deserialize)]
struct Metadata {
    variantvalidator_version: Text,
}
#[derive(Deserialize)]
struct Variant {
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
            let reference = locus
                .hgvs_genomic_description
                .as_str()
                .split_once(':')
                .and_then(|(r, _)| Text::new(r).ok())
                .ok_or_else(bad)?;
            representations.push(VariantRepresentation {
                source: Source::VariantValidator,
                notation: locus.hgvs_genomic_description,
                reference,
                assembly: Text::new(&assembly).ok(),
                coordinate: None,
                reference_allele: None,
                alternate_allele: None,
            });
        }
        let facts = vec![biodata::VariantFact {
            source: Text::new("variantvalidator").map_err(|_| bad())?,
            field: Text::new("version").map_err(|_| bad())?,
            value: raw.metadata.variantvalidator_version.clone(),
        }];
        candidates.push(VariantHeader {
            identity: VariantIdentity {
                source: Source::VariantValidator,
                key,
                aliases: Box::default(),
                representations: representations.into(),
            },
            facts: facts.into(),
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
