//! Record-owned permissions shared by article adapters and retained replies.
mod abstracts;
mod policy;
pub(crate) mod pubmed;
pub(crate) mod pubtator;
pub(crate) mod semantic;
pub use policy::{
    Licenses, item_unchecked, items, licenses, snapshot, text_unchecked, text_withheld,
    unchecked_notice,
};
pub(crate) use policy::{europepmc, litsense, permitted, unchecked, withheld, withheld_item};
use policy::{is_abstract, pmid};
