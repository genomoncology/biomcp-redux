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
    UnsupportedTrialFilter {
        source: crate::Source,
        option: crate::TrialFilter,
    },
    /// NCI requires explicit disease identities.
    NciDiseaseError,
    /// An opaque query cannot enforce source terms.
    VariantQueryPolicyError,
    /// A refused PDF retains its native link.
    PdfSaveRefused(crate::PdfLink),
    /// A refused save retains its native URL.
    ArticleSaveRefused(crate::Text),
    /// A provider cannot apply the selected article filter.
    UnsupportedFilter {
        source: crate::Source,
        option: crate::ArticleFilter,
    },
    /// A body position is invalid.
    TextPositionError,
    /// Citation ordering exceeds its checked bound.
    TooManyPmids { count: u64 },
    /// Generic checked arguments and fixed grammar guidance.
    Arguments(UsageArguments),
    /// An invalid schema identity.
    Schema,
    /// An unknown command and its grammar-owned recovery.
    UnknownCommand { next: crate::Step },
    /// Fixed migration guidance.
    Retired(crate::ErrorMessage),
    /// A transport cannot execute this command.
    McpUnavailable(crate::ErrorMessage),
    /// A hosted source key belongs in account settings.
    HostedKey(crate::Source),
    /// A complete MCP result exceeds its bound.
    McpResultTooBroad { cap: u64 },
    /// Native source selector vocabulary and recovery.
    SourceSelector {
        source: crate::Source,
        option: Box<str>,
        valid_values: Box<[crate::Text]>,
        next: Vec<crate::Step>,
    },
    /// Checked selectors differ from the retained pool.
    PoolMismatch { restart: crate::Step },
}

/// Fixed grammar guidance without rejected caller values.
#[derive(Clone, Debug, Default)]
pub struct UsageArguments {
    /// Grammar-owned argument identity.
    pub argument: Option<Box<str>>,
    /// Constructor-owned refusal message.
    pub message: Option<crate::ErrorMessage>,
    /// Supported vocabulary for the checked argument.
    pub valid_values: Option<Box<[crate::Text]>>,
    /// Grammar-owned recovery steps.
    pub next: Vec<crate::Step>,
}
impl UsageError {
    /// Refuse arguments without retaining caller values.
    pub fn arguments() -> Self {
        Self::Arguments(UsageArguments::default())
    }
    /// Name a grammar-owned argument.
    pub fn argument(argument: Box<str>) -> Self {
        Self::Arguments(UsageArguments {
            argument: Some(argument),
            ..Default::default()
        })
    }
    /// Preserve a fixed constructor refusal message.
    pub fn message(message: crate::ErrorMessage) -> Self {
        Self::Arguments(UsageArguments {
            message: Some(message),
            ..Default::default()
        })
    }
    /// Name an argument and its supported vocabulary.
    pub fn argument_values(argument: Box<str>, valid_values: Box<[crate::Text]>) -> Self {
        Self::Arguments(UsageArguments {
            argument: Some(argument),
            valid_values: Some(valid_values),
            ..Default::default()
        })
    }
}
