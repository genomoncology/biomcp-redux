//! The MCP guide and the same worked examples the CLI embeds.
use serde_json::Value;
use std::sync::LazyLock;

pub(crate) const GUIDE: &str = include_str!("biomcp-research/SKILL.md");
pub(crate) static FRONTMATTER: LazyLock<Value> = LazyLock::new(|| {
    serde_json::from_str(include_str!(concat!(
        env!("OUT_DIR"),
        "/research-frontmatter.json"
    )))
    .expect("build generated research skill frontmatter")
});

pub(crate) fn references() -> impl Iterator<Item = (&'static str, &'static str)> {
    super::examples().filter_map(|(path, text)| Some((path.strip_prefix("use-cases/")?, *text)))
}
