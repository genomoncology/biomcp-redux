//! Reviewed explicit noncommercial study wording; unknown studies remain enabled.
/// Exact catalog description for a study with an explicit noncommercial license.
/// Refresh this checked-in list when reviewing a changed public study catalog.
pub fn commercial_restriction(id: &biodata::StudyId) -> Option<&'static str> {
    include_str!("commercial-restrictions.tsv")
        .lines()
        .filter_map(|line| line.split_once('\t'))
        .find_map(|(study, wording)| (study == id.as_str()).then_some(wording))
}
