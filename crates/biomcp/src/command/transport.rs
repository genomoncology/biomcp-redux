//! Command transport selection preserves server-owned resources under offline restrictions.
use crate::{command::Context, config::StartupError, grammar::Globals};
use biomcp_fetch::Fetch;
use std::io;
pub(super) fn fetch(
    globals: &Globals,
    settings: &crate::config::Settings,
    context: Option<Context<'_>>,
) -> io::Result<Result<Fetch, StartupError>> {
    match context.map(|context| context.fetch) {
        Some(Ok(fetch)) => {
            let fetch = if globals.offline {
                fetch.offline()
            } else {
                fetch.as_ref().clone()
            };
            snapshot(fetch, globals, settings, context)
        }
        Some(Err(error)) if !globals.offline => Ok(Err(error.clone())),
        _ if globals.hosted => match crate::config::fetch(globals, settings) {
            Ok(fetch) => snapshot(fetch, globals, settings, context),
            Err(error) => Ok(Err(error)),
        },
        _ => Ok(crate::config::fetch(globals, settings)),
    }
}
fn snapshot(
    fetch: Fetch,
    globals: &Globals,
    settings: &crate::config::Settings,
    context: Option<Context<'_>>,
) -> io::Result<Result<Fetch, StartupError>> {
    let fetch = if let Some(scope) = context.and_then(|context| context.scope) {
        match fetch.with_cancellation(scope) {
            Ok(fetch) => fetch,
            Err(_) => return Ok(Err(StartupError::Local)),
        }
    } else {
        fetch
    };
    let credentials = if context.is_some() {
        crate::config::credentials::refresh(globals)
    } else {
        crate::config::secrets(settings)
    };
    let secrets = match credentials {
        Ok(secrets) => secrets,
        Err(error) => return Ok(Err(error.into())),
    };
    let fetch = if globals.hosted {
        fetch.with_hosted_secrets(
            secrets.hosted(
                context.and_then(|context| context.secrets),
                biomcp_fetch::SecretName::ALL
                    .iter()
                    .copied()
                    .filter(|&name| {
                        biomcp_query::sources::key_tier(name) == biomcp_types::KeyTier::NoFallback
                    }),
            ),
        )
    } else {
        fetch.with_secrets(secrets)
    };
    crate::config::restrict(fetch, globals).map(Ok)
}
