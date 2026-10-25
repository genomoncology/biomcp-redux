//! Source-owned disease facts without identity choice or cross-provider fallback.
pub mod ols;
use crate::{Ctx, Outcome};
use biomcp_types::{DiseaseGet, DiseaseSearch};
/// Discover one page from the explicitly selected source.
pub fn search(cx: &Ctx<'_>, query: &DiseaseSearch) -> Outcome {
    ols::search(cx, query)
}
/// Read one exact concept or selected source page.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    ols::get(cx, query)
}
