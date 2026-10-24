//! Checked command inputs shared by every interface.
mod article;
pub use article::{ArticleSearch, ArticleSource, ArticleSort};
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;
