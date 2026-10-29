//! Exact native FAERS fields; unrelated publisher fields are ignored at this edge.
use biodata::ReportId;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Response {
    Found {
        meta: super::raw::Meta,
        results: Vec<Report>,
    },
    Absent {
        error: super::raw::Absence,
    },
}
impl Reply for Response {
    fn check(&self) -> Result<(), BadReply> {
        match self {
            Self::Found { meta, results }
                if meta.results.skip == 0
                    && meta.results.limit == 5
                    && meta.results.total == 1
                    && results.len() == 1 =>
            {
                Ok(())
            }
            Self::Absent { error } if error.valid() => Ok(()),
            _ => Err(BadReply),
        }
    }
    fn check_status(&self, status: u16) -> Result<(), BadReply> {
        if matches!(
            (self, status),
            (Self::Found { .. }, 200) | (Self::Absent { .. }, 404)
        ) {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
macro_rules! strings {
    ($name:ident { $($field:ident),* $(,)? }) => {
        #[derive(Debug, Deserialize)]
        pub(super) struct $name { $(pub $field: Option<Box<str>>),* }
    };
}
#[derive(Debug, Deserialize)]
pub(super) struct Report {
    pub safetyreportid: ReportId,
    pub safetyreportversion: Option<Box<str>>,
    pub serious: Option<Box<str>>,
    pub receivedate: Option<Box<str>>,
    pub receivedateformat: Option<Box<str>>,
    pub receiptdate: Option<Box<str>>,
    pub receiptdateformat: Option<Box<str>>,
    pub transmissiondate: Option<Box<str>>,
    pub transmissiondateformat: Option<Box<str>>,
    pub seriousnessdeath: Option<Box<str>>,
    pub seriousnesslifethreatening: Option<Box<str>>,
    pub seriousnesshospitalization: Option<Box<str>>,
    pub seriousnessdisabling: Option<Box<str>>,
    pub seriousnesscongenitalanomali: Option<Box<str>>,
    pub seriousnessother: Option<Box<str>>,
    pub primarysource: Option<Reporter>,
    pub patient: Patient,
}
strings!(Reporter {
    qualification,
    reportercountry
});
#[derive(Debug, Deserialize)]
pub(super) struct Patient {
    pub patientonsetage: Option<Box<str>>,
    pub patientonsetageunit: Option<Box<str>>,
    pub patientsex: Option<Box<str>>,
    pub patientweight: Option<Box<str>>,
    pub drug: Vec<Drug>,
    pub reaction: Vec<Reaction>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Reaction {
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub reactionmeddrapt: biodata::Text,
    pub reactionmeddraversionpt: Option<Box<str>>,
    pub reactionoutcome: Option<Box<str>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Identifiers {
    pub unii: Option<Box<[Box<str>]>>,
    pub rxcui: Option<Box<[Box<str>]>>,
    pub product_ndc: Option<Box<[Box<str>]>>,
    pub package_ndc: Option<Box<[Box<str>]>>,
    pub spl_id: Option<Box<[Box<str>]>>,
    pub spl_set_id: Option<Box<[Box<str>]>>,
    pub application_number: Option<Box<[Box<str>]>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Drug {
    pub medicinalproduct: Option<Box<str>>,
    pub drugcharacterization: Option<Box<str>>,
    pub drugindication: Option<Box<str>>,
    pub drugadministrationroute: Option<Box<str>>,
    pub drugauthorizationnumb: Option<Box<str>>,
    pub drugbatchnumb: Option<Box<str>>,
    pub drugcumulativedosagenumb: Option<Box<str>>,
    pub drugcumulativedosageunit: Option<Box<str>>,
    pub drugdosageform: Option<Box<str>>,
    pub drugdosagetext: Option<Box<str>>,
    pub drugenddate: Option<Box<str>>,
    pub drugenddateformat: Option<Box<str>>,
    pub drugintervaldosagedefinition: Option<Box<str>>,
    pub drugintervaldosageunitnumb: Option<Box<str>>,
    pub drugrecurreadministration: Option<Box<str>>,
    pub drugseparatedosagenumb: Option<Box<str>>,
    pub drugstartdate: Option<Box<str>>,
    pub drugstartdateformat: Option<Box<str>>,
    pub drugstructuredosagenumb: Option<Box<str>>,
    pub drugstructuredosageunit: Option<Box<str>>,
    pub drugtreatmentduration: Option<Box<str>>,
    pub drugtreatmentdurationunit: Option<Box<str>>,
    pub actiondrug: Option<Box<str>>,
    pub drugadditional: Option<Box<str>>,
    pub openfda: Option<Identifiers>,
}
