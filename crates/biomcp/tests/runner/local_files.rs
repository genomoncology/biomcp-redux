//! Feature local-file-persistence: real CLI files in isolated homes contain only synthetic data.
use super::{key_registration::invoke, launch::Environment};
use std::fs;
type Check = Result<(), String>;
