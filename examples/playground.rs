use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::get;
use axum_error_object::{ErrorResponse, Result};
use derive_more::Display;
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Debug, Serialize, Deserialize, ErrorResponse, Display)]
#[serde(rename_all = "snake_case", tag = "code", content = "meta")]
enum PlaygroundError {
    #[display("far too high, try again with {what}")]
    #[response(status = 400)]
    TooHigh { what: u8 },
}

// Display is optional, without it the serialized error object will not have a title
#[derive(Debug, Serialize, Deserialize, ErrorResponse)]
#[serde(tag = "code", rename = "too_low")]
#[response(status = 422)]
struct PlaygroundErrorTooLow {}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = Router::new().route("/{param}", get(handler));
    let listener = TcpListener::bind("0.0.0.0:3000").await?;

    axum::serve(listener, app).await?;

    Ok(())
}

async fn handler(Path(param): Path<u8>) -> Result<StatusCode> {
    if param < 42 {
        return Err((PlaygroundErrorTooLow {}).into());
    }

    if param > 42 {
        return Err(PlaygroundError::TooHigh { what: 42 }.into());
    }

    Ok(StatusCode::OK)
}
