//! Credential placement and reflected metadata share one transport boundary.
use super::*;

#[test]
fn reflected_credentials_are_sent_but_refused_without_retry_or_public_disclosure() {
    let secret = "placeholder-only-abc123";
    for place in [
        KeyPlace::Query("api_key"),
        KeyPlace::Header("x-api-key"),
        KeyPlace::Bearer,
    ] {
        let media = format!("application/json\r\nContent-Range: {secret}");
        for (media, body) in [
            ("application/json", secret),
            (media.as_str(), "safe response body"),
        ] {
            let stub = Stub::new(500, media, body, None);
            let spec = stub.spec(KeyUse::Required(credential(place)), Duration::from_secs(10));
            let mut config = FetchConfig::default();
            config
                .secrets
                .insert(SecretName::NcbiApiKey, Secret::new(secret));
            assert!(!format!("{config:?}").contains(secret));
            let fetch = Fetch::open(config).unwrap();
            let leg = fetch.leg(spec, fetch.deadline(Duration::from_secs(10)));
            let error = leg.json::<Answer>(&spec.get("/answer"), &[]).unwrap_err();
            let biomcp_fetch::RequestError::Source(error) = error else {
                panic!("expected provider failure: {error}");
            };
            for formatted in [
                format!("{fetch:?}"),
                format!("{leg:?}"),
                format!("{error:?}"),
                error.to_string(),
                Secret::new(secret).to_string(),
            ] {
                assert!(!formatted.contains(secret));
            }
            assert_eq!(error.reason, Reason::BadReply);
            let requests = stub.finish();
            assert_eq!(requests.len(), 1);
            assert!(requests[0].contains(secret));
        }
    }
}
