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
pub struct Secrets(BTreeMap<SecretName, Secret>);

impl Secrets {
    /// Insert or replace a configured secret.
    pub fn insert(&mut self, name: SecretName, value: Secret) {
        self.0.insert(name, value);
    }

    pub(crate) fn get(&self, name: SecretName) -> Option<&str> {
        self.0
            .get(&name)
            .map(Secret::value)
            .filter(|v| !v.is_empty())
    }
}

impl SecretName {
    /// Every supported credential identity, shared by configuration and CLI discovery.
    pub const ALL: &[Self] = &[
        Self::NcbiApiKey,
        Self::SemanticScholarApiKey,
        Self::OncokbApiKey,
    ];
    /// Source name accepted by key registration.
    pub const fn source(self) -> &'static str {
        match self {
            Self::NcbiApiKey => "ncbi",
            Self::SemanticScholarApiKey => "semantic-scholar",
            Self::OncokbApiKey => "oncokb",
        }
    }
    /// Field within the configuration's keys object.
    pub const fn config(self) -> &'static str {
        match self {
            Self::NcbiApiKey => "ncbi_api_key",
            Self::SemanticScholarApiKey => "semantic_scholar_api_key",
            Self::OncokbApiKey => "oncokb_api_key",
        }
    }
    /// Optional environment fallback.
    pub const fn environment(self) -> &'static str {
        match self {
            Self::NcbiApiKey => "NCBI_API_KEY",
            Self::SemanticScholarApiKey => "SEMANTIC_SCHOLAR_API_KEY",
            Self::OncokbApiKey => "ONCOKB_API_KEY",
        }
    }
}
