// mod error;
mod response;
mod error;
mod display;

// internal items needed by the proc macros
#[doc(hidden)]
pub mod __private {
    pub use super::display::{DisplayKind, NoDisplayKind};
}

pub type Result<T> = std::result::Result<T, ErrorResponse>;

pub use response::ErrorResponse;
pub use axum_error_object_macros::ErrorResponse;
pub use error::ErrorObject;
