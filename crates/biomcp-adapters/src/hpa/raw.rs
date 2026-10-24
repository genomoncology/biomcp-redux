use crate::raw::Field;
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;
use std::collections::BTreeMap;
#[derive(Debug, Deserialize)]
pub(super) struct Gene {
    #[serde(rename = "Ensembl")]
    pub ensembl: String,
    #[serde(rename = "RNA tissue specificity")]
    pub tissue_specificity: Option<Field<String>>,
    #[serde(rename = "RNA tissue distribution")]
    pub tissue_distribution: Option<Field<String>>,
    #[serde(rename = "RNA tissue specific nTPM")]
    pub ntpm: Option<Field<BTreeMap<String, Field<String>>>>,
    #[serde(rename = "RNA cancer specificity")]
    pub cancer_specificity: Option<Field<String>>,
    #[serde(rename = "RNA cancer specific pTPM")]
    pub ptpm: Option<Field<BTreeMap<String, Field<String>>>>,
    #[serde(rename = "Subcellular main location")]
    pub main: Option<Field<Vec<Field<String>>>>,
    #[serde(rename = "Subcellular additional location")]
    pub additional: Option<Field<Vec<Field<String>>>>,
}
impl Reply for Gene {
    fn check(&self) -> Result<(), BadReply> {
        self.ensembl.parse::<biodata::EnsemblGeneId>().map_err(|_| BadReply)?;
        Ok(())
    }
}
