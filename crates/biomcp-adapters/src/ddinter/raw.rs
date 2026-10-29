use biodata::Text;
use biomcp_fetch::{KeyKind, Keys, Row};
use serde::Deserialize;

/// One checked pair from the publisher's native CSV schema.
#[derive(Debug, Deserialize)]
#[serde(try_from = "Fields")]
pub(super) struct NativeRow(Fields);
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Fields {
    #[serde(rename = "DDInterID_A")]
    id_a: Text,
    #[serde(rename = "Drug_A")]
    name_a: Text,
    #[serde(rename = "DDInterID_B")]
    id_b: Text,
    #[serde(rename = "Drug_B")]
    name_b: Text,
    #[serde(rename = "Level")]
    level: Text,
}
impl TryFrom<Fields> for NativeRow {
    type Error = biomcp_fetch::BadReply;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        for id in [&fields.id_a, &fields.id_b] {
            if !valid_id(id.as_str()) {
                return Err(biomcp_fetch::BadReply);
            }
        }
        Ok(Self(fields))
    }
}
fn valid_id(id: &str) -> bool {
    id.strip_prefix("DDInter")
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}
impl NativeRow {
    /// Published identity at the first end of the native pair.
    pub(super) fn id_a(&self) -> &str {
        self.0.id_a.as_str()
    }
    /// Published identity at the second end of the native pair.
    pub(super) fn id_b(&self) -> &str {
        self.0.id_b.as_str()
    }
    /// Native display name for the first identity.
    pub(super) fn name_a(&self) -> &str {
        self.0.name_a.as_str()
    }
    /// Native display name for the second identity.
    pub(super) fn name_b(&self) -> &str {
        self.0.name_b.as_str()
    }
    /// Native interaction level, without interpretation.
    pub(super) fn level(&self) -> &str {
        self.0.level.as_str()
    }
}
impl Row for NativeRow {
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![
                (KeyKind::Id, self.id_a().into()),
                (KeyKind::Id, self.id_b().into()),
            ],
            words: vec![self.name_a().into(), self.name_b().into()],
        }
    }
}
