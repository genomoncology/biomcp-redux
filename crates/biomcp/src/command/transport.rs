//! Command transport selection preserves server-owned resources under offline restrictions.
use crate::grammar::Globals;
use biomcp_fetch::Fetch;
use std::{io, sync::Arc};
pub(super) fn fetch(
    globals: &Globals,
    shared: Option<&Result<Arc<Fetch>, ()>>,
) -> io::Result<Result<Fetch, ()>> {
    match shared {
        Some(Ok(fetch)) => {
            let secrets = match crate::config::secrets(globals) {
                Ok(secrets) => secrets,
                Err(()) => return Ok(Err(())),
            };
            let fetch = if globals.offline {
                fetch.offline()
            } else {
                fetch.as_ref().clone()
            };
            crate::config::restrict(fetch.with_secrets(secrets), globals).map(Ok)
        }
        Some(Err(())) if !globals.offline => Ok(Err(())),
        _ => Ok(crate::config::fetch(globals)),
    }
}
