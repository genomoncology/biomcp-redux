//! NCI STRAP glossary, Study Design > Design > Phase, publishes this complete LOV.
// https://clinicaltrialsapi.cancer.gov/glossary
biodata::native_enum! {
    /// Exact NCI phase labels; registry phase values belong to TrialPhase.
    NciTrialPhase {
        /// Publisher NA label.
        NotApplicable => "NA",
        /// Publisher phase 0 label.
        Phase0 => "0",
        /// Publisher phase I label.
        Phase1 => "I",
        /// Publisher phase I_II label.
        Phase1And2 => "I_II",
        /// Publisher phase II label.
        Phase2 => "II",
        /// Publisher phase II_III label.
        Phase2And3 => "II_III",
        /// Publisher phase III label.
        Phase3 => "III",
        /// Publisher phase IV label.
        Phase4 => "IV",
    }
}
