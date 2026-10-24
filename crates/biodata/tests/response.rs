//! Section state wire contract before the first section consumer arrives.
use biodata::{Reason, Section, Source, SourceRecord, Text};
#[test]
fn every_section_state_round_trips_without_conflating_data_and_failure() {
    let states = [
        Section::Data {
            source: Source::Civic,
            data: Text::new("answer").unwrap(),
        },
        Section::Empty {
            source: Source::Civic,
            record: Some(SourceRecord {
                id: Text::new("1").unwrap(),
                url: None,
            }),
        },
        Section::Empty {
            source: Source::Civic,
            record: None,
        },
        Section::Unavailable {
            source: Source::Civic,
            reason: Reason::Unavailable,
        },
        Section::Skipped {
            source: Source::Civic,
            reason: Reason::NoKey,
        },
        Section::NotApplicable,
    ];
    for state in states {
        let value = serde_json::to_value(&state).unwrap();
        assert_eq!(
            serde_json::from_value::<Section<Text>>(value).unwrap(),
            state
        );
    }
    assert!(serde_json::from_value::<Section<Text>>(serde_json::json!({"state":"unavailable","source":"civic","reason":"unavailable","data":"bad"})).is_err());
}
