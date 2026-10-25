//! Native min and base tiers; complete bounded collections stay in the shared cache.
use super::SPEC;
use biodata::{PgxAnnotationDetail, PgxAnnotationHeader, PgxAnnotationReference};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, RequestError};
use biomcp_types::{PgxAnnotationGet, PgxAnnotationKind, PgxGet};
use serde::Deserialize;
/// Bounded whole native collection with no claimed server-wide total.
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Native<T> {
    /// Complete native result.
    pub data: T,
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
) -> Result<Vec<T>, RequestError> {
    let endpoint = SPEC
        .get(path)
        .fresh(Freshness::Days(1))
        .bytes(4 * 1024 * 1024);
    let body: Native<Vec<T>> = leg.json(
        &endpoint,
        &[
            (key, Arg::Text(q.gene().as_ref())),
            ("view", Arg::Text("min")),
        ],
    )?;
    Ok(body.data)
}
/// Clinical min preserves full locations, evidence labels and relationship arrays.
pub fn clinical(leg: &Leg<'_>, q: &PgxGet) -> Result<Vec<PgxAnnotationHeader>, RequestError> {
    collection(leg, q, "/data/clinicalAnnotation", "location.genes.symbol")
}
/// Reference collections remain independent; neither supplies full guidance.
pub fn references(leg: &Leg<'_>, q: &PgxGet) -> Result<Vec<PgxAnnotationReference>, RequestError> {
    let path = match q.kind() {
        Some(PgxAnnotationKind::Guideline) => "/data/guidelineAnnotation",
        Some(PgxAnnotationKind::Label) => "/data/labelAnnotation",
        _ => return Err(biomcp_fetch::InputError::Serialization.into()),
    };
    collection(leg, q, path, "relatedGenes.symbol")
}
/// Check exact returned accession on every read before cache admission.
pub fn annotation(
    leg: &Leg<'_>,
    q: &PgxAnnotationGet,
) -> Result<PgxAnnotationDetail, RequestError> {
    let endpoint = SPEC
        .get("/data/clinicalAnnotation")
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
