//! Source contracts exercise the adapters that own native admission and mapping.
#[path = "mapped_refusals/mod.rs"]
mod mapped_refusals;
#[path = "../../biomcp-fetch/tests/support/fixtures.rs"]
mod fixture_paths;
#[path = "../../biomcp-fetch/tests/support/hash.rs"]
mod stable_hash;
#[path = "../../biomcp-fetch/tests/support/capture.rs"]
mod capture_support;
#[path = "../../biomcp-fetch/tests/support/replay.rs"]
mod replay_support;
#[path = "../../biomcp-fetch/tests/support/value.rs"]
mod value;
#[path = "../../biomcp-fetch/tests/support/network_stub.rs"]
#[expect(dead_code, reason = "The TLS fixture also serves binary launch and evidence contracts")]
mod network_stub;
use libtest_mimic::{Trial, Failed};
fn trial(name: &str, run: impl FnOnce() -> Result<(), Failed> + Send + 'static) -> Trial { Trial::test(name, run) }
fn main() { libtest_mimic::run(&libtest_mimic::Arguments::from_args(), mapped_refusals::trials()).exit(); }
