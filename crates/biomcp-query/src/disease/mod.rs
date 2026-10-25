//! Source-owned disease facts without identity choice or cross-provider fallback.
/// MyDisease exact annotation outcomes.
pub mod mydisease;
/// OLS source-owned identity and discovery outcomes.
pub mod ols;
mod page;
/// Native SEER site and statistics pages.
pub mod seer;
use crate::{Ctx, Outcome};
use biomcp_types::{DiseaseGet, DiseaseSearch, DiseaseSource};
/// Discover one page from the explicitly selected source.
pub fn search(cx: &Ctx<'_>, query: &DiseaseSearch) -> Outcome {
    match query.source() {
        biomcp_types::DiseaseSource::Seer => seer::search(cx, query),
        _ => ols::search(cx, query),
    }
}
/// Read one exact concept or selected source page.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    match query.source() {
        DiseaseSource::MyDisease => mydisease::get(cx, query),
        biomcp_types::DiseaseSource::Seer => seer::get(cx, query),
        _ => ols::get(cx, query),
    }
}
