#![doc = include_str!("../README.md")]

mod response;
mod source;
mod text;

pub use response::{Problem, ProblemCode};
pub use source::{Reason, Source};
pub use text::Text;

mod coord;
mod date;
mod error;
mod id;
mod macros;

pub use coord::{Position1, Span0};
pub use date::PartialDate;
pub use error::Error;
pub use id::HgncId;
