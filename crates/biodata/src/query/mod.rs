//! Checked command inputs shared by every interface.
mod article;
mod article_get;
mod article_text;
pub use article::{ArticleSearch, ArticleSort, ArticleSource, ArticleType};
pub use article_get::{ArticleGet, ArticleSection, ArticleSectionWord};
pub use article_text::ArticleText;
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;

mod article_edges;
pub use article_edges::ArticleEdges;
