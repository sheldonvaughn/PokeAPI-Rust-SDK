use std::fmt;

/// Custom error type for the PokeAPI SDK.
#[derive(Debug)]
pub enum PokeApiError {
    /// Error occurred while making the HTTP request.
    RequestError(reqwest::Error),
    /// Error occurred while deserializing the response.
    DeserializationError(reqwest::Error),
    /// API returned a non-success status code.
    ApiError { status: u16, url: String },
}

impl PokeApiError {
    /// Returns the HTTP status code if the error is an API error.
    pub fn status_code(&self) -> Option<u16> {
        if let PokeApiError::ApiError { status, .. } = self {
            Some(*status)
        } else {
            None
        }
    }

    /// Checks if the error is a 404 Not Found.
    pub fn is_not_found(&self) -> bool {
        matches!(self, PokeApiError::ApiError { status: 404, .. })
    }

    /// Retrieves the URL associated with the error, if any.
    pub fn url(&self) -> Option<&str> {
        match self {
            PokeApiError::ApiError { url, .. } => Some(url),
            _ => None,
        }
    }
}

impl fmt::Display for PokeApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PokeApiError::RequestError(err) => write!(f, "Request error: {}", err),
            PokeApiError::DeserializationError(err) => write!(f, "Deserialization error: {}", err),
            PokeApiError::ApiError { status, url } => write!(f, "API error {}: {}", status, url),
        }
    }
}

impl std::error::Error for PokeApiError {}

impl From<reqwest::Error> for PokeApiError {
    fn from(error: reqwest::Error) -> Self {
        if error.is_decode() {
            PokeApiError::DeserializationError(error)
        } else {
            PokeApiError::RequestError(error)
        }
    }
}
