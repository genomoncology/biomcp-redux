//! Command transport selection preserves server-owned resources under offline restrictions.
use crate::{config::StartupError, grammar::Globals};
use biomcp_fetch::Fetch;
use std::{io, sync::Arc};
pub(super) fn fetch(
    globals: &Globals,
    shared: Option<&Result<Arc<Fetch>, StartupError>>,
) -> io::Result<Result<Fetch, StartupError>> {
    match shared {
        Some(Ok(fetch)) => {
            let secrets = match crate::config::secrets(globals) {
                Ok(secrets) => secrets,
                Err(error) => return Ok(Err(error.into())),
            };
            let fetch = if globals.offline {
                fetch.offline()
            } else {
                fetch.as_ref().clone()
            };
            crate::config::restrict(fetch.with_secrets(secrets), globals).map(Ok)
        }
        Some(Err(error)) if !globals.offline => Ok(Err(error.clone())),
        _ => Ok(crate::config::fetch(globals)),
    }
}
