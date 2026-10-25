use std::{fmt, str::FromStr};
/// A published native drug record identifier, without a consumer source prefix.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct DrugId(Box<str>);
impl FromStr for DrugId {
    type Err = crate::Error;
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        if value.is_empty()
            || !value.bytes().next().is_some_and(|b| b.is_ascii_alphanumeric())
            || !value.bytes().all(|b| b.is_ascii_alphanumeric() || b"-._/".contains(&b))
            || value.split('/').any(str::is_empty)
        {
            return Err(crate::Error::DrugIdInvalid);
        }
        Ok(Self(value.into()))
    }
}
impl fmt::Display for DrugId {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.0)
    }
}
crate::text_value!(DrugId, "DrugId", "^[A-Za-z0-9][A-Za-z0-9_.-]*(/[A-Za-z0-9_.-]+)*$");
