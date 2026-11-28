//! Stored Purple Book rows share lookup keys between matching and indexing.
use biomcp_fetch::{KeyKind, Keys, Row};
use biomcp_types::{PurpleBookLicense, PurpleBookPresentation};
use serde::{Deserialize, Serialize};
use std::{borrow::Cow, collections::BTreeSet};

#[derive(Debug, Deserialize, Serialize)]
pub(super) struct Stored {
    pub(super) date: Box<str>,
    pub(super) row: PurpleBookPresentation,
}
impl Stored {
    fn product_keys(&self) -> impl Iterator<Item = Cow<'_, str>> {
        let row = &self.row;
        [row.name.as_ref(), row.proper_name.as_ref()]
            .into_iter()
            .filter(|key| !key.is_empty())
            .map(Cow::Borrowed)
            .chain(std::iter::once_with(move || {
                Cow::Owned(row.bla.to_string())
            }))
    }

    fn reference_keys(&self) -> impl Iterator<Item = Cow<'_, str>> {
        let row = &self.row;
        [
            row.reference_name.as_ref(),
            row.reference_proper_name.as_ref(),
        ]
        .into_iter()
        .filter(|key| !key.is_empty())
        .map(Cow::Borrowed)
        .chain(
            row.reference_bla
                .iter()
                .chain(row.reference_bla_candidates.iter())
                .map(|bla| Cow::Owned(bla.to_string())),
        )
    }

    pub(super) fn matches_product(&self, query: &str) -> bool {
        self.product_keys()
            .any(|key| key.eq_ignore_ascii_case(query))
    }

    pub(super) fn matches_reference(&self, query: &str) -> bool {
        self.row.license_type != PurpleBookLicense::Standalone
            && self
                .reference_keys()
                .any(|key| key.eq_ignore_ascii_case(query))
    }
}
impl Row for Stored {
    fn keys(&self) -> Keys {
        Keys {
            exact: self
                .product_keys()
                .chain(self.reference_keys())
                .map(|key| key.to_ascii_lowercase())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .map(|key| (KeyKind::Alias, key.into()))
                .collect(),
            words: Vec::new(),
        }
    }
    fn data_date(&self) -> Option<&str> {
        Some(&self.date)
    }
}
