//! Complete multi-file publication through the ordinary checked-byte transport.
use super::{Bundle, BundleInfo, Row, RowFormat, parse, store};
use crate::{Fetch, Freshness, RequestError, SourceSpec};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use std::marker::PhantomData;

/// One independently published part of a complete replacement set.
#[derive(Clone, Copy, Debug)]
pub struct BundlePart {
    /// Stable source-native part identifier.
    pub name: &'static str,
    /// Exact path under the source declaration.
    pub path: &'static str,
    /// A part-specific encoding; absent uses the set encoding.
    pub format: Option<RowFormat>,
}
/// A source requires every declared part before any row becomes visible.
#[derive(Debug)]
pub struct BundleSet<R: Row + 'static> {
    /// Canonical source and policy declaration.
    pub spec: &'static SourceSpec,
    /// Stable store name for the whole set.
    pub name: &'static str,
    /// Native part order, also defining row order.
    pub parts: &'static [BundlePart],
    /// Common native row encoding.
    pub format: RowFormat,
    /// Client freshness policy, independent of publication cadence.
    pub fresh: Freshness,
    /// Decoded native row type.
    pub row: PhantomData<R>,
}
/// A checked source row with its original location in the replacement set.
#[derive(Debug, Deserialize)]
pub struct SetRow<R> {
    /// Native part identifier.
    pub part: Box<str>,
    /// One-based logical row number, counting the header.
    pub row: u64,
    /// Original decoded native record.
    pub value: R,
}
impl<R: Row> Row for SetRow<R> {
    fn keys(&self) -> super::Keys {
        self.value.keys()
    }
}
impl<R: Row> BundleSet<R> {
    /// Inspect the complete stored set without downloading any part.
    pub fn state(
        &self,
        leg: &crate::Leg<'_>,
    ) -> Result<biomcp_types::LocalBundleState, RequestError> {
        leg.stored_bundle_state(&self.stored())
    }

    pub(super) fn stored(&self) -> Bundle<SetRow<R>> {
        Bundle {
            spec: self.spec,
            name: self.name,
            path: "",
            format: self.format,
            fresh: self.fresh,
            row: PhantomData,
        }
    }
}
impl Fetch {
    /// Download and check every part, then atomically replace the complete stored set.
    pub fn sync_set<R: Row>(&self, set: &BundleSet<R>) -> Result<BundleInfo, RequestError> {
        let leg = self.sync_leg(set.spec);
        let mut info = leg.publish_set(set, None)?;
        info.downloaded_bytes = leg.finish().downloaded_bytes;
        Ok(info)
    }
}
impl crate::Leg<'_> {
    /// Publish a whole set within this leg's deadline. An automatic first read passes its
    /// byte ceiling; a set over it, or out of time, stays missing and answers `syncing`.
    fn publish_set<R: Row>(
        &self,
        set: &BundleSet<R>,
        auto: Option<&super::first::Size>,
    ) -> Result<BundleInfo, RequestError> {
        let leg = self;
        let bundle = set.stored();
        leg.bundle_source(&bundle)?;
        let cache = leg.bundle_cache()?;
        let mut complete = Vec::new();
        let mut rejections = Vec::new();
        let mut revision = Sha256::new();
        let mut bytes = 0_u64;
        // Streamed parts have their own ceiling; the shared one covers every other part.
        let mut held = 0_u64;
        let max = auto.map(|_| {
            self.fetch
                .config
                .auto_sync_max_bytes
                .min(super::MAX_BUNDLE_BYTES)
        });
        for (index, part) in set.parts.iter().enumerate() {
            let format = part.format.unwrap_or(set.format);
            let streamed = matches!(format, RowFormat::StreamedXml { .. });
            let ceiling = max.map(|max| max.saturating_sub(bytes));
            if ceiling == Some(0) {
                return Err(leg.failure(biomcp_types::Reason::TooLarge, None).into());
            }
            let total = auto.filter(|_| index + 1 == set.parts.len());
            let body = part_body(leg, set, part, format, ceiling, |length| {
                if let Some(size) = total {
                    size.set(bytes.checked_add(length));
                }
            })?;
            bytes = bytes.saturating_add(body.len() as u64);
            if !streamed {
                held = held.saturating_add(body.len() as u64);
            }
            if held > super::MAX_BUNDLE_BYTES {
                return Err(leg
                    .failure(biomcp_types::Reason::BadReply, Some(200))
                    .into());
            }
            revision.update((part.path.len() as u64).to_be_bytes());
            revision.update(part.path.as_bytes());
            revision.update((body.len() as u64).to_be_bytes());
            revision.update(&body);
            let rows = parse::part_rows::<R>(&body, format, part.name)
                .map_err(|_| leg.failure(biomcp_types::Reason::BadReply, Some(200)))?;
            if streamed && rows.is_empty() {
                return Err(leg
                    .failure(biomcp_types::Reason::BadReply, Some(200))
                    .into());
            }
            rejections.extend(rows.rejections.into_iter().map(|mut rejection| {
                rejection.reason = format!("{}: {}", part.name, rejection.reason);
                rejection
            }));
            append_part(&mut complete, part, rows.rows)?;
        }
        if set.parts.is_empty() || leg.budget().expired() {
            return Err(leg.failure(biomcp_types::Reason::Syncing, None).into());
        }
        let metadata = store::Meta {
            rejections,
            bytes,
            rows: complete.len() as u64,
            fresh_until: crate::cache::now().saturating_add(
                i64::try_from(set.fresh.seconds().unwrap_or(0)).unwrap_or(i64::MAX),
            ),
            // Publisher HTTP validators never hold a computed set digest.
            etag: None,
            modified: None,
            content_revision: Some(format!(
                "sha256:{}",
                super::digest_hex(&revision.finalize())
            )),
            retrieved_at: Some(crate::cache::now()),
        };
        cache.publish(&bundle, None, &metadata, Some(&complete), leg.budget())?;
        Ok(store::info(&bundle, &metadata))
    }
}
/// Read one part's checked body. A streamed part is decoded once, after download,
/// instead of once in the transport check and again for its rows.
fn part_body<R: Row>(
    leg: &crate::Leg<'_>,
    set: &BundleSet<R>,
    part: &BundlePart,
    format: RowFormat,
    ceiling: Option<u64>,
    part_length: impl Fn(u64),
) -> Result<Vec<u8>, RequestError> {
    let limit = match format {
        RowFormat::NativeCsv { max_bytes } => max_bytes.min(super::MAX_BUNDLE_BYTES),
        RowFormat::StreamedXml { max_bytes, .. } => max_bytes.min(super::MAX_STREAMED_BYTES),
        _ => R::MAX_BYTES.min(super::MAX_BUNDLE_BYTES),
    }
    .min(ceiling.unwrap_or(u64::MAX));
    let endpoint = set.spec.get(part.path).bytes(limit).fresh(Freshness::Never);
    let check = |body: &[u8]| {
        let parsed = parse::part_rows::<R>(body, format, part.name)
            .map_err(|_| biomcp_types::Reason::BadReply)?;
        if parsed.is_empty() {
            return Err(biomcp_types::Reason::BadReply);
        }
        Ok(())
    };
    let read = match format {
        RowFormat::StreamedXml { .. } => crate::Read::bytes().set_part(),
        _ => crate::Read::bytes().set_part().validate_body(&check),
    };
    let read = match format {
        RowFormat::Document { media } => read.document_media(media),
        _ => read,
    };
    let mut read = read;
    read.part_length = ceiling.map(|_| &part_length as &dyn Fn(u64));
    let body = leg
        .call(&endpoint, crate::Input::Params(&[]), read)?
        .value
        .0;
    part_length(body.len() as u64);
    Ok(body)
}
fn append_part(
    complete: &mut Vec<parse::Parsed>,
    part: &BundlePart,
    rows: Vec<parse::Parsed>,
) -> Result<(), RequestError> {
    for row in rows {
        let value: serde_json::Value =
            serde_json::from_str(&row.row).map_err(|_| crate::CacheError::Encoding)?;
        let document = serde_json::json!({"part":part.name,"row":row.number,"value":value});
        complete.push(parse::Parsed {
            number: row.number,
            row: document.to_string(),
            keys: row.keys,
        });
    }
    Ok(())
}
impl crate::Leg<'_> {
    /// Read one validated complete set; a stored set is never refreshed on read. With no
    /// set and no publisher, a live reader fetches it once within `auto_sync_max_bytes`.
    pub fn stored_set_rows<R: Row>(
        &self,
        set: &BundleSet<R>,
        find: super::Find<'_>,
    ) -> Result<(Vec<SetRow<R>>, BundleInfo), RequestError> {
        self.name_busy(self.read_stored_rows(&set.stored(), find, |size| {
            self.publish_set(set, Some(size)).map(drop)
        }))
    }
}
