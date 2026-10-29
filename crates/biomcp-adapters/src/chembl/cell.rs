//! Exact source-local cell metadata and assay counts.
use super::SPEC;
use crate::native_text::NativeText;
use biodata::{CellosaurusAccession, ChemblCellId};
use biomcp_fetch::{Arg, BadReply, Freshness, Leg, Reply, ReplyMetadata, RequestError};
use biomcp_types::{CellLineChemblRecord, CellLineRelease};
use serde::Deserialize;
const CELLS: biomcp_fetch::Endpoint = SPEC.get("/cell_line.json").fresh(Freshness::Days(1));
const ASSAYS: biomcp_fetch::Endpoint = SPEC.get("/assay.json").fresh(Freshness::Days(1));
const STATUS: biomcp_fetch::Endpoint = SPEC.get("/status.json").fresh(Freshness::Days(1));
/// Cell-specific native paging; drug endpoint validation remains independent.
#[derive(Debug, Deserialize)]
struct PageMeta {
    total_count: u64,
    offset: u32,
    limit: u32,
    next: Option<biodata::Text>,
    previous: Option<biodata::Text>,
}
impl PageMeta {
    fn check(&self, count: usize, limit: u32, offset: u32) -> Result<(), BadReply> {
        let expected = self
            .total_count
            .saturating_sub(u64::from(offset))
            .min(u64::from(limit));
        let more = u64::from(offset) + u64::from(limit) < self.total_count;
        if (1..=25).contains(&limit)
            && self.limit == limit
            && self.offset == offset
            && count as u64 == expected
            && self.next.is_some() == more
            && (offset != 0 || self.previous.is_none())
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
#[derive(Debug, Deserialize)]
struct Cells {
    cell_lines: Vec<Cell>,
    page_meta: PageMeta,
}
#[derive(Debug, Deserialize)]
struct Cell {
    cell_chembl_id: ChemblCellId,
    cell_id: u64,
    cellosaurus_id: CellosaurusAccession,
    cell_name: Option<NativeText>,
    cell_description: Option<NativeText>,
    cell_source_organism: Option<NativeText>,
    cell_source_tax_id: Option<u32>,
    cell_source_tissue: Option<NativeText>,
    clo_id: Option<biodata::Text>,
    efo_id: Option<biodata::Text>,
}
impl Reply for Cells {
    fn check(&self) -> Result<(), BadReply> {
        if self
            .cell_lines
            .iter()
            .all(|r| r.cell_id > 0 && r.cell_source_tax_id != Some(0))
        {
            Ok(())
        } else {
            Err(BadReply)
        }
    }
}
impl From<Cell> for CellLineChemblRecord {
    fn from(r: Cell) -> Self {
        Self {
            id: r.cell_chembl_id,
            cell_id: r.cell_id,
            cellosaurus_id: r.cellosaurus_id,
            name: r.cell_name.map(Into::into),
            description: r.cell_description.map(Into::into),
            organism: r.cell_source_organism.map(Into::into),
            taxon: r.cell_source_tax_id,
            tissue: r.cell_source_tissue.map(Into::into),
            clo: r.clo_id,
            efo: r.efo_id,
        }
    }
}
/// Read one native cell metadata page and check every exact accession before cache admission.
pub fn cells(
    leg: &Leg<'_>,
    id: &CellosaurusAccession,
    limit: u32,
    offset: u32,
) -> Result<(Vec<CellLineChemblRecord>, u64, ReplyMetadata), RequestError> {
    let text = id.to_string();
    let (reply, metadata) = leg.json_received_checked(
        &CELLS,
        &[
            ("cellosaurus_id", Arg::Text(&text)),
            ("limit", Arg::Num(limit.into())),
            ("offset", Arg::Num(offset.into())),
        ],
        |r: &Cells| {
            r.page_meta.check(r.cell_lines.len(), limit, offset)?;
            if r.cell_lines.iter().all(|r| &r.cellosaurus_id == id) {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    Ok((
        reply.cell_lines.into_iter().map(Into::into).collect(),
        reply.page_meta.total_count,
        metadata,
    ))
}
#[derive(Debug, Deserialize)]
struct Assays {
    assays: Vec<Assay>,
    page_meta: PageMeta,
}
#[derive(Debug, Deserialize)]
struct Assay {
    cell_chembl_id: ChemblCellId,
}
impl Reply for Assays {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// Read only the native assay total; validate every returned cell identity first.
pub fn assay_count(leg: &Leg<'_>, id: &ChemblCellId) -> Result<(u64, ReplyMetadata), RequestError> {
    let text = id.to_string();
    let (reply, metadata) = leg.json_received_checked(
        &ASSAYS,
        &[
            ("cell_chembl_id", Arg::Text(&text)),
            ("limit", Arg::Num(1)),
            ("offset", Arg::Num(0)),
        ],
        |r: &Assays| {
            r.page_meta.check(r.assays.len(), 1, 0)?;
            if (r.page_meta.total_count == 0) == r.assays.is_empty()
                && r.assays.iter().all(|r| &r.cell_chembl_id == id)
            {
                Ok(())
            } else {
                Err(BadReply)
            }
        },
    )?;
    Ok((reply.page_meta.total_count, metadata))
}
#[derive(Debug, Deserialize)]
struct Release {
    chembl_db_version: biodata::Text,
    chembl_release_date: biodata::PartialDate,
}
impl Reply for Release {
    fn check(&self) -> Result<(), BadReply> {
        Ok(())
    }
}
/// Read publisher release independently from data retrieval time.
pub fn release(leg: &Leg<'_>) -> Result<CellLineRelease, RequestError> {
    let r: Release = leg.json(&STATUS, &[])?;
    Ok(CellLineRelease {
        version: r.chembl_db_version,
        updated: r.chembl_release_date,
    })
}
