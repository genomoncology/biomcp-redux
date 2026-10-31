//! Hosted callers share only their effective credential's source pacing budget.
#[path = "support/answer.rs"]
mod answer;
use answer::Answer;
use biomcp_fetch::{
    Credential, Fetch, FetchConfig, KeyPlace, KeyUse, Mode, Quota, Rate, RateShare, RequestError,
    Secret, SecretName, Secrets, SourceKey, SourceSpec,
};
use biomcp_types::Reason;
use sha2::{Digest, Sha256};
use std::time::Duration;

type Check = Result<(), Box<dyn std::error::Error>>;

fn caller(fetch: &Fetch, key: Option<&str>) -> Fetch {
    let mut secrets = Secrets::default();
    if let Some(key) = key {
        secrets.insert(SecretName::NcbiApiKey, Secret::new(key));
    }
    fetch.with_hosted_secrets(secrets)
}

fn fixture(
    share: RateShare,
) -> Result<(tempfile::TempDir, Fetch, &'static SourceSpec), Box<dyn std::error::Error>> {
    let dir = tempfile::tempdir()?;
    let url = "https://example.invalid/answer";
    let hash = Sha256::digest([b"GET\0".as_slice(), url.as_bytes()].concat())
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect::<String>();
    std::fs::create_dir(dir.path().join("pubmed"))?;
    std::fs::write(
        dir.path().join("pubmed").join(format!("{hash}.json")),
        serde_json::to_vec(&serde_json::json!({
            "method":"GET","source":"pubmed","url":url,
            "exchanges":vec![serde_json::json!({"status":200,"content_type":"application/json","delay_ms":0,
                "body":{"kind":"json","value":{"answer":true}}}); 3]
        }))?,
    )?;
    let spec = Box::leak(Box::new(SourceSpec {
        source: biomcp_types::Source::PubMed,
        base: "https://example.invalid",
        destinations: biomcp_fetch::Destinations::Primary,
        user_agent: None,
        key: KeyUse::Optional(Credential {
            secret: SecretName::NcbiApiKey,
            place: KeyPlace::Header("x-api-key"),
            tier: biomcp_types::KeyTier::Fallback,
        }),
        quota: Box::leak(Box::new(Quota::new(
            SourceKey::declare("hosted-test"),
            Rate::per_second(2),
            Some(Rate::per_second(4)),
            1,
        ))),
        deadline: Duration::from_secs(10),
        terms: biomcp_fetch::Terms {
            name: "Synthetic test data",
            url: "https://example.test/terms",
            usage: biomcp_types::Use::Open,
        },
    }));
    let fetch = Fetch::open(FetchConfig {
        mode: Mode::Replay {
            dir: dir.path().into(),
        },
        rate_share: share,
        ..FetchConfig::default()
    })?;
    Ok((dir, fetch, spec))
}

fn get(
    fetch: &Fetch,
    spec: &'static SourceSpec,
    budget: Duration,
) -> Result<Duration, RequestError> {
    let leg = fetch.leg(spec, fetch.deadline(budget));
    let reply: Answer = leg.json(&spec.get("/answer"), &[])?;
    assert!(reply.answer, "the recorded request must complete");
    Ok(leg.remaining())
}

#[test]
fn one_callers_exhausted_bucket_does_not_delay_another_key() -> Check {
    let (_dir, shared, spec) = fixture(RateShare::default())?;
    let a = caller(&shared, Some("placeholder-caller-a"));
    let b = caller(&shared, Some("placeholder-caller-b"));
    get(&a, spec, Duration::from_secs(1))?;
    let error = get(&a, spec, Duration::from_millis(50));
    assert!(
        matches!(error, Err(RequestError::Source(e)) if e.reason == Reason::DeadlineExceeded),
        "the exhausted caller must reach its typed pacing deadline"
    );
    assert_eq!(
        get(&b, spec, Duration::from_millis(50))?,
        Duration::from_millis(50),
        "another credential must have its own initial token"
    );
    Ok(())
}

#[test]
fn separate_keyless_callers_share_one_source_bucket() -> Check {
    let (_dir, shared, spec) = fixture(RateShare::default())?;
    get(&caller(&shared, None), spec, Duration::from_secs(1))?;
    let error = get(&caller(&shared, None), spec, Duration::from_millis(50));
    assert!(
        matches!(error, Err(RequestError::Source(e)) if e.reason == Reason::DeadlineExceeded),
        "a second keyless caller must not receive a new bucket"
    );
    Ok(())
}

#[test]
fn rate_share_scales_both_keyed_and_keyless_source_rates() -> Check {
    for (key, expected) in [(None, 1000), (Some("placeholder-rate-key"), 500)] {
        let (_dir, shared, spec) = fixture("0.5".parse()?)?;
        let fetch = caller(&shared, key);
        get(&fetch, spec, Duration::from_secs(2))?;
        assert_eq!(
            get(&fetch, spec, Duration::from_secs(2))?,
            Duration::from_millis(2000 - expected),
            "replay token accounting must scale without wall-clock timing noise"
        );
    }
    Ok(())
}

#[test]
fn rate_share_rejects_zero_nonfinite_and_excess_shares() {
    for value in ["0", "-0.5", "1.01", "NaN", "inf", "-inf", "", "bad"] {
        assert!(
            value.parse::<RateShare>().is_err(),
            "invalid rate share {value}"
        );
    }
    for value in ["1", "0.5", "1e-20"] {
        assert!(
            value.parse::<RateShare>().is_ok(),
            "positive rate share {value}"
        );
    }
}
