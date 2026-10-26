//! Native record paths shared by search and fetch projection.
pub(super) const TITLES: &[&str] = &[
    "/title",
    "/name",
    "/header/name",
    "/concept/label",
    "/label",
    "/term/label",
    "/pathway/name",
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
