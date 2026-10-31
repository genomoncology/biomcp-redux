//! The checked synthetic JSON reply shared by transport contracts.
use biomcp_fetch::{BadReply, Reply};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(crate) struct Answer {
    pub(crate) answer: bool,
}
impl Reply for Answer {
    fn check(&self) -> Result<(), BadReply> {
        if self.answer { Ok(()) } else { Err(BadReply) }
    }
}
