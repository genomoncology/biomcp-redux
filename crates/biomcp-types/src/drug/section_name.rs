//! Public drug section names.
biodata::canonical_enum! {
    /// Explicit drug disclosures; identity cards never expand into these sections.
    DrugSection, SourceUnknown {
        /// All native identity cross-references and product links, mechanically paged; MyChem rows include ChEMBL ATC codes.
        Xrefs => "xrefs",
        /// Native target and mechanism relations.
        Targets => "targets",
        /// Native indication wording.
        Indications => "indications",
        /// Native CIViC therapy evidence and profile context.
        Civic => "civic",
        /// One exact native label heading; label replies also name the application numbers and recent major changes.
        Label => "label",
        /// Native application and product history rows.
        Approvals => "approvals",
        /// The Drugs@FDA label version whose --label-section first holds a --find text, read from FDA's label PDFs.
        LabelHistory => "label-history",
        /// Source-scoped regulator events.
        Regulatory => "regulatory",
        /// Native DDInter partner rows and levels; five rows per default page, with explicit section limits preserved.
        /// `levels` counts pairs per level across every page. DDInter's published files carry a level per pair and
        /// no management or mechanism text; `next` offers a search for the drug's openFDA label, whose drug_interactions
        /// section has that text.
        Interactions => "interactions",
        /// One row per market (Drugs@FDA, EMA, WHO prequalification), each from its own source; --source names one market.
        Markets => "markets",
        /// Essential Medicines List listings from the eEML: core or complementary, section, indication,
        /// date added, therapeutic alternatives and Expert Committee text; five per default page.
        EssentialMedicines => "essential-medicines",
    }
}
