//! Feature diagnostic-who: exact cards consume a complete explicit stored export.

mod search;
use super::{
    configuration::invoke,
    launch::{Environment, fixtures},
};
use crate::runner::transcript::trial as transcript_trial;
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::{
    fs,
    path::{Path, PathBuf},
};
type Check = Result<(), String>;
pub(super) fn trials() -> Vec<Trial> {
    // The test follows runtime revision-bound pages and compares the complete candidate set without replacement requests.
    vec![transcript_trial(
        crate::runner::pool_contract::WHO_REVISION.0,
        || search::walk().map_err(Into::into),
    )]
}
fn call(env: &Environment, replay: &Path, args: &[&str], exit: i32) -> Result<Value, String> {
    let mut command = env.globals(Some(replay), false);
    command.extend(args.iter().map(|s| (*s).into()));
    invoke(env, command, exit)
}
