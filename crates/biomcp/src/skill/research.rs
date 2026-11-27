//! The MCP guide and the same worked examples the CLI embeds.
pub(crate) const GUIDE: &str = include_str!("biomcp-research/SKILL.md");
pub(crate) const FRONTMATTER: &str =
    include_str!(concat!(env!("OUT_DIR"), "/research-frontmatter.json"));

pub(crate) fn references() -> impl Iterator<Item = (&'static str, &'static str)> {
    super::examples().filter_map(|(path, text)| Some((path.strip_prefix("use-cases/")?, *text)))
}
