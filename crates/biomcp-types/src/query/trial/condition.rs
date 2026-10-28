use super::{TrialSearch, TrialSource};

impl TrialSearch {
    /// Whether the source must search the registered condition phrase without expansion.
    pub fn exact_condition(&self) -> bool { self.0.exact_condition }

    /// Offer an explicit source-field restriction with all other filters retained.
    pub fn exact_condition_search(&self) -> Option<Self> {
        if self.source() != TrialSource::Ctgov || self.condition().is_none() || self.exact_condition() { return None; }
        let mut next = self.clone();
        next.0.exact_condition = true;
        next.0.next_page = None;
        Some(next)
    }
}

pub(super) fn valid_id(id: &biodata::Curie, source: TrialSource) -> bool {
    let value = id.to_string();
    let Some((namespace, local)) = value.split_once(':') else {
        return false;
    };
    match namespace {
        "MONDO" | "DOID" => !local.is_empty() && local.bytes().all(|v| v.is_ascii_digit()),
        "NCIT" => {
            source == TrialSource::Nci
                && local
                    .strip_prefix('C')
                    .is_some_and(|v| !v.is_empty() && v.bytes().all(|v| v.is_ascii_digit()))
        }
        _ => false,
    }
}
