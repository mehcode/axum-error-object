mod context;
mod display;
mod error;
mod response;
mod status;

// internal items needed by the proc macros
#[doc(hidden)]
pub mod __private {
    pub use super::display::{DisplayKind, NoDisplayKind};
}

pub type Result<T> = std::result::Result<T, ErrorResponse>;

pub use axum_error_object_macros::ErrorResponse;
pub use context::Context;
pub use error::ErrorObject;
pub use response::ErrorResponse;
pub use status::Status;
