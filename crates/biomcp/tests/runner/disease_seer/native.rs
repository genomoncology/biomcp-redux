use std::path::Path;
pub(in crate::runner) fn run(_: &Path) -> Result<(), String> {
    Err("native SEER command proof is not implemented".into())
}
