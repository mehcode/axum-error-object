use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};

#[derive(Debug)]
#[must_use]
pub struct ErrorResponse {
    response: Response,
    source: Option<anyhow::Error>,
}

impl ErrorResponse {
    pub(crate) fn from_response(response: Response) -> Self {
        Self { response, source: None }
    }

    pub(crate) fn from_status(status: StatusCode) -> Self {
        Self { response: status.into_response(), source: None }
    }

    pub(crate) fn with_source(self, source: anyhow::Error) -> Self {
        Self { response: self.response, source: Some(source) }
    }

    /// Returns the wrapped error contained by this error response.
    pub fn error(&self) -> Option<&anyhow::Error> {
        self.source.as_ref()
    }
}

impl<E> From<E> for ErrorResponse
where
    E: Into<anyhow::Error>,
{
    fn from(error: E) -> Self {
        // unhandled fallback converts errors into an opaque 500 response
        let error = error.into();
        let response = StatusCode::INTERNAL_SERVER_ERROR.into_response();

        Self { response, source: Some(error) }
    }
}

impl IntoResponse for ErrorResponse {
    fn into_response(self) -> Response {
        self.response
    }
}
