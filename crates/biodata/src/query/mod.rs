//! Checked command inputs shared by every interface.
mod article;
pub use article::{ArticleSearch, ArticleSort, ArticleSource, ArticleType};
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;
