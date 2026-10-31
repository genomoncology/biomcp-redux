//! Copy source recordings from the common public fixture root.
#[path = "recordings.rs"]
#[expect(dead_code, reason = "The shared recording support also serves synthetic cache setup")]
mod recordings;
pub(crate) fn copy_capture(capture: &str, target: &std::path::Path, change: impl FnMut(&str, &mut serde_json::Value)) -> Result<(), Box<dyn std::error::Error>> {
    recordings::copy_capture(&super::fixture_paths::fixtures().join("replies").join(capture), target, change)
}
