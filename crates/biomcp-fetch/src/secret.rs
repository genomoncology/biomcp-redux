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
