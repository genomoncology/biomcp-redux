use super::DrugIdentity;
use crate::{Attribution, Error, Text};
use serde::{Deserialize, Serialize};

/// One native interaction pair and level, without invented management advice.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "Fields")]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(with = "Fields"))]
pub struct DrugInteractionClaim(Fields);
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
struct Fields {
    /// Selected native anchor ID and name.
    anchor: DrugIdentity,
    /// Native partner ID and name.
    partner: DrugIdentity,
    /// Original native level, without a computed severity.
    level: Text,
    /// Primary pair record and bundle provenance.
    attribution: Attribution,
    /// Further distinct provenance, excluding the primary attribution.
    #[serde(default, skip_serializing_if = "<[ _ ]>::is_empty")]
    #[cfg_attr(feature = "schema", schemars(extend("uniqueItems" = true)))]
    additional_attributions: Box<[Attribution]>,
}
impl DrugInteractionClaim {
    /// Keep primary and further origins; reject exact duplicate provenance records.
    pub fn new(
        anchor: DrugIdentity,
        partner: DrugIdentity,
        level: Text,
        attribution: Attribution,
        additional_attributions: Box<[Attribution]>,
    ) -> Result<Self, Error> {
        Self::try_from(Fields {
            anchor,
            partner,
            level,
            attribution,
            additional_attributions,
        })
    }
    /// Selected native anchor.
    pub fn anchor(&self) -> &DrugIdentity {
        &self.0.anchor
    }
    /// Native interaction partner.
    pub fn partner(&self) -> &DrugIdentity {
        &self.0.partner
    }
    /// Original native level.
    pub fn level(&self) -> &Text {
        &self.0.level
    }
    /// Primary source record or bundle provenance.
    pub fn attribution(&self) -> &Attribution {
        &self.0.attribution
    }
}
impl TryFrom<Fields> for DrugInteractionClaim {
    type Error = Error;
    fn try_from(value: Fields) -> Result<Self, Error> {
        let mut seen = vec![&value.attribution];
        for item in &value.additional_attributions {
            if seen.contains(&item) {
                return Err(Error::DrugAttributionDuplicate);
            }
            seen.push(item);
        }
        Ok(Self(value))
    }
}
