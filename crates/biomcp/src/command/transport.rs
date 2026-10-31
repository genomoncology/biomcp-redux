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
            let fetch = if globals.offline {
                fetch.offline()
            } else {
                fetch.as_ref().clone()
            };
            snapshot(fetch, globals)
        }
        Some(Err(error)) if !globals.offline => Ok(Err(error.clone())),
        _ if globals.hosted => match crate::config::fetch(globals) {
            Ok(fetch) => snapshot(fetch, globals),
            Err(error) => Ok(Err(error)),
        },
        _ => Ok(crate::config::fetch(globals)),
    }
}
fn snapshot(fetch: Fetch, globals: &Globals) -> io::Result<Result<Fetch, StartupError>> {
    let secrets = match crate::config::secrets(globals) {
        Ok(secrets) => secrets,
        Err(error) => return Ok(Err(error.into())),
    };
    let secrets = if globals.hosted {
        fetch.hosted_secrets(
            secrets,
            biomcp_fetch::SecretName::ALL
                .iter()
                .copied()
                .filter(|&name| {
                    biomcp_adapters::registry::key_tier(name) == biomcp_types::KeyTier::NoFallback
                }),
        )
    } else {
        secrets
    };
    crate::config::restrict(fetch.with_secrets(secrets), globals).map(Ok)
}
