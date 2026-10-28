//! Native min and base tiers; complete bounded collections stay in the shared cache.
use super::SPEC;
use biodata::{PgxAnnotationDetail, PgxAnnotationHeader, PgxAnnotationReference};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, RequestError};
use biomcp_types::{PgxAnnotationGet, PgxAnnotationKind, PgxGet};
use serde::Deserialize;
/// Bounded whole native collection with no claimed server-wide total.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields, bound(deserialize = "T: Deserialize<'de>"))]
struct Native<T> {
    /// Complete native result.
    #[serde(deserialize_with = "crate::strict_input::deserialize")]
    data: T,
    status: Box<str>,
}
impl<T: serde::de::DeserializeOwned> Reply for Native<T> {
    fn check(&self) -> Result<(), BadReply> {
        if self.status.as_ref() == "success" {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
fn collection<T: serde::de::DeserializeOwned>(
    leg: &Leg<'_>,
    q: &PgxGet,
    path: &'static str,
    key: &'static str,
    check: impl Fn(&[T]) -> bool,
) -> Result<Vec<T>, RequestError> {
    let endpoint = SPEC
        .get(path)
        .fresh(Freshness::Days(1))
        .bytes(4 * 1024 * 1024);
    let gene = q.gene().to_string();
    let body: Native<Vec<T>> = leg.json_checked_admission(
        &endpoint,
        &[(key, Arg::Text(&gene)), ("view", Arg::Text("min"))],
        |body: &Native<Vec<T>>| {
            if check(&body.data) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
        |_| true,
    )?;
    Ok(body.data)
}
/// Clinical min preserves full locations, evidence labels and relationship arrays.
pub fn clinical(leg: &Leg<'_>, q: &PgxGet) -> Result<Vec<PgxAnnotationHeader>, RequestError> {
    let symbol = q.gene().to_string();
    collection(
        leg,
        q,
        super::CLINICAL_PATH,
        "location.genes.symbol",
        |rows: &[PgxAnnotationHeader]| {
            rows.iter().all(|row| {
                row.location
                    .genes
                    .iter()
                    .any(|gene| gene.symbol.as_deref() == Some(symbol.as_str()))
            })
        },
    )
}
/// Reference collections remain independent; neither supplies full guidance.
pub fn references(leg: &Leg<'_>, q: &PgxGet) -> Result<Vec<PgxAnnotationReference>, RequestError> {
    let path = match q.kind() {
        Some(PgxAnnotationKind::Guideline) => "/data/guidelineAnnotation",
        Some(PgxAnnotationKind::Label) => "/data/labelAnnotation",
        _ => return Err(biomcp_fetch::InputError::Serialization.into()),
    };
    collection(leg, q, path, "relatedGenes.symbol", |_| true)
}
/// Check exact returned accession on every read before cache admission.
pub fn annotation(
    leg: &Leg<'_>,
    q: &PgxAnnotationGet,
) -> Result<PgxAnnotationDetail, RequestError> {
    let endpoint = SPEC
        .get(super::CLINICAL_PATH)
        .with_path(format!("/data/clinicalAnnotation/{}", q.id()))?
        .fresh(Freshness::Days(1))
        .bytes(4 * 1024 * 1024);
    let body: Native<PgxAnnotationDetail> = leg.json_checked_admission(
        &endpoint,
        &[("view", Arg::Text("base"))],
        |body: &Native<PgxAnnotationDetail>| {
            if body.data.accession_id == *q.id() {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
        |_| true,
    )?;
    Ok(body.data)
}
