//! Public drug section names.
biodata::canonical_enum! {
    /// Explicit drug disclosures; identity cards never expand into these sections.
    DrugSection, SourceUnknown {
        /// Paged identity cross-references and product links. MyChem includes ChEMBL ATC codes.
        Xrefs => "xrefs",
        /// Native target and mechanism relations.
        Targets => "targets",
        /// Native indication wording.
        Indications => "indications",
        /// Native CIViC therapy evidence and profile context.
        Civic => "civic",
        /// Exact label heading, application numbers and recent major changes.
        Label => "label",
        /// Native application and product history rows.
        Approvals => "approvals",
        /// First Drugs@FDA label PDF version with a --find match in --label-section.
        LabelHistory => "label-history",
        /// Source-scoped regulator events.
        Regulatory => "regulatory",
        /// DDInter partners and levels (default five per page). `levels` counts pairs across all pages.
        /// For management and mechanism text, follow `next` to the openFDA drug_interactions label field.
        Interactions => "interactions",
        /// One row per market (Drugs@FDA, EMA, WHO prequalification). Use --source for one market.
        Markets => "markets",
        /// eEML listings: core or complementary, section, indication, date added, therapeutic alternatives
        /// and Expert Committee text (default five per page).
        EssentialMedicines => "essential-medicines",
    }
}
