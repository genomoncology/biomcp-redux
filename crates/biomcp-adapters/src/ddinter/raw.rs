use biodata::Text;
use biomcp_fetch::{KeyKind, Keys, Row};
use serde::{Deserialize, Serialize};

/// One checked pair from the publisher's native CSV schema.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(try_from = "Fields", into = "Fields")]
pub struct NativeRow(Fields);
#[derive(Clone, Debug, Deserialize, Serialize)]
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
    type Error = &'static str;
    fn try_from(fields: Fields) -> Result<Self, Self::Error> {
        for id in [&fields.id_a, &fields.id_b] {
            if !valid_id(id.as_str()) {
                return Err("expected a published DDInter identifier");
            }
        }
        Ok(Self(fields))
    }
}
impl From<NativeRow> for Fields {
    fn from(row: NativeRow) -> Self {
        row.0
    }
}
pub(super) fn valid_id(id: &str) -> bool {
    id.strip_prefix("DDInter")
        .is_some_and(|n| !n.is_empty() && n.bytes().all(|b| b.is_ascii_digit()))
}
impl NativeRow {
    /// Published identity at the first end of the native pair.
    pub fn id_a(&self) -> &str {
        self.0.id_a.as_str()
    }
    /// Published identity at the second end of the native pair.
    pub fn id_b(&self) -> &str {
        self.0.id_b.as_str()
    }
    /// Native display name for the first identity.
    pub fn name_a(&self) -> &str {
        self.0.name_a.as_str()
    }
    /// Native display name for the second identity.
    pub fn name_b(&self) -> &str {
        self.0.name_b.as_str()
    }
    /// Native interaction level, without interpretation.
    pub fn level(&self) -> &str {
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
