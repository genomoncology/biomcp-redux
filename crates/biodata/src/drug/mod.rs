//! Source-native drug records without consumer requests, policy or I/O.
mod claims;
mod native;
mod target;
mod interaction;
mod id;
mod identity;
mod regulatory;
pub use claims::{
    DrugIndicationClaim, DrugLabelReference, DrugLabelSection,
};
pub use id::DrugId;
pub use identity::{DrugIdentity, DrugIdentityLevel, DrugIdentityRelation, DrugProduct};
pub use regulatory::{AuthorizationDate, AuthorizationEvent};

pub use native::{DrugNativeTarget, DrugDiseaseReference, DrugStage, DrugNativeReference};
pub use target::DrugTargetRelation;
pub use interaction::DrugInteractionClaim;
