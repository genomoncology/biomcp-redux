//! Preserve reviewed source field boundaries independently of open reply models.
use serde::{Deserialize, Deserializer, de::Error};

pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<T, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    let mut ignored = false;
    let value = serde_ignored::deserialize(deserializer, |_| ignored = true)?;
    if ignored {
        return Err(D::Error::custom("unrecognized source field"));
    }
    Ok(value)
}
