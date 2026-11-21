//! Trial search options each provider can answer, named before any source access.
use super::{TrialSearch, TrialSource};

biodata::wire_enum! {
    /// Checked trial capability names, never caller-supplied values.
    TrialFilter {
        /// Registry general text.
        Text => "TEXT",
        /// Registry acronym search.
        Acronym => "--acronym",
        /// Registry official title search.
        OfficialTitle => "--official-title",
        /// Registry gene discovery.
        Gene => "--gene",
        /// Registry variant discovery.
        Variant => "--variant",
        /// Registry criteria text.
        Criteria => "--criteria",
        /// Registry intervention text.
        Intervention => "--intervention",
        /// Registry status vocabulary.
        Status => "--status",
        /// Registry phase vocabulary.
        Phase => "--phase",
        /// Registry study type.
        StudyType => "--study-type",
        /// Registry sex vocabulary.
        Sex => "--sex",
        /// Registry age range.
        Age => "--age",
        /// Registry facility text.
        Facility => "--facility",
        /// Registry radius.
        Latitude => "--lat",
        /// Registry sponsor text.
        Sponsor => "--sponsor",
        /// Registry sponsor role.
        SponsorRole => "--sponsor-role",
        /// Registry funding vocabulary.
        SponsorType => "--sponsor-type",
        /// Registry lower update bound.
        UpdatedSince => "--updated-since",
        /// Registry upper update bound.
        UpdatedUntil => "--updated-until",
        /// Registry ID conjunction.
        Id => "--id",
        /// Registry results restriction.
        HasResults => "--has-results",
        /// Explicit NCI keyword mode.
        Keyword => "--keyword",
        /// Registry stop reason words.
        WhyStopped => "--why-stopped",
        /// Facility counts across a registry match.
        SiteCount => "--site-count",
    }
}
impl TrialSearch {
    /// Reject a source disease name instead of silently degrading to free text.
    pub fn nci_disease_invalid(&self) -> bool {
        self.source() == TrialSource::Nci
            && self.condition().iter().any(|v| {
                !v.as_str()
                    .strip_prefix('C')
                    .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
            })
    }
    /// Name the first unsupported capability before any source, key, or cache access.
    pub fn unsupported(&self) -> Option<TrialFilter> {
        if self.source() == TrialSource::Ctgov {
            return None;
        }
        use TrialFilter as F;
        [
            (self.query().is_some(), F::Text),
            (self.acronym().is_some(), F::Acronym),
            (self.official_title().is_some(), F::OfficialTitle),
            (self.keyword().len() > 1, F::Keyword),
            (self.gene().is_some(), F::Gene),
            (self.variant().is_some(), F::Variant),
            (self.criteria().is_some(), F::Criteria),
            (!self.intervention().is_empty(), F::Intervention),
            (self.study_type().is_some(), F::StudyType),
            (self.sex().is_some(), F::Sex),
            (self.age().is_some(), F::Age),
            (self.lat().is_some(), F::Latitude),
            (self.sponsor().is_some(), F::Sponsor),
            (self.sponsor_role().is_some(), F::SponsorRole),
            (self.sponsor_type().is_some(), F::SponsorType),
            (self.updated_since().is_some(), F::UpdatedSince),
            (self.updated_until().is_some(), F::UpdatedUntil),
            (self.id().is_some(), F::Id),
            (self.has_results(), F::HasResults),
            (self.why_stopped().is_some(), F::WhyStopped),
            (self.site_count(), F::SiteCount),
        ]
        .into_iter()
        .find_map(|(present, option)| present.then_some(option))
    }
}
