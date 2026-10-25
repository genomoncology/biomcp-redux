//! Source-native drug records without consumer requests, policy or I/O.
mod id;
mod identity;
mod claims;
mod regulatory;
pub use id::DrugId;
pub use identity::{DrugIdentity, DrugIdentityLevel, DrugIdentityRelation, DrugProduct};
pub use claims::{DrugIndicationClaim, DrugInteractionClaim, DrugLabelReference, DrugLabelSection, DrugTargetRelation};
pub use regulatory::{AuthorizationDate, AuthorizationEvent};
