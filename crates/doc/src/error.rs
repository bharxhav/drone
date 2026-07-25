use miette::Diagnostic;
use thiserror::Error;

#[derive(Debug, Diagnostic, Error)]
pub enum Error {
    #[error("failed to retrieve documentation")]
    #[diagnostic(code(doc::request))]
    Http(
        #[from]
        #[source]
        reqwest::Error,
    ),

    #[error("received invalid documentation data")]
    #[diagnostic(code(doc::invalid_data))]
    Json(
        #[from]
        #[source]
        serde_json::Error,
    ),

    #[error("failed to parse documentation selector: {0}")]
    #[diagnostic(code(doc::selector))]
    Selector(String),

    #[error("documentation data is unavailable")]
    #[diagnostic(code(doc::unavailable))]
    Unavailable,

    #[error("documentation path not found: {0}")]
    #[diagnostic(code(doc::not_found))]
    NotFound(String),
}
