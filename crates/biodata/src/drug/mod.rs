//! Source-native drug records without consumer requests, policy or I/O.
mod claims;
mod id;
mod identity;
mod regulatory;
pub use claims::{
    DrugIndicationClaim, DrugInteractionClaim, DrugLabelReference, DrugLabelSection,
    DrugTargetRelation,
};
pub use id::DrugId;
pub use identity::{DrugIdentity, DrugIdentityLevel, DrugIdentityRelation, DrugProduct};
pub use regulatory::{AuthorizationDate, AuthorizationEvent};
