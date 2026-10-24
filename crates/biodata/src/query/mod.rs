//! Checked command inputs shared by every interface.
mod article;
mod article_get;
mod article_text;
pub use article_text::ArticleText;
pub use article::{ArticleSearch, ArticleSort, ArticleSource, ArticleType};
pub use article_get::{ArticleGet, ArticleSection, ArticleSectionWord};
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;
