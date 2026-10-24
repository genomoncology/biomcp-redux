//! Checked command inputs shared by every interface.
mod article;
mod article_get;
pub use article::{ArticleFilter, ArticleSearch, ArticleSort, ArticleSource, ArticleType};
pub use article_get::{ArticleGet, ArticleSection, ArticleSectionWord};
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;

mod article_edges;
pub use article_edges::ArticleEdges;
