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

#[test]
fn constraint_numbers_refuse_non_finite_values_and_empty_sections_omit_record() {
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        assert!(biodata::FiniteNumber::try_from(value).is_err());
    }
    for value in [0.0, -2.5, f64::MAX] {
        let checked = biodata::FiniteNumber::try_from(value).unwrap();
        assert_eq!(f64::from(checked), value);
        let wire = serde_json::to_string(&checked).unwrap();
        assert_eq!(
            serde_json::from_str::<biodata::FiniteNumber>(&wire).unwrap(),
            checked
        );
    }
    assert_eq!(
        serde_json::to_value(Section::<Text>::Empty {
            source: Source::Gnomad,
            record: None
        })
        .unwrap(),
        serde_json::json!({"state":"empty","source":"gnomad"})
    );
}
