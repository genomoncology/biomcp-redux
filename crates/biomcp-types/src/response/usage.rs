//! Fixed query refusals kept separate from successful reply bodies.
/// Checked refusal without rejected caller input.
#[derive(Clone, Debug)]
pub enum UsageError {
    /// An observation exceeds its source bound.
    BiomarkerTooBroad,
    /// An organization observation exceeds its source bound.
    OrganizationTooBroad,
    /// The selected report surface is unavailable.
    UnsupportedAdverseEvent,
    /// A trial provider cannot apply the selected filter.
    UnsupportedTrialFilter { source: crate::Source, option: crate::TrialFilter },
    /// NCI requires explicit disease identities.
    NciDiseaseError,
    /// An opaque query cannot enforce source terms.
    VariantQueryPolicyError,
    /// A refused PDF retains its native link.
    PdfSaveRefused(crate::PdfLink),
    /// A refused save retains its native URL.
    ArticleSaveRefused(crate::Text),
    /// A provider cannot apply the selected article filter.
    UnsupportedFilter { source: crate::Source, option: crate::ArticleFilter },
    /// A body position is invalid.
    TextPositionError,
    /// Citation ordering exceeds its checked bound.
    TooManyPmids { count: u64 },
    /// Snapshot selectors require a typed restart.
    PoolError { mismatch: bool },
}
