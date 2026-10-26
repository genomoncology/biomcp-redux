use crate::SecretName;
use std::{collections::BTreeMap, fmt};

/// A credential that never formats its value.
pub struct Secret(Box<str>);

impl Secret {
    /// Store an explicitly supplied credential.
    pub fn new(value: impl Into<Box<str>>) -> Self {
        Self(value.into())
    }

    pub(crate) fn value(&self) -> &str {
        &self.0
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("***")
    }
}

/// Explicitly supplied secrets; never reads environment or files.
#[derive(Debug, Default)]
pub struct Secrets(BTreeMap<SecretName, SecretValue>);
#[derive(Debug)]
enum SecretValue {
    Ready(Secret),
    Deferred(DeferredSecret),
}
// A caller-supplied credential resolves once per Secrets owner, never globally.
struct DeferredSecret {
    resolved: std::sync::OnceLock<Option<Secret>>,
    load: Box<dyn Fn() -> Option<Secret> + Send + Sync>,
}
impl fmt::Debug for DeferredSecret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DeferredSecret(***)")
    }
}
impl SecretValue {
    fn get(&self) -> Option<&str> {
        match self {
            Self::Ready(secret) => Some(secret.value()),
            Self::Deferred(secret) => secret
                .resolved
                .get_or_init(|| (secret.load)())
                .as_ref()
                .map(Secret::value),
        }
        .filter(|v| !v.is_empty())
    }
}

impl Secrets {
    /// Insert or replace a configured secret.
    pub fn insert(&mut self, name: SecretName, value: Secret) {
        self.0.insert(name, SecretValue::Ready(value));
    }

    /// Resolve a caller-owned credential only when an admitted request needs it.
    /// The loader runs once for this owner, including a missing credential.
    pub fn defer(
        &mut self,
        name: SecretName,
        load: impl Fn() -> Option<Secret> + Send + Sync + 'static,
    ) {
        self.0.insert(
            name,
            SecretValue::Deferred(DeferredSecret {
                resolved: std::sync::OnceLock::new(),
                load: Box::new(load),
            }),
        );
    }
    pub(crate) fn get(&self, name: SecretName) -> Option<&str> {
        self.0.get(&name).and_then(SecretValue::get)
    }
}

impl SecretName {
    /// Every supported credential identity, shared by configuration and CLI discovery.
    pub const ALL: &[Self] = &[
        Self::NcbiApiKey,
        Self::SemanticScholarApiKey,
        Self::OncokbApiKey,
        Self::NciApiKey,
    ];
    /// Source name accepted by key registration.
    pub const fn source(self) -> &'static str {
        match self {
            Self::NcbiApiKey => "ncbi",
            Self::SemanticScholarApiKey => "semantic-scholar",
            Self::OncokbApiKey => "oncokb",
            Self::NciApiKey => "nci",
        }
    }
    /// Field within the configuration's keys object.
    pub const fn config(self) -> &'static str {
        match self {
            Self::NcbiApiKey => "ncbi_api_key",
            Self::SemanticScholarApiKey => "semantic_scholar_api_key",
            Self::OncokbApiKey => "oncokb_api_key",
            Self::NciApiKey => "nci_api_key",
        }
    }
    /// Optional environment fallback.
    pub const fn environment(self) -> &'static str {
        match self {
            Self::NcbiApiKey => "NCBI_API_KEY",
            Self::SemanticScholarApiKey => "S2_API_KEY",
            Self::OncokbApiKey => "ONCOKB_TOKEN",
            Self::NciApiKey => "NCI_API_KEY",
        }
    }
}
