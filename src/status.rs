use axum::http::StatusCode;

use crate::ErrorResponse;

pub trait Status<T> {
    #[inline]
    fn status(self, status: StatusCode) -> crate::Result<T>
    where
        Self: Sized,
    {
        self.with_status(move || status)
    }

    fn with_status(self, status: impl FnOnce() -> StatusCode) -> crate::Result<T>;
}

impl<T, E> Status<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn with_status(self, status: impl FnOnce() -> StatusCode) -> crate::Result<T> {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(ErrorResponse::from_status(status()).with_source(error.into())),
        }
    }
}

impl<T> Status<T> for Option<T> {
    fn with_status(self, status: impl FnOnce() -> StatusCode) -> crate::Result<T> {
        match self {
            Some(value) => Ok(value),
            None => Err(ErrorResponse::from_status(status())),
        }
    }
}
