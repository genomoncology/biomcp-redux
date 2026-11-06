//! Record-owned permissions shared by article adapters and retained replies.
mod policy;
pub(crate) mod pubmed;
pub(crate) mod pubtator;
pub(crate) mod semantic;
pub(crate) use policy::{europepmc, litsense, permitted, withheld, withheld_item};
use policy::{is_abstract, pmid};
pub use policy::{items, licenses, snapshot, text_withheld};
