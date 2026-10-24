//! Provider contacts and sites mapped once into source-preserving values.
use biodata::{TrialContact, TrialCoordinates, TrialLocation, TrialOfficial};
use serde::Deserialize;
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ContactsLocations {
    central_contacts: Option<Box<[Contact]>>,
    overall_officials: Option<Box<[Official]>>,
    locations: Option<Box<[Location]>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contact {
    name: Option<Box<str>>,
    role: Option<biodata::TrialContactRole>,
    phone: Option<Box<str>>,
    phone_ext: Option<Box<str>>,
    email: Option<Box<str>>,
}
#[derive(Deserialize)]
struct Official {
    name: Option<Box<str>>,
    affiliation: Option<Box<str>>,
    role: Option<biodata::TrialOfficialRole>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location {
    facility: Option<Box<str>>,
    city: Option<Box<str>>,
    state: Option<Box<str>>,
    zip: Option<Box<str>>,
    country: Option<Box<str>>,
    status: Option<biodata::TrialSiteStatus>,
    // Coordinate aliases belong to this provider's shape, not the shared domain value.
    geo_point: Option<Point>,
    contacts: Option<Box<[Contact]>>,
}
type SiteRows = (
    Box<[TrialLocation]>,
    Box<[TrialContact]>,
    Box<[TrialOfficial]>,
);
impl ContactsLocations {
    pub(super) fn rows(self) -> SiteRows {
        let locations = self
            .locations
            .unwrap_or_default()
            .into_vec()
            .into_iter()
            .map(Location::mapped)
            .collect();
        (
            locations,
            self.central_contacts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .map(Contact::mapped)
                .collect(),
            self.overall_officials
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .map(|v| TrialOfficial {
                    name: v.name,
                    affiliation: v.affiliation,
                    role: v.role,
                })
                .collect(),
        )
    }
}
impl Contact {
    fn mapped(self) -> TrialContact {
        TrialContact {
            name: self.name,
            role: self.role,
            phone: self.phone,
            phone_ext: self.phone_ext,
            email: self.email,
        }
    }
}
impl Location {
    fn mapped(self) -> TrialLocation {
        TrialLocation {
            facility: self.facility,
            city: self.city,
            state: self.state,
            zip: self.zip,
            country: self.country,
            status: self.status,
            coordinates: self.geo_point.map(|v| v.0),
            contacts: self
                .contacts
                .unwrap_or_default()
                .into_vec()
                .into_iter()
                .map(Contact::mapped)
                .collect(),
        }
    }
}

#[derive(Deserialize)]
#[serde(try_from = "RawPoint")]
struct Point(TrialCoordinates);
#[derive(Deserialize)]
struct RawPoint {
    lat: biodata::FiniteNumber,
    lon: biodata::FiniteNumber,
}
impl TryFrom<RawPoint> for Point {
    type Error = &'static str;
    fn try_from(v: RawPoint) -> Result<Self, Self::Error> {
        TrialCoordinates::new(v.lat, v.lon).map(Self)
    }
}
