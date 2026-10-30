use super::*;

// Disease has one source, so the shared item owner proves secondary failures.
#[test]
fn item_rules_keep_optional_problems_and_preserve_secondary_failures() {
    for (found, identity_failed, unavailable, secondary, dropped, expected) in [
        (true, false, false, SourceState::Ok, false, Status::Complete),
        (false, false, false, SourceState::Ok, false, Status::Absent),
        (false, true, false, SourceState::Ok, false, Status::Failed),
        (true, false, true, SourceState::Ok, false, Status::Partial),
        (
            true,
            false,
            false,
            SourceState::Failed,
            false,
            Status::Partial,
        ),
        (
            true,
            false,
            false,
            SourceState::Stale,
            false,
            Status::Partial,
        ),
        (true, false, false, SourceState::Ok, true, Status::Partial),
    ] {
        let mut out = empty(QueryBody::DiseaseCard(None));
        out.sources.push(biomcp_adapters::registry::outcome(
            Source::MyDisease,
            SourceState::Ok,
            None,
        ));
        out.sources.push(biomcp_adapters::registry::outcome(
            Source::Monarch,
            secondary,
            None,
        ));
        out.problems.push(Problem {
            source: Source::MyDisease,
            record: None,
            field: "label".into(),
            code: biomcp_types::ProblemCode::ValueUnknown,
        });
        out.dropped = dropped;
        item(&mut out, found, identity_failed, unavailable);
        assert_eq!(
            out.status(),
            expected,
            "identity={found}/{identity_failed}, section={unavailable}, secondary={secondary:?}, dropped={dropped}"
        );
    }
}
