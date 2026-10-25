//! Source-owned disease facts without identity choice or cross-provider fallback.
pub mod mydisease;
/// OLS source-owned identity and discovery outcomes.
pub mod ols;
mod page;
use crate::{Ctx, Outcome};
use biomcp_types::{DiseaseGet, DiseaseSearch, DiseaseSource};
/// Discover one page from the explicitly selected source.
pub fn search(cx: &Ctx<'_>, query: &DiseaseSearch) -> Outcome {
    ols::search(cx, query)
}
/// Read one exact concept or selected source page.
pub fn get(cx: &Ctx<'_>, query: &DiseaseGet) -> Outcome {
    match query.source() {
        DiseaseSource::MyDisease => mydisease::get(cx, query),
        _ => ols::get(cx, query),
    }
}
