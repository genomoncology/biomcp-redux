//! Explicit Mutalyzer GET resolution; no inferred transcript or local correction.
use crate::quotas;
use biodata::{Representation, Source, SourceRecord, Text, Variant, VariantHeader};
use biomcp_fetch::{BadReply, Endpoint, KeyUse, Leg, PathSegment, Reply, RequestError, SourceSpec};
use serde::Deserialize;
use std::time::Duration;
/// Service rights remain distinct from the software license.
pub static SPEC: SourceSpec = SourceSpec {
    terms: biomcp_fetch::Terms {
        name: "Mutalyzer service output terms require review",
        url: "https://mutalyzer.nl/",
        usage: biodata::Use::Restricted,
    },
    source: Source::Mutalyzer,
    base: "https://mutalyzer.nl/api",
    key: KeyUse::None,
    quota: &quotas::MUTALYZER,
    deadline: Duration::from_secs(15),
};
const NORMALIZE: Endpoint = SPEC.get("/normalize").json_rejections();
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
    input_description: Option<Text>,
    normalized_description: Option<Text>,
    #[serde(default)]
    errors: Vec<Message>,
}
#[derive(Deserialize)]
struct Message {
    code: Text,
}
impl Reply for Normalized {
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if (200..300).contains(&status) || matches!(status, 400 | 422) && !self.errors.is_empty() {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
    fn check(&self) -> Result<(), BadReply> {
        if self.errors.len() > 1000
            || self.errors.iter().any(|error| {
                let code = error.code.as_str();
                code.len() > 128 || !code.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_')
            })
        {
            return Err(BadReply);
        }
        if !self.errors.is_empty()
            || self.input_description.is_some() && self.normalized_description.is_some()
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
/// Route only the explicitly selected service, preserving the supplied description.
pub fn resolve(leg: &Leg<'_>, input: &str) -> Result<Resolution, RequestError> {
    let endpoint = NORMALIZE.clone().with_segments(&[PathSegment::new(input)?]);
    let (raw, _) = leg.json_received_checked::<Normalized>(&endpoint, &[], |reply| {
        if !reply.errors.is_empty() || reply.input_description.as_ref().map(Text::as_str) == Some(input) {
            Ok(())
        } else { Err(BadReply) }
    })?;
    if !raw.errors.is_empty() {
        return Ok(Resolution::Rejected(
            raw.errors.into_iter().map(|e| e.code).collect(),
        ));
    }
    let notation = raw.normalized_description.ok_or_else(bad)?;
    let expression = biodata::Expression::hgvs(notation.clone()).map_err(|_| bad())?;
    Ok(Resolution::Found(VariantHeader {
        identity: Variant {
            ids: Box::default(),
            representations: vec![Representation {
                source: Source::Mutalyzer,
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
        myvariant: None,
    }))
}

fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: Source::Mutalyzer,
        reason: biodata::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
