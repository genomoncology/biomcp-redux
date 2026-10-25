use biodata::{GtrTestId, GtrTestType, GtrTestVersion};
use biomcp_fetch::{KeyKind, Keys, Row};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub(super) enum Native {
    Version(Box<GtrTestVersion>),
    Relation(Relation),
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Relation {
    #[serde(rename = "#accession_version")]
    pub id: GtrTestId,
    pub test_type: GtrTestType,
    pub object: Object,
    #[serde(rename = "GTR_identifier")]
    pub medgen: Box<str>,
    #[serde(rename = "MIM_number")]
    pub mim: Box<str>,
    pub object_name: Box<str>,
    #[serde(rename = "gene_or_SNOMED_CT_ID")]
    pub object_id: Box<str>,
    pub gene_symbol: Box<str>,
}
#[derive(Debug, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub(super) enum Object { Gene, Condition }
impl Row for Native {
    fn keys(&self) -> Keys {
        let mut exact = vec![(KeyKind::Id, self.id().to_string().into())];
        match self {
            Self::Version(_) => exact.push((KeyKind::Band, "versions".into())),
            Self::Relation(r) => {
                exact.push((KeyKind::Band, "relations".into()));
                exact.push((KeyKind::Alias, r.test_type.to_string().into()));
                if r.object == Object::Gene {
                    exact.push((KeyKind::Symbol, r.gene_symbol.clone()));
                    exact.push((KeyKind::Xref, format!("gene:{}", r.object_id).into()));
                } else {
                    exact.push((KeyKind::Xref, format!("MedGen:{}", r.medgen).into()));
                }
            }
        }
        Keys { exact, words: Vec::new() }
    }
}
impl Native {
    pub(super) fn id(&self) -> &GtrTestId {
        match self { Self::Version(v) => &v.test_accession_ver, Self::Relation(v) => &v.id }
    }
}
