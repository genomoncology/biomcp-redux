//! MCP discovery settings.
use super::{McpCatalog, Settings};
use crate::grammar::Globals;

pub(crate) const DEFAULT_TYPED_TOOL_LIMIT: usize = 20;
pub(crate) const DEFAULT_TYPED_DEFINITION_BYTES: usize = 128 * 1024;
pub(crate) const DEFAULT_TYPED_OUTPUT_DEFINITION_BYTES: usize = 192 * 1024;

pub(crate) fn mcp_skills(settings: &Settings) -> bool {
    super::configured(settings)
        .ok()
        .flatten()
        .and_then(|file| file.mcp_skills)
        .unwrap_or(true)
}

pub(crate) fn mcp_typed_budget(settings: &Settings) -> (usize, usize) {
    let file = settings.as_ref().ok().and_then(Option::as_ref);
    let byte_default = if super::mcp_typed_schema_options(settings).0 {
        DEFAULT_TYPED_OUTPUT_DEFINITION_BYTES
    } else {
        DEFAULT_TYPED_DEFINITION_BYTES
    };
    (
        file.and_then(|file| file.mcp_typed_tool_limit)
            .map_or(DEFAULT_TYPED_TOOL_LIMIT, std::num::NonZeroUsize::get),
        file.and_then(|file| file.mcp_typed_definition_bytes)
            .map_or(byte_default, std::num::NonZeroUsize::get),
    )
}

pub(crate) fn mcp_catalog(globals: &Globals, settings: &Settings) -> McpCatalog {
    globals
        .mcp_catalog
        .or_else(|| {
            settings
                .as_ref()
                .ok()
                .and_then(Option::as_ref)
                .and_then(|file| file.mcp_catalog)
        })
        .unwrap_or_default()
}
