//! A search row's status and dates, with the same qualifier the record view shows.
use super::Status;
use crate::{ctgov::detail::RawDate, mapped::Problems, raw::Field};
use biodata::{PartialDate, TrialDate, TrialStatus};

/// A row's status and dates; a missing status module still reports its two required fields.
pub(super) struct Dates {
    pub(super) status: Option<TrialStatus>,
    pub(super) updated: Option<PartialDate>,
    pub(super) start: Option<TrialDate>,
    pub(super) primary_completion: Option<TrialDate>,
}
impl Dates {
    pub(super) fn read(module: Option<Field<Status>>, problems: &mut Problems) -> Self {
        let Some(status) = problems.field("statusModule", module) else {
            return Self {
                status: problems.required("overallStatus", None),
                updated: problems.required("lastUpdatePostDateStruct.date", None),
                start: None,
                primary_completion: None,
            };
        };
        let overall = problems.required_field("overallStatus", status.overall_status);
        let updated = problems
            .field(
                "lastUpdatePostDateStruct",
                status.last_update_post_date_struct,
            )
            .and_then(|d| problems.field("lastUpdatePostDateStruct.date", d.date));
        Self {
            status: overall,
            updated: problems.required("lastUpdatePostDateStruct.date", updated),
            start: RawDate::field("startDateStruct", status.start_date_struct, problems),
            primary_completion: RawDate::field(
                "primaryCompletionDateStruct",
                status.primary_completion_date_struct,
                problems,
            ),
        }
    }
}
