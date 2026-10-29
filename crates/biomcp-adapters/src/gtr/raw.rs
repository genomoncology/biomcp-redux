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
pub(super) enum Object {
    Gene,
    Condition,
}
impl Row for Native {
    fn check_part(&self, part: &str) -> Result<(), biomcp_fetch::BadReply> {
        if matches!(
            (self, part),
            (Self::Version(_), "versions") | (Self::Relation(_), "relations")
        ) {
            Ok(())
        } else {
            Err(biomcp_fetch::BadReply)
        }
    }
    fn check_bundle(&self, _rows: usize) -> Result<(), biomcp_fetch::BadReply> {
        match self {
            Self::Version(v) => super::map::card(v, &mut Vec::new())
                .map(|_| ())
                .map_err(|_| biomcp_fetch::BadReply),
            Self::Relation(v) => super::map::relation(v)
                .map(|_| ())
                .map_err(|_| biomcp_fetch::BadReply),
        }
    }
    fn keys(&self) -> Keys {
        let mut exact = vec![(KeyKind::Id, self.id().to_string().into())];
        match self {
            Self::Version(_) => exact.push((KeyKind::Band, "versions".into())),
            Self::Relation(r) => {
                exact.push((KeyKind::Band, "relations".into()));
                exact.push((KeyKind::Alias, r.test_type.as_str().into()));
                if r.object == Object::Gene {
                    exact.push((KeyKind::Symbol, r.gene_symbol.clone()));
                    if let Ok(id) = r.object_id.parse::<biodata::NcbiGeneId>() {
                        exact.push((KeyKind::Xref, format!("gene:{id}").into()));
                    }
                } else {
                    exact.push((KeyKind::Xref, format!("MedGen:{}", r.medgen).into()));
                }
            }
        }
        Keys {
            exact,
            words: Vec::new(),
        }
    }
}
impl Native {
    pub(super) fn id(&self) -> &GtrTestId {
        match self {
            Self::Version(v) => &v.test_accession_ver,
            Self::Relation(v) => &v.id,
        }
    }
}
