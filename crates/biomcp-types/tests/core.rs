//! Public library tables own the core value and JSON contracts.
use biodata::{Error, Text};
use biomcp_types::{Problem, ProblemCode, Reason, Source};
use serde::{Serialize, de::DeserializeOwned};
use std::{fmt::Debug, str::FromStr};

type Check = Result<(), Box<dyn std::error::Error>>;

fn canonical_names<T>(rows: &[(T, &str)], error: Error) -> Check
where
    T: Copy + Debug + Eq + FromStr<Err = Error> + ToString + Serialize + DeserializeOwned,
{
    for &(value, name) in rows {
        assert_eq!(
            value.to_string(),
            name,
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        assert_eq!(
            name.parse::<T>()?,
            value,
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        let json = serde_json::to_string(&value)?;
        assert_eq!(
            json,
            format!("\"{name}\""),
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        assert_eq!(
            serde_json::from_str::<T>(&json)?,
            value,
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
    }
    for input in ["", " ", "distinctive-rejected-input", "HGNC", " hgnc "] {
        let parsed = input.parse::<T>().err().ok_or("accepted an unknown name")?;
        assert_eq!(
            parsed, error,
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        assert!(
            !parsed.to_string().contains("distinctive-rejected-input"),
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        assert!(
            !format!("{parsed:?}").contains("distinctive-rejected-input"),
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        let json = serde_json::to_string(input)?;
        let decoded = serde_json::from_str::<T>(&json)
            .err()
            .ok_or("decoded an unknown name")?;
        assert!(
            decoded.to_string().contains(error.code()),
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
        assert!(
            !decoded.to_string().contains("distinctive-rejected-input"),
            "Closed vocabularies must preserve canonical names and reject other forms"
        );
    }
    Ok(())
}

#[test]
fn source_names_round_trip_and_reject_unknown_names() -> Check {
    canonical_names(
        &[
            (Source::Hgnc, "hgnc"),
            (Source::MyGene, "mygene"),
            (Source::PubMed, "pubmed"),
            (Source::EuropePmc, "europepmc"),
            (Source::PubTator, "pubtator"),
            (Source::ClinicalTrialsGov, "ctgov"),
            (Source::Civic, "civic"),
        ],
        Error::SourceUnknown,
    )?;
    assert_eq!(Error::SourceUnknown.code(), "source.unknown");
    Ok(())
}

#[test]
fn reason_names_round_trip_and_reject_unknown_names() -> Check {
    canonical_names(
        &[
            (Reason::NotFound, "not_found"),
            (Reason::RateLimited, "rate_limited"),
            (Reason::Unavailable, "unavailable"),
            (Reason::Rejected, "rejected"),
            (Reason::BadReply, "bad_reply"),
            (Reason::DeadlineExceeded, "deadline_exceeded"),
            (Reason::Offline, "offline"),
            (Reason::NoKey, "no_key"),
            (Reason::Disabled, "disabled"),
            (Reason::ReplayMiss, "replay_miss"),
            (Reason::Syncing, "syncing"),
        ],
        Error::ReasonUnknown,
    )?;
    assert_eq!(Error::ReasonUnknown.code(), "reason.unknown");
    Ok(())
}

#[test]
fn problem_codes_round_trip_and_reject_unknown_codes() -> Check {
    canonical_names(
        &[(ProblemCode::ValueUnknown, "value.unknown")],
        Error::ProblemCodeUnknown,
    )?;
    assert_eq!(Error::ProblemCodeUnknown.code(), "problem_code.unknown");
    Ok(())
}

#[test]
fn text_construction_and_json_share_trimming_and_validation() {
    for (input, expected) in [
        ("summary", Some("summary")),
        (" \n summary \t", Some("summary")),
        ("\u{2003}HER-2/neu α\u{2003}", Some("HER-2/neu α")),
        ("a \n b", Some("a \n b")),
        ("", None),
        (" \n\t\u{2003}", None),
    ] {
        let direct = Text::new(input);
        let parsed = input.parse::<Text>();
        let converted = Text::try_from(input.to_owned());
        assert_eq!(direct, parsed);
        assert_eq!(direct, converted);
        let json = serde_json::to_string(input).unwrap();
        let decoded = serde_json::from_str::<Text>(&json);
        if let Some(expected) = expected {
            let value = direct.unwrap();
            assert_eq!(value.as_str(), expected);
            assert_eq!(value.to_string(), expected);
            assert_eq!(value.to_string().parse::<Text>().unwrap(), value);
            assert_eq!(decoded.unwrap(), value);
            let json = serde_json::to_string(&value).unwrap();
            assert_eq!(json, serde_json::to_string(expected).unwrap());
        } else {
            let error = direct.unwrap_err();
            assert_eq!(error, Error::TextEmpty);
            assert_eq!(error.code(), "text.empty");
            assert!(decoded.unwrap_err().to_string().contains(error.code()));
        }
    }
    fn standard_error(error: &dyn std::error::Error) {
        assert_eq!(error.to_string(), "text.empty");
        assert!(error.source().is_none());
    }
    standard_error(&Error::TextEmpty);
}

#[test]
fn problems_round_trip_with_optional_record_and_reject_unknown_fields() {
    for record in [None, Some(Text::new("record-123").unwrap())] {
        let problem = Problem {
            source: Source::PubMed,
            record,
            field: "publication_type".into(),
            code: ProblemCode::ValueUnknown,
        };
        let json = serde_json::to_value(&problem).unwrap();
        assert_eq!(json["source"], "pubmed");
        assert_eq!(json["field"], "publication_type");
        assert_eq!(json["code"], "value.unknown");
        if problem.record.is_some() {
            assert_eq!(json["record"], "record-123");
        } else {
            assert!(json.get("record").is_none());
        }
        assert_eq!(
            serde_json::from_value::<Problem>(json.clone()).unwrap(),
            problem
        );
        let mut extra = json;
        extra["unexpected"] = true.into();
        assert!(serde_json::from_value::<Problem>(extra).is_err());
    }
    let invalid = r#"{"source":"pubmed","record":"  ","field":"pmid","code":"value.unknown"}"#;
    let error = serde_json::from_str::<Problem>(invalid).unwrap_err();
    assert!(error.to_string().contains("text.empty"));
}
