//! Feature drug-next-steps: wholly synthetic D05/D17/D19-shaped source contexts.
//! These fixtures test selector propagation, never clinical answers or licensed annotations.
use super::{configuration::invoke, launch::Environment, network_stub::Stub};
use libtest_mimic::Trial;
use serde_json::{Value, json};
use std::fs;
mod native;

pub(super) fn trials() -> Vec<Trial> {
    native::trials()
}
