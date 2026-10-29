//! Registry site records retain source contacts and never imply trial recruitment.
use crate::FiniteNumber;
use serde::{Deserialize, Serialize};
/// One source site, kept whole during pagination.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialLocation {
    /// Source facility name, absent only when not supplied.
    pub facility: Option<Box<str>>,
    /// Source city.
    pub city: Option<Box<str>>,
    /// Source state or region.
    pub state: Option<Box<str>>,
    /// Source postal code.
    pub zip: Option<Box<str>>,
    /// Source country.
    pub country: Option<Box<str>>,
    /// This site's native status; never copied from the overall trial status.
    pub status: Option<TrialSiteStatus>,
    /// Registry coordinates without a derived distance.
    pub coordinates: Option<TrialCoordinates>,
    /// Contacts belong only to this site, in source order.
    #[serde(default, skip_serializing_if = "<[_]>::is_empty")]
    pub contacts: Box<[TrialContact]>,
}
/// A registry contact with its source role and original contact details.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialContact {
    /// Source name.
    pub name: Option<Box<str>>,
    /// Source contact role, absent if the registry omitted it.
    pub role: Option<TrialContactRole>,
    /// Source telephone number, without normalization.
    pub phone: Option<Box<str>>,
    /// Source extension, kept separate from the number.
    pub phone_ext: Option<Box<str>>,
    /// Source email address.
    pub email: Option<Box<str>>,
}
/// An overall official, distinct from a central or site contact.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
pub struct TrialOfficial {
    /// Registered name.
    pub name: Option<Box<str>>,
    /// Source organization.
    pub affiliation: Option<Box<str>>,
    /// Source official role.
    pub role: Option<TrialOfficialRole>,
}
crate::wire_enum! {
    /// Native site recruitment status, distinct from overall trial status.
    TrialSiteStatus {
        /// Not yet recruiting at this site.
        NotYetRecruiting => "NOT_YET_RECRUITING",
        /// Recruiting at this site.
        Recruiting => "RECRUITING",
        /// Enrolling by invitation at this site.
        EnrollingByInvitation => "ENROLLING_BY_INVITATION",
        /// Active but not recruiting at this site.
        ActiveNotRecruiting => "ACTIVE_NOT_RECRUITING",
        /// Temporarily suspended at this site.
        Suspended => "SUSPENDED",
        /// Terminated at this site.
        Terminated => "TERMINATED",
        /// Completed at this site.
        Completed => "COMPLETED",
        /// Withdrawn at this site.
        Withdrawn => "WITHDRAWN",
        /// Available at this site.
        Available => "AVAILABLE",
    }
}
crate::wire_enum! {
    /// Native contact role.
    TrialContactRole {
        /// Named contact.
        Contact => "CONTACT",
        /// Study chair.
        StudyChair => "STUDY_CHAIR",
        /// Study director.
        StudyDirector => "STUDY_DIRECTOR",
        /// Principal investigator.
        PrincipalInvestigator => "PRINCIPAL_INVESTIGATOR",
        /// Sub-investigator.
        SubInvestigator => "SUB_INVESTIGATOR",
    }
}
crate::wire_enum! {
    /// Native overall official role.
    TrialOfficialRole {
        /// Sub-investigator.
        SubInvestigator => "SUB_INVESTIGATOR",
        /// Principal investigator.
        PrincipalInvestigator => "PRINCIPAL_INVESTIGATOR",
        /// Study director.
        StudyDirector => "STUDY_DIRECTOR",
        /// Study chair.
        StudyChair => "STUDY_CHAIR",
    }
}
/// Checked geographic coordinates in degrees; no genomic coordinate type is involved.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "Coordinates")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Coordinates"))]
pub struct TrialCoordinates {
    latitude: FiniteNumber,
    longitude: FiniteNumber,
}
#[derive(Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Coordinates {
    /// Latitude in degrees.
    #[cfg_attr(feature = "schema", schemars(range(min = -90, max = 90)))]
    latitude: FiniteNumber,
    /// Longitude in degrees.
    #[cfg_attr(feature = "schema", schemars(range(min = -180, max = 180)))]
    longitude: FiniteNumber,
}
impl TryFrom<Coordinates> for TrialCoordinates {
    type Error = &'static str;
    fn try_from(v: Coordinates) -> Result<Self, Self::Error> {
        if !(-90.0..=90.0).contains(&f64::from(v.latitude))
            || !(-180.0..=180.0).contains(&f64::from(v.longitude))
        {
            return Err("trial.coordinates_invalid");
        }
        Ok(Self {
            latitude: v.latitude,
            longitude: v.longitude,
        })
    }
}

impl TrialCoordinates {
    /// Source latitude in degrees.
    pub fn latitude(&self) -> FiniteNumber {
        self.latitude
    }
    /// Source longitude in degrees.
    pub fn longitude(&self) -> FiniteNumber {
        self.longitude
    }
    /// Validate source latitude and longitude before admitting a registry record.
    pub fn new(latitude: FiniteNumber, longitude: FiniteNumber) -> Result<Self, &'static str> {
        Self::try_from(Coordinates {
            latitude,
            longitude,
        })
    }
}
