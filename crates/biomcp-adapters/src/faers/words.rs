//! Words for coded FAERS values, copied from openFDA's published field reference:
//! <https://open.fda.gov/apis/drug/event/searchable-fields/>, downloadable as
//! <https://open.fda.gov/fields/drugevent.yaml>. Native codes stay in every reply.

/// One published code list.
pub type Words = &'static [(&'static str, &'static str)];

/// `patient.reaction.reactionoutcome`.
pub const REACTION_OUTCOME: Words = &[
    ("1", "Recovered/resolved"),
    ("2", "Recovering/resolving"),
    ("3", "Not recovered/not resolved"),
    (
        "4",
        "Recovered/resolved with sequelae (consequent health issues)",
    ),
    ("5", "Fatal"),
    ("6", "Unknown"),
];
/// `patient.patientsex`.
pub const PATIENT_SEX: Words = &[("0", "Unknown"), ("1", "Male"), ("2", "Female")];
/// `patient.drug.drugcharacterization`.
pub const DRUG_ROLE: Words = &[
    (
        "1",
        "Suspect (the drug was considered by the reporter to be the cause)",
    ),
    (
        "2",
        "Concomitant (the drug was reported as being taken along with the suspect drug)",
    ),
    (
        "3",
        "Interacting (the drug was considered by the reporter to have interacted with the suspect drug)",
    ),
];
/// `patient.drug.actiondrug`.
pub const ACTION_TAKEN: Words = &[
    ("1", "Drug withdrawn"),
    ("2", "Dose reduced"),
    ("3", "Dose increased"),
    ("4", "Dose not changed"),
    ("5", "Unknown"),
    ("6", "Not applicable"),
];
/// `patient.patientonsetageunit`.
pub const AGE_UNIT: Words = &[
    ("800", "Decade"),
    ("801", "Year"),
    ("802", "Month"),
    ("803", "Week"),
    ("804", "Day"),
    ("805", "Hour"),
];
/// `serious`, the report-level flag.
pub const SERIOUS: Words = &[
    (
        "1",
        "The adverse event resulted in death, a life threatening condition, hospitalization, disability, congenital anomaly, or other serious condition",
    ),
    ("2", "The adverse event did not result in any of the above"),
];
/// Short words for `serious` on every FAERS row and section header, drawn from
/// [`SERIOUS`]: `1` is serious and `2` is not; the header's field description
/// carries openFDA's full sentences once.
pub const SERIOUS_SHORT: Words = &[("1", "serious"), ("2", "not serious")];
/// The six seriousness flags have no published code list. Each description reads
/// "This value is `1` if the adverse event resulted in ..., and absent otherwise";
/// the `1` words are that clause. The data also carries `2` on most reports, which
/// openFDA does not document; its words say only that the report did not record
/// that result, so an importer never needs the 1/2 rule. Any other code is off-list.
pub const SERIOUSNESS_DEATH: Words = &[
    ("1", "The adverse event resulted in death"),
    (
        "2",
        "The report did not record that the adverse event resulted in death",
    ),
];
/// `seriousnesslifethreatening`; see [`SERIOUSNESS_DEATH`].
pub const SERIOUSNESS_LIFETHREATENING: Words = &[
    (
        "1",
        "The adverse event resulted in a life threatening condition",
    ),
    (
        "2",
        "The report did not record that the adverse event resulted in a life threatening condition",
    ),
];
/// `seriousnesshospitalization`; see [`SERIOUSNESS_DEATH`].
pub const SERIOUSNESS_HOSPITALIZATION: Words = &[
    ("1", "The adverse event resulted in a hospitalization"),
    (
        "2",
        "The report did not record that the adverse event resulted in a hospitalization",
    ),
];
/// `seriousnessdisabling`; see [`SERIOUSNESS_DEATH`].
pub const SERIOUSNESS_DISABLING: Words = &[
    ("1", "The adverse event resulted in disability"),
    (
        "2",
        "The report did not record that the adverse event resulted in disability",
    ),
];
/// `seriousnesscongenitalanomali`; see [`SERIOUSNESS_DEATH`].
pub const SERIOUSNESS_CONGENITALANOMALI: Words = &[
    ("1", "The adverse event resulted in a congenital anomaly"),
    (
        "2",
        "The report did not record that the adverse event resulted in a congenital anomaly",
    ),
];
/// `seriousnessother`; see [`SERIOUSNESS_DEATH`].
pub const SERIOUSNESS_OTHER: Words = &[
    (
        "1",
        "The adverse event resulted in some other serious condition",
    ),
    (
        "2",
        "The report did not record that the adverse event resulted in some other serious condition",
    ),
];
/// `primarysource.qualification`.
pub const REPORTER_QUALIFICATION: Words = &[
    ("1", "Physician"),
    ("2", "Pharmacist"),
    ("3", "Other health professional"),
    ("4", "Lawyer"),
    ("5", "Consumer or non-health professional"),
];

/// Whether a present code is missing from a published list; an omitted field is not off-list.
pub fn off_list(words: Words, code: Option<&str>) -> bool {
    code.is_some_and(|code| words.iter().all(|(known, _)| *known != code))
}
/// The published word for a code; a code outside the list has none.
pub fn label(words: Words, code: Option<&str>) -> Option<Box<str>> {
    let code = code?;
    words
        .iter()
        .find(|(known, _)| *known == code)
        .map(|(_, word)| (*word).into())
}
