#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("API error rc={0}")]
    ApiError(reqwest::StatusCode),

    #[error("HTTP error {0}")]
    HttpError(#[from] reqwest::Error),

    #[error("URL error {0}")]
    UrlError(#[from] url::ParseError),
}

pub type Result<T> = std::result::Result<T, Error>;
