//! One MCP call's process settings, shared transport, credentials and lifetime.
use crate::{config::StartupError, grammar::Globals};
use biomcp_fetch::{Cancellation, Fetch, Secrets};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(crate) struct Context<'a> {
    pub(crate) globals: &'a Globals,
    pub(crate) fetch: &'a Result<Arc<Fetch>, StartupError>,
    pub(crate) secrets: Option<&'a Secrets>,
    pub(crate) scope: Option<&'a Cancellation>,
}
impl<'a> Context<'a> {
    pub(crate) fn new(globals: &'a Globals, fetch: &'a Result<Arc<Fetch>, StartupError>) -> Self {
        Self { globals, fetch, secrets: None, scope: None }
    }
}
