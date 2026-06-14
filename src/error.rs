use thiserror::Error;

/// Errors returned by the Meshbrow SDK.
#[derive(Debug, Error)]
pub enum Error {
    /// HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// API returned an error response.
    #[error("API error ({status}): {body}")]
    Api { status: u16, body: String },

    /// Failed to parse response.
    #[error("Parse error: {0}")]
    Parse(#[from] serde_json::Error),
}
