//! Provider contacts and sites mapped once into source-preserving values.
use biodata::{TrialContact, TrialCoordinates, TrialLocation, TrialOfficial};

use serde::Deserialize;
use crate::{mapped::Problems, raw::Field};
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub(super) struct ContactsLocations {
    central_contacts: Option<Field<Vec<Field<Contact>>>>,
    overall_officials: Option<Field<Vec<Field<Official>>>>,
    locations: Option<Field<Vec<Field<Location>>>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Contact {
    name: Option<Field<Box<str>>>,
    role: Option<Field<biodata::TrialContactRole>>,
    phone: Option<Field<Box<str>>>,
    phone_ext: Option<Field<Box<str>>>,
    email: Option<Field<Box<str>>>,
}
#[derive(Deserialize)]
struct Official {
    name: Option<Field<Box<str>>>,
    affiliation: Option<Field<Box<str>>>,
    role: Option<Field<biodata::TrialOfficialRole>>,
}
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Location {
    facility: Option<Field<Box<str>>>,
    city: Option<Field<Box<str>>>,
    state: Option<Field<Box<str>>>,
    zip: Option<Field<Box<str>>>,
    country: Option<Field<Box<str>>>,
    status: Option<Field<biodata::TrialSiteStatus>>,
    // Coordinate aliases belong to this provider's shape, not the shared domain value.
    geo_point: Option<Field<Point>>,
    contacts: Option<Field<Vec<Field<Contact>>>>,
}
type SiteRows = (
    Box<[TrialLocation]>,
    Box<[TrialContact]>,
    Box<[TrialOfficial]>,
);
impl ContactsLocations {
    pub(super) fn rows(self, problems: &mut Problems) -> SiteRows {
        let locations = problems.fields("locations", self.locations).into_vec().into_iter().map(|v| v.mapped(problems)).collect();
        let contacts = problems.fields("centralContacts", self.central_contacts).into_vec().into_iter().map(|v| v.mapped(problems)).collect();
        let officials = problems.fields("overallOfficials", self.overall_officials).into_vec().into_iter().map(|v| TrialOfficial {
            name: problems.field("overallOfficials.name", v.name),
            affiliation: problems.field("overallOfficials.affiliation", v.affiliation),
            role: problems.field("overallOfficials.role", v.role),
        }).collect();
        (locations, contacts, officials)
    }
}

impl Contact {
    fn mapped(self, problems: &mut Problems) -> TrialContact {
        TrialContact {
            name: problems.field("name", self.name),
            role: problems.field("role", self.role),
            phone: problems.field("phone", self.phone),
            phone_ext: problems.field("phoneExt", self.phone_ext),
            email: problems.field("email", self.email),
        }
    }
}
impl Location {
    fn mapped(self, problems: &mut Problems) -> TrialLocation {
        TrialLocation {
            facility: problems.field("facility", self.facility),
            city: problems.field("city", self.city),
            state: problems.field("state", self.state),
            zip: problems.field("zip", self.zip),
            country: problems.field("country", self.country),
            status: problems.field("locations.status", self.status),
            coordinates: problems.field("geoPoint", self.geo_point).map(|v| v.0),
            contacts: problems.fields("locations.contacts", self.contacts).into_vec().into_iter().map(|v| v.mapped(problems)).collect(),
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
