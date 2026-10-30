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
        pub(super) struct $name { $(pub $field: Option<crate::raw::Field<Box<str>>>),* }
    };
}
#[derive(Debug, Deserialize)]
pub(super) struct Report {
    pub safetyreportid: ReportId,
    pub safetyreportversion: Option<crate::raw::Field<Box<str>>>,
    pub serious: Option<crate::raw::Field<Box<str>>>,
    pub receivedate: Option<crate::raw::Field<Box<str>>>,
    pub receivedateformat: Option<crate::raw::Field<Box<str>>>,
    pub receiptdate: Option<crate::raw::Field<Box<str>>>,
    pub receiptdateformat: Option<crate::raw::Field<Box<str>>>,
    pub transmissiondate: Option<crate::raw::Field<Box<str>>>,
    pub transmissiondateformat: Option<crate::raw::Field<Box<str>>>,
    pub seriousnessdeath: Option<crate::raw::Field<Box<str>>>,
    pub seriousnesslifethreatening: Option<crate::raw::Field<Box<str>>>,
    pub seriousnesshospitalization: Option<crate::raw::Field<Box<str>>>,
    pub seriousnessdisabling: Option<crate::raw::Field<Box<str>>>,
    pub seriousnesscongenitalanomali: Option<crate::raw::Field<Box<str>>>,
    pub seriousnessother: Option<crate::raw::Field<Box<str>>>,
    pub primarysource: Option<crate::raw::Field<Reporter>>,
    pub patient: Option<crate::raw::Field<Patient>>,
}
strings!(Reporter {
    qualification,
    reportercountry
});
#[derive(Debug, Deserialize)]
pub(super) struct Patient {
    pub patientonsetage: Option<crate::raw::Field<String>>,
    pub patientonsetageunit: Option<crate::raw::Field<biodata::FaersAgeUnit>>,
    pub patientsex: Option<crate::raw::Field<Box<str>>>,
    pub patientweight: Option<crate::raw::Field<Box<str>>>,
    pub drug: Option<crate::raw::Field<Vec<crate::raw::Field<Drug>>>>,
    pub reaction: Option<crate::raw::Field<Vec<crate::raw::Field<Reaction>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Reaction {
    pub reactionmeddrapt: Option<crate::raw::Field<crate::native_text::NativeText>>,
    pub reactionmeddraversionpt: Option<crate::raw::Field<Box<str>>>,
    pub reactionoutcome: Option<crate::raw::Field<Box<str>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Identifiers {
    pub unii: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    pub rxcui: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    pub product_ndc: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    pub package_ndc: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    pub spl_id: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    pub spl_set_id: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
    pub application_number: Option<crate::raw::Field<Vec<crate::raw::Field<Box<str>>>>>,
}
#[derive(Debug, Deserialize)]
pub(super) struct Drug {
    pub medicinalproduct: Option<crate::raw::Field<Box<str>>>,
    pub drugcharacterization: Option<crate::raw::Field<Box<str>>>,
    pub drugindication: Option<crate::raw::Field<Box<str>>>,
    pub drugadministrationroute: Option<crate::raw::Field<Box<str>>>,
    pub drugauthorizationnumb: Option<crate::raw::Field<Box<str>>>,
    pub drugbatchnumb: Option<crate::raw::Field<Box<str>>>,
    pub drugcumulativedosagenumb: Option<crate::raw::Field<Box<str>>>,
    pub drugcumulativedosageunit: Option<crate::raw::Field<Box<str>>>,
    pub drugdosageform: Option<crate::raw::Field<Box<str>>>,
    pub drugdosagetext: Option<crate::raw::Field<Box<str>>>,
    pub drugenddate: Option<crate::raw::Field<Box<str>>>,
    pub drugenddateformat: Option<crate::raw::Field<Box<str>>>,
    pub drugintervaldosagedefinition: Option<crate::raw::Field<Box<str>>>,
    pub drugintervaldosageunitnumb: Option<crate::raw::Field<Box<str>>>,
    pub drugrecurreadministration: Option<crate::raw::Field<Box<str>>>,
    pub drugseparatedosagenumb: Option<crate::raw::Field<Box<str>>>,
    pub drugstartdate: Option<crate::raw::Field<Box<str>>>,
    pub drugstartdateformat: Option<crate::raw::Field<Box<str>>>,
    pub drugstructuredosagenumb: Option<crate::raw::Field<Box<str>>>,
    pub drugstructuredosageunit: Option<crate::raw::Field<Box<str>>>,
    pub drugtreatmentduration: Option<crate::raw::Field<Box<str>>>,
    pub drugtreatmentdurationunit: Option<crate::raw::Field<Box<str>>>,
    pub actiondrug: Option<crate::raw::Field<Box<str>>>,
    pub drugadditional: Option<crate::raw::Field<Box<str>>>,
    pub openfda: Option<crate::raw::Field<Identifiers>>,
}
