//! Explicit Mutalyzer GET resolution; no inferred transcript or local correction.
use crate::quotas;
use biodata::{Representation, Text, Variant};

use biomcp_fetch::{BadReply, Endpoint, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec};
use biomcp_types::{Source, SourceRecord, VariantHeader};
use serde::Deserialize;
mod normalization;
use std::time::Duration;
/// Service rights remain distinct from the software license.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    terms: biomcp_fetch::Terms {
        name: "Mutalyzer service output terms require review",
        url: "https://mutalyzer.nl/",
        usage: biomcp_types::Use::Restricted,
    },
    source: Source::Mutalyzer,
    base: "https://mutalyzer.nl/api",
    key: KeyUse::None,
    quota: &quotas::MUTALYZER,
    deadline: Duration::from_secs(15),
};
const NORMALIZE: Endpoint = SPEC
    .get("/normalize")
    .json_rejections()
    .fresh(biomcp_fetch::Freshness::Days(1));
/// A source's exact outcome, never a local normalization.
#[derive(Debug)]
pub enum Resolution {
    /// Source supplied a normalized identity.
    Found(VariantHeader),
    /// Source validation codes reject the supplied input.
    Rejected(Vec<Text>),
}
#[derive(Deserialize)]
struct Normalized {
    #[serde(default)]
    custom: Option<Box<Normalized>>,
    corrected_description: Option<String>,
    protein: Option<Description>,
    rna: Option<Description>,
    equivalent_descriptions: Option<std::collections::BTreeMap<String, Vec<Description>>>,
    back_translated_descriptions: Option<Vec<String>>,
    infos: Option<Vec<Message>>,
    warnings: Option<Vec<Message>>,
    version: Option<String>,
    input_description: Option<String>,
    normalized_description: Option<String>,
    errors: Option<Vec<Message>>,
}
#[derive(Deserialize)]
struct Message {
    code: Text,
    details: Option<String>,
}
#[derive(Deserialize)]
struct Description {
    description: String,
}
impl Normalized {
    fn payload(&self) -> &Self {
        self.custom.as_deref().unwrap_or(self)
    }
    fn errors(&self) -> &[Message] {
        self.payload().errors.as_deref().unwrap_or(&[])
    }
}
impl Reply for Normalized {
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if (200..300).contains(&status) || matches!(status, 400 | 422) && !self.errors().is_empty()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
    fn check(&self) -> Result<(), BadReply> {
        if self.errors().len() > 1000
            || self.errors().iter().any(|error| {
                let code = error.code.as_str();
                code.len() > 128 || !code.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
            })
        {
            return Err(BadReply);
        }
        if !self.errors().is_empty()
            || self.input_description.is_some() && self.normalized_description.is_some()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
/// Route only the explicitly selected service, preserving the supplied description.
fn fetch(leg: &Leg<'_>, input: &str) -> Result<Normalized, RequestError> {
    let endpoint = NORMALIZE.clone().with_segments(&[PathSegment::new(input)?]);
    let mut raw = leg.json_checked_admission::<Normalized>(
        &endpoint,
        &[],
        |reply| {
            if (!reply.errors().is_empty()
                && reply
                    .payload()
                    .input_description
                    .as_ref()
                    .is_none_or(|description| description.as_str() == input))
                || reply.payload().input_description.as_deref() == Some(input)
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
        |reply| reply.errors().is_empty(),
    )?;
    Ok(if let Some(custom) = raw.custom.take() {
        *custom
    } else {
        raw
    })
}
/// Route only the explicitly selected service, preserving the supplied description.
pub fn resolve(leg: &Leg<'_>, input: &str) -> Result<Resolution, RequestError> {
    let raw = fetch(leg, input)?;
    if !raw.errors().is_empty() {
        return Ok(Resolution::Rejected(
            raw.errors.into_iter().flatten().map(|e| e.code).collect(),
        ));
    }
    let notation = Text::new(&raw.normalized_description.ok_or_else(bad)?).map_err(|_| bad())?;
    let expression = biodata::Expression::hgvs(notation.clone()).map_err(|_| bad())?;
    Ok(Resolution::Found(VariantHeader {
        identity: Variant {
            ids: Box::default(),
            representations: vec![Representation {
                source: Source::Mutalyzer.text(),
                expression,
                build: None,
                allele: None,
            }]
            .into(),
            genes: Box::default(),
        },
        source: Source::Mutalyzer,
        record: SourceRecord {
            id: notation,
            url: None,
        },
        transcripts: Box::default(),
        myvariant: None,
        requested: None,
        sections: None,
        component_terms: Box::default(),
    }))
}

fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Mutalyzer,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}

/// Source normalization descriptions, mappings and native messages.
pub use normalization::normalize;
