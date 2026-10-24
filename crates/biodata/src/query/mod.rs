//! Checked command inputs shared by every interface.
mod article;
mod article_get;
pub use article::{ArticleSearch, ArticleSort, ArticleSource};
pub use article_get::ArticleGet;
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;
