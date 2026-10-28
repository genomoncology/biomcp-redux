//! Existing trial case registration, in stable registration order.
use super::{
    Trial, nci, nci_walk, trial_condition, trial_documents, trial_get, trial_sites, trial_tables,
    trial_wider,
};

pub(super) fn trials() -> Vec<Trial> {
    [
        trial_tables::trials(),
        super::trial_bad_values::trials(),
        trial_wider::trials(),
        trial_get::trials(),
        super::trial_eligibility::trials(),
        nci::trials(),
        nci_walk::trials(),
        trial_condition::trials(),
        trial_documents::trials(),
        trial_sites::trials(),
    ]
    .into_iter()
    .flatten()
    .collect()
}
