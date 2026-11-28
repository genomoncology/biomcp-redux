use crate::openfda::{Envelope, Item};
use crate::raw::Field;
pub(super) type Response<T = Header> = Envelope<T>;
use biodata::ReportId;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct Header {
    pub safetyreportid: ReportId,
    pub safetyreportversion: Option<crate::raw::Field<Box<str>>>,
    pub serious: Option<crate::raw::Field<Box<str>>>,
    pub receivedate: Option<Field<Box<str>>>,
}

impl Item for Header {}

#[derive(Debug, Deserialize)]
pub(super) struct Candidate {
    #[serde(flatten)]
    pub header: Header,
    pub patient: Patient,
}

impl Item for Candidate {}

#[derive(Debug, Deserialize)]
pub(super) struct Patient {
    pub drug: Vec<Drug>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Drug {
    pub drugcharacterization: Option<Box<str>>,
    pub medicinalproduct: Option<Box<str>>,
    pub openfda: Option<Names>,
}

#[derive(Debug, Deserialize)]
pub(super) struct Names {
    pub generic_name: Option<Vec<Box<str>>>,
    pub brand_name: Option<Vec<Box<str>>>,
}
