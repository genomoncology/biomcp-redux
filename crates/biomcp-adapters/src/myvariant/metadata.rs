//! Component provenance from the aggregator, without a blanket component license.
use biodata::{Text, VariantComponent};
use biomcp_fetch::{BadReply, Endpoint, Freshness, Leg, Reply, RequestError};
use serde::Deserialize;
const METADATA: Endpoint = super::SPEC.get("/metadata").fresh(Freshness::Days(1));
/// Read the historical COSMIC snapshot's published metadata.
pub(super) fn cosmic(leg: &Leg<'_>) -> Result<VariantComponent, RequestError> {
    let raw = leg.json::<Metadata>(&METADATA, &[])?;
    let component = raw.src.cosmic;
    Ok(VariantComponent {
        terms: super::components::label(super::components::COSMIC.terms)?,
        name: Text::new("cosmic").map_err(|_| super::leg_bad())?,
        version: component.version,
        license: component.license_url,
        aggregator_build: raw.build_version,
        aggregator_built: raw.build_date,
        uploaded: component.upload_date,
        note: component.note,
    })
}
#[derive(Deserialize)]
struct Metadata {
    build_version: Text,
    build_date: Text,
    src: Sources,
}
#[derive(Deserialize)]
struct Sources {
    cosmic: Component,
}
#[derive(Deserialize)]
struct Component {
    version: Option<Text>,
    license_url: Text,
    upload_date: Option<Text>,
    note: Option<Text>,
}
impl Reply for Metadata {
    fn check(&self) -> Result<(), BadReply> {
        if self.src.cosmic.version.is_none() || self.src.cosmic.note.is_none() {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
