use axum::Json;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};

use crate::ErrorResponse;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorObject<E = ()> {
    #[serde(skip)]
    pub status: StatusCode,

    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,

    #[serde(flatten)]
    pub error: E,
}

impl<E> From<ErrorObject<E>> for ErrorResponse
where
    E: Serialize,
{
    fn from(value: ErrorObject<E>) -> Self {
        let response = (value.status, Json(value)).into_response();

        ErrorResponse::from_response(response)
    }
}
