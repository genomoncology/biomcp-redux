//! Execute checked requests with the CLI and MCP field selection.
use biomcp_types::Request;
use std::io;

pub(super) fn run_request(
    fetch: &biomcp_fetch::Fetch,
    request: &Request,
    fresh: bool,
    settings: &crate::config::Settings,
    allow_save: bool,
    fields: &[String],
) -> io::Result<biomcp_query::Outcome> {
    let configured_request = crate::config::request_defaults(settings, request)
        .map_err(|_| io::Error::other("local configuration failed"))?;
    let request = &configured_request;
    let cx = super::query_context(fetch, fresh, settings)?;
    Ok(match request {
        Request::TrialDocument(query) if let Some(path) = &query.save => {
            crate::save::trial::document(&cx, query, path, allow_save)
        }
        Request::ArticlePdf(query) => crate::save::run(&cx, query, allow_save),
        Request::ArticleText(query) if query.save().is_some() => {
            crate::save::article::text(&cx, query, allow_save)
        }
        Request::ArticleSupplement(query) if query.save().is_some() => {
            crate::save::article::supplement(&cx, query, allow_save)
        }
        _ => {
            let projection = crate::fields::arm_projection(request, fields);
            biomcp_query::run_with_arm_projection(
                &cx,
                request,
                projection
                    .as_ref()
                    .map(|project| project as &biomcp_query::TrialArmProjection<'_>),
            )
        }
    })
}
