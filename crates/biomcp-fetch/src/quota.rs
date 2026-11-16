//! Provider pacing declarations and process-owned source admission.
mod admit;
pub use admit::PACED_CEILING;
mod declaration;
mod limiter;
mod pacing;
mod rate_share;
pub use pacing::{PacingOverride, PacingRate, SourcePacing};

pub use declaration::{Quota, Rate, SourceKey};
pub(crate) use limiter::Limiters;
pub use rate_share::{InvalidRateShare, RateShare};
