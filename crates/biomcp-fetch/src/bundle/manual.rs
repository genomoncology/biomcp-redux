//! Read and publish dated manual imports without invoking any network path.
use super::{Bundle, BundleInfo, Find, Row, parse::ParsedRows, read, store};
use crate::{CacheError, Leg, RequestError};
use biomcp_types::Reason;
use serde::Serialize;

impl Leg<'_> {
    /// Read matching imported rows and their date in one store snapshot, if present.
    /// This path never synchronizes, including in live, fresh and replay modes.
    pub fn manual_rows<R: Row>(
        &self,
        bundle: &Bundle<R>,
        find: Find<'_>,
    ) -> Result<Option<(Vec<R>, BundleInfo)>, RequestError> {
        self.bundle_source(bundle)?;
        let Some(cache) = self.fetch.cache.as_deref() else {
            return Ok(None);
        };
        let Some(stored) = cache.stored_bundle(bundle.name, find, self.budget())? else {
            return Ok(None);
        };
        let mut info = store::info(bundle, &stored.meta);
        info.data_date = read::data_date::<R>(stored.sample)?;
        Ok(Some((read::decode(stored.rows)?, info)))
    }

    /// Atomically replace an imported catalog with a strictly newer dated release.
    /// The supplied rows have already been validated by their source adapter.
    pub fn import_manual_rows<R: Row + Serialize>(
        &self,
        bundle: &Bundle<R>,
        rows: &[R],
        source_bytes: u64,
        bundled_date: &str,
    ) -> Result<Option<BundleInfo>, RequestError> {
        self.bundle_source(bundle)?;
        let date = rows
            .first()
            .and_then(Row::data_date)
            .ok_or_else(|| self.failure(Reason::BadReply, None))?;
        if date <= bundled_date {
            return Ok(None);
        }
        let mut parsed = ParsedRows::default();
        for (index, row) in rows.iter().enumerate() {
            if row.data_date() != Some(date) {
                return Err(self.failure(Reason::BadReply, None).into());
            }
            let json = serde_json::to_string(row).map_err(|_| CacheError::Encoding)?;
            parsed.accept(json, row.keys(), index + 1);
        }
        if !parsed.rejections.is_empty() {
            return Err(self.failure(Reason::BadReply, None).into());
        }
        let meta = store::Meta {
            rejections: Vec::new(),
            bytes: source_bytes,
            rows: rows.len() as u64,
            fresh_until: i64::MAX,
            etag: None,
            modified: None,
            content_revision: None,
            retrieved_at: None,
        };
        let imported = self.bundle_cache()?.publish_if_newer(
            bundle,
            None,
            &meta,
            Some(&parsed),
            self.budget(),
            Some(date),
        )?;
        Ok(imported.then(|| {
            let mut info = store::info(bundle, &meta);
            info.data_date = Some(date.into());
            info
        }))
    }
}
