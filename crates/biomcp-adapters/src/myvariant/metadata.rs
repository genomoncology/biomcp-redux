//! Component provenance from the aggregator, without a blanket component license.
use crate::raw::Field;
use biodata::Text;
use biomcp_fetch::{BadReply, Endpoint, Freshness, Leg, Reply, RequestError};
use biomcp_types::{ComponentUpload, VariantComponent};
use serde::Deserialize;
use std::collections::BTreeMap;
const METADATA: Endpoint = super::SPEC.get("/metadata").fresh(Freshness::Days(1));
pub(super) fn cosmic(leg: &Leg<'_>) -> Result<VariantComponent, RequestError> {
    components(leg, &[super::components::COSMIC])?
        .into_iter()
        .next()
        .ok_or_else(super::leg_bad)
}
pub(super) fn components(
    leg: &Leg<'_>,
    selected: &[super::components::Component],
) -> Result<Vec<VariantComponent>, RequestError> {
    let raw = leg.json::<Metadata>(&METADATA, &[])?;
    selected
        .iter()
        .map(|declaration| {
            let component = match raw.src.get(declaration.name) {
                Some(Field::Value(component)) => component,
                _ => return Err(super::leg_bad()),
            };
            let (uploaded, uploads) = match &component.upload_date {
                Some(Uploads::One(value)) => (Some(value.clone()), Vec::new()),
                Some(Uploads::Many(values)) => (
                    None,
                    values
                        .iter()
                        .map(|(name, date)| {
                            Ok(ComponentUpload {
                                name: Text::new(name).map_err(|_| super::leg_bad())?,
                                date: date.clone().into(),
                            })
                        })
                        .collect::<Result<Vec<_>, RequestError>>()?,
                ),
                None => (None, Vec::new()),
            };
            Ok(VariantComponent {
                terms: super::components::label(declaration.terms)?,
                name: Text::new(declaration.name).map_err(|_| super::leg_bad())?,
                version: component.version.clone(),
                license: component.license_url.clone(),
                aggregator_build: raw.build_version.clone(),
                aggregator_built: raw.build_date.clone(),
                uploaded,
                uploads: uploads.into(),
                note: component.note.clone(),
            })
        })
        .collect()
}
#[derive(Deserialize)]
struct Metadata {
    build_version: Text,
    build_date: Text,
    src: BTreeMap<String, Field<Component>>,
}
#[derive(Deserialize)]
struct Component {
    version: Option<Text>,
    license_url: Text,
    upload_date: Option<Uploads>,
    note: Option<Text>,
}
#[derive(Deserialize)]
#[serde(untagged)]
enum Uploads {
    One(Text),
    Many(BTreeMap<String, Text>),
}
impl Reply for Metadata {
    fn check(&self) -> Result<(), BadReply> {
        if self.src.is_empty() {
            Err(BadReply)
        } else {
            Ok(())
        }
    }
}
