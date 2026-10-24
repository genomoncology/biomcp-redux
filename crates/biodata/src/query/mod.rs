//! Checked command inputs shared by every interface.
mod article;
mod article_get;
mod article_supplement;
pub use article::{
    ArticleFilter, ArticleSearch, ArticleSort, ArticleSource, ArticleType, RetractionFilter,
};
pub use article_supplement::{ArticleSupplement, SheetRows};
mod article_pdf;
pub use article_pdf::ArticlePdf;
mod article_text;
pub use article_get::{ArticleGet, ArticleSection, ArticleSectionWord};
pub use article_text::ArticleText;
mod gene;
mod request;
pub use gene::{GeneGet, GeneSearch, GeneSection, GeneSectionWord};
pub use request::Request;

mod article_edges;
pub use article_edges::{ArticleEdges, EdgeSource};

mod article_evidence;
mod article_recommend;
pub use article_evidence::ArticleEvidence;
pub use article_recommend::ArticleRecommend;

mod trial;
pub use trial::{PhaseFilter, SponsorType, TrialSearch};

mod trial_get;
pub use trial_get::{TrialGet, TrialSection};

mod trial_document;
pub use trial_document::TrialDocumentGet;
