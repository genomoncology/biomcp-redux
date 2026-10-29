use super::{SPEC, raw::CatalogRow};
use biodata::{Pathway, PathwayId, PathwaySpecies, WikiPathwaysId};
use biomcp_fetch::{
    BundleInfo, BundlePart, BundleSet, Fetch, Find, Freshness, KeyKind, Leg, RequestError,
    RowFormat,
};
use biomcp_types::{PathwayCard, PathwayHeader, PathwaySection, WikiPathwaysSync};
use std::marker::PhantomData;
const CATALOGS: BundleSet<CatalogRow> = BundleSet {
    spec: &SPEC,
    name: "wikipathways",
    parts: &[
        BundlePart {
            name: "search",
            path: "/json/findPathwaysByText.json",
            format: None,
        },
        BundlePart {
            name: "get",
            path: "/json/getPathwayInfo.json",
            format: None,
        },
    ],
    format: RowFormat::Document {
        media: biomcp_fetch::DocumentMedia::Json,
    },
    fresh: Freshness::Days(7),
    row: PhantomData,
};
fn bad() -> RequestError {
    biomcp_fetch::SourceError {
        source: biomcp_types::Source::WikiPathways,
        reason: biomcp_types::Reason::BadReply,
        status: None,
        retry_after: None,
    }
    .into()
}
/// Download both metadata catalogs and publish their checked pair atomically.
pub fn sync(fetch: &Fetch) -> Result<crate::Mapped<WikiPathwaysSync>, RequestError> {
    let info = fetch.sync_set(&CATALOGS)?;
    let leg = fetch.leg(&SPEC, fetch.deadline(SPEC.deadline));
    let (rows, _) = leg.stored_set_rows(&CATALOGS, Find::Exact(KeyKind::Band, "catalog"))?;
    let search = rows
        .iter()
        .find(|r| r.part.as_ref() == "search")
        .ok_or_else(bad)?
        .value
        .receipt
        .clone();
    let get = rows
        .iter()
        .find(|r| r.part.as_ref() == "get")
        .ok_or_else(bad)?
        .value
        .receipt
        .clone();
    let problems = crate::mapped::bundle_problems(SPEC.source, &info);
    Ok(crate::Mapped {
        problems,
        value: WikiPathwaysSync {
            search,
            get,
            downloaded_bytes: info.downloaded_bytes,
        },
    })
}
/// Search only native titles in original catalog order, with exact source species admission.
pub fn search(leg: &Leg<'_>, text: &str) -> Result<(Vec<PathwayHeader>, BundleInfo), RequestError> {
    let (rows, info) = leg.stored_set_rows(&CATALOGS, Find::Exact(KeyKind::Band, "catalog"))?;
    let needle = text.to_lowercase();
    let items = rows
        .into_iter()
        .filter(|r| {
            r.part.as_ref() == "search"
                && r.value.native.species.as_ref() == "Homo sapiens"
                && r.value
                    .native
                    .name
                    .as_str()
                    .to_lowercase()
                    .contains(&needle)
        })
        .map(|r| {
            let row = r.value.native;
            PathwayHeader {
                id: PathwayId::Wikipathways(row.id),
                name: row.name,
                species: PathwaySpecies::Human,
                version: None,
                revision: Some(row.revision),
            }
        })
        .collect();
    Ok((items, info))
}
/// Read an exact native human identity from the stored get catalog without source requests.
pub fn card(
    leg: &Leg<'_>,
    id: &WikiPathwaysId,
    summary: bool,
) -> Result<Option<PathwayCard>, RequestError> {
    let (rows, _) = leg.stored_set_rows(&CATALOGS, Find::Exact(KeyKind::Id, &id.to_string()))?;
    let row = rows.into_iter().find(|r| {
        r.part.as_ref() == "get"
            && r.value.native.id == *id
            && r.value.native.species.as_ref() == "Homo sapiens"
    });
    Ok(row.map(|r| {
        let row = r.value.native;
        PathwayCard {
            pathway: Pathway {
                id: PathwayId::Wikipathways(row.id),
                name: row.name,
                species: PathwaySpecies::Human,
                description: summary.then_some(row.description),
                summations: None,
                references: None,
                cited_in: Some(row.cited_in),
                version: None,
                revision: Some(row.revision),
            },
            release: None,
            description_truncated: summary.then_some(true),
            sections: vec![PathwaySection::Summary, PathwaySection::Genes].into(),
            genes: None,
            events: None,
        }
    }))
}

/// Inspect this source's validated local catalog without reading rows or synchronizing.
pub(crate) fn bundle_state(leg: &Leg<'_>) -> Result<biomcp_types::LocalBundleState, RequestError> {
    CATALOGS.state(leg)
}
