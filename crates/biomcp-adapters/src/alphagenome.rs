//! Declare AlphaGenome's restricted, user-keyed prediction service.
use biomcp_fetch::{Credential, KeyPlace, KeyUse, SecretName, SourceSpec};
/// Publisher policy shared by capture and the future variant section.
pub static SPEC: SourceSpec = SourceSpec {
    destinations: biomcp_fetch::Destinations::Primary,
    user_agent: None,
    source: biomcp_types::Source::AlphaGenome,
    terms: biomcp_fetch::Terms {
        name: "AlphaGenome API terms",
        url: "https://alphagenome.google/terms",
        usage: biomcp_types::Use::Restricted,
    },
    base: "https://gdmscience.googleapis.com:443",
    key: KeyUse::Required(Credential {
        secret: SecretName::AlphaGenomeApiKey,
        place: KeyPlace::Header("x-goog-api-key"),
    }),
    quota: &crate::quotas::ALPHAGENOME,
    deadline: std::time::Duration::from_secs(120),
};
/// One documented prediction method, with complete-stream admission and no cache.
pub const PREDICT_VARIANT: biomcp_fetch::grpc::GrpcEndpoint = SPEC
    .grpc(biomcp_fetch::grpc::alphagenome::PREDICT_VARIANT)
    .checked(
        biomcp_fetch::grpc::alphagenome::SCHEMA,
        biomcp_fetch::Freshness::Never,
    );

/// Ruled legacy scorer method, admitted only after complete checked terminal success.
pub const SCORE_VARIANT: biomcp_fetch::grpc::GrpcEndpoint = SPEC
    .grpc(biomcp_fetch::grpc::alphagenome::SCORE_VARIANT)
    .bounded(biomcp_fetch::grpc::alphagenome::SCORE_LIMITS)
    .checked(
        biomcp_fetch::grpc::alphagenome::SCORE_SCHEMA,
        biomcp_fetch::Freshness::Never,
    );
