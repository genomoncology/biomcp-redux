//! Trial section names and which source can supply them.
use crate::TrialSource;

biodata::wire_enum! {
    described
    /// Available source protocol sections.
    TrialSection {
        /// Intact criteria and structured age/sex fields.
        Eligibility => "eligibility",
        /// Primary, secondary and other outcome records.
        Outcomes => "outcomes",
        /// Posted outcome values and analyses, then participant flow, baseline and adverse-event tables (ClinicalTrials.gov).
        Results => "results",
        /// Participant flow alone: started, completed and not completed by arm and period, with dropout reasons (ClinicalTrials.gov).
        Flow => "flow",
        /// Baseline characteristics alone: analysis population, groups and measures (ClinicalTrials.gov).
        Baseline => "baseline",
        /// Arms and interventions with original relationships.
        Arms => "arms",
        /// Whole source sites with contacts. Filters apply before paging; missing coordinates are counted before the radius filter.
        Locations => "locations",
        /// Central contacts and overall officials in their original roles.
        Contacts => "contacts",
        /// Registry publication identities and native reference types.
        References => "references",
        /// Posted document metadata and source URLs.
        Documents => "documents",
        /// Every implemented trial section, each with independent continuations; flow and baseline stay within results.
        All => "all",
        /// Native NCI disease concepts, including source enrichment metadata.
        Diseases => "diseases",
        /// Complete NCI summary, paged by original character offsets.
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
