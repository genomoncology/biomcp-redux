//! Trial section names and which source can supply them.
use crate::TrialSource;

biodata::wire_enum! {
    described
    /// Available source protocol sections.
    TrialSection {
        /// Criteria and age/sex fields.
        Eligibility => "eligibility",
        /// Primary, secondary and other outcomes.
        Outcomes => "outcomes",
        /// Posted outcomes, analyses, flow, baseline and adverse events (CT.gov).
        Results => "results",
        /// Started, finished, and dropped out by arm and period, with reasons (CT.gov).
        Flow => "flow",
        /// Baseline populations, groups and measures (CT.gov).
        Baseline => "baseline",
        /// Arms and linked interventions.
        Arms => "arms",
        /// CT.gov sites with contacts. Count missing coordinates before radius filtering.
        Locations => "locations",
        /// Central contacts and officials with roles.
        Contacts => "contacts",
        /// Publication IDs and reference types.
        References => "references",
        /// Document metadata and URLs.
        Documents => "documents",
        /// All sections page separately. Flow and baseline stay in results.
        All => "all",
        /// NCI disease concepts and enrichment.
        Diseases => "diseases",
        /// NCI summary, paged by character.
        Summary => "summary",
    }
}

impl TrialSource {
    /// Implemented protocol sections in source presentation order.
    pub const fn sections(self) -> &'static [TrialSection] {
        use TrialSection::{
            Arms, Baseline, Contacts, Diseases, Documents, Eligibility, Flow, Locations, Outcomes,
            References, Results, Summary,
        };
        match self {
            Self::Ctgov => &[
                Eligibility,
                Outcomes,
                Arms,
                Locations,
                Contacts,
                References,
                Documents,
                Results,
                Flow,
                Baseline,
            ],
            Self::Nci => &[Eligibility, Outcomes, Arms, Diseases, Summary],
        }
    }
}
