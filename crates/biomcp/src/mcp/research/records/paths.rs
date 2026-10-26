//! Native record paths shared by search and fetch projection.
pub(super) const TITLES: &[&str] = &[
    "/title",
    "/name",
    "/header/name",
    "/concept/label",
    "/label",
    "/term/label",
    "/pathway/name",
    "/study/header/name",
    "/brief_title",
    "/protein/name",
    "/identity/name",
];
pub(super) const URLS: &[&str] = &[
    "/url",
    "/header/url",
    "/attribution/url",
    "/record/url",
    "/term/attribution/url",
    "/identity/url",
];

pub(super) const VARIANT_IDS: &[&str] = &["/record/id"];

pub(super) const IDS: &[&str] = &[
    "/id",
    "/header/id",
    "/concept/id",
    "/product/id",
    "/protein/accession",
    "/identity/id",
    "/studyId",
    "/study/header/studyId",
    "/pathway/id",
    "/k_number",
    "/reference",
    "/nci_id",
    "/nct_id",
    "/term/returned_id",
    "/association/id",
];
