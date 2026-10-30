//! Feature cell-line-chembl: exact source metadata and separately requested assay totals.
use super::{
    Environment, Stub, Trial, Value, Write, canonical, ceiling, fs, invoke, json, path, setup,
};
use crate::runner::transcript::trial as transcript_trial;

pub(super) mod surface;
use surface::{compared, invoke_measured, measure_bytes, measure_mcp, parity};
