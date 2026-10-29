use biodata::{Text, WikiPathwaysId};
use biomcp_fetch::{
    BadReply, KeyKind, Keys, Row, bundle_sha256, decode_bundle_document, encode_bundle_row,
};
use biomcp_types::WikiPathwaysCatalogReceipt;
use serde::{Deserialize, Serialize};
use std::collections::BTreeSet;
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Catalog {
    #[serde(rename = "pathwayInfo")]
    rows: Vec<Native>,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub(super) struct Native {
    pub id: WikiPathwaysId,
    url: Text,
    #[serde(deserialize_with = "crate::native_text::deserialize")]
    pub name: biodata::Text,
    pub species: Box<str>,
    pub revision: Text,
    authors: Box<str>,
    pub description: Box<str>,
    #[serde(rename = "citedIn")]
    pub cited_in: Box<str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    datanodes: Option<Box<str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    annotations: Option<Box<str>>,
}
#[derive(Debug, Deserialize, Serialize)]
pub(super) struct CatalogRow {
    pub native: Native,
    pub receipt: WikiPathwaysCatalogReceipt,
}
impl Row for CatalogRow {
    const MAX_BYTES: u64 = 8 * 1024 * 1024;
    fn decode_document(body: &[u8]) -> Result<Vec<String>, BadReply> {
        if body.len() as u64 > Self::MAX_BYTES {
            return Err(BadReply);
        }
        let catalog: Catalog = decode_bundle_document(body)?;
        if catalog.rows.is_empty() {
            return Err(BadReply);
        }
        let mut ids = BTreeSet::new();
        for row in &catalog.rows {
            if row.species.trim().is_empty()
                || !ids.insert(row.id.as_str())
                || row.url.as_str() != format!("https://www.wikipathways.org/instance/{}", row.id)
                || row.datanodes.is_some() != row.annotations.is_some()
            {
                return Err(BadReply);
            }
        }
        let sha256 = bundle_sha256(body);
        let receipt = WikiPathwaysCatalogReceipt {
            sha256: sha256.parse().map_err(|_| BadReply)?,
            bytes: body.len() as u64,
            revision: format!("sha256:{sha256}").parse().map_err(|_| BadReply)?,
            human_rows: catalog
                .rows
                .iter()
                .filter(|r| r.species.as_ref() == "Homo sapiens")
                .count() as u64,
            total_rows: catalog.rows.len() as u64,
        };
        catalog
            .rows
            .into_iter()
            .map(|native| {
                encode_bundle_row(&Self {
                    native,
                    receipt: receipt.clone(),
                })
            })
            .collect()
    }
    fn keys(&self) -> Keys {
        Keys {
            exact: vec![
                (KeyKind::Id, self.native.id.to_string().into()),
                (KeyKind::Band, "catalog".into()),
            ],
            words: vec![],
        }
    }
    fn check_part(&self, part: &str) -> Result<(), BadReply> {
        if (part == "search" && self.native.datanodes.is_some())
            || (part == "get" && self.native.datanodes.is_none())
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
