//! Checked command inputs shared by every interface.
mod article;
mod article_get;
pub use article_get::ArticleGet;
pub use article::{ArticleSearch, ArticleSort, ArticleSource};
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;
