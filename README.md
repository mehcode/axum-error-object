# axum-error-object

Provides a `Result<T>` type and related utility types
that can be used to holistically provide object response errors.

## Install

```toml
axum-error-object = "0.0.1"
```

## Usage

```rust
use axum_error_object::{Result, Status, IntoErrorResponse};
use serde::Serialize;
use derive_more::Display;

fn call_fallible() -> std::result::Result<T, E> { /* ... */ }
fn call_maybe() -> Option<T> { /* ... */ }

#[derive(Display, ErrorResponse, Serialize)]
#[serde(rename_all = "snake_case", tag = "code", content = "meta")]
enum AppError {
  #[display("Whoa there that is {that}")]
  #[response(status = 420)]
  WhoaThere { that: u8 },

  #[display("Oh No")]
  #[response(status = 501)]
  OhNo,
}

async fn handler() -> Result<StatusCode> {
  // will return an opaque 500 on any unhandled error or None
  call_fallible()?;
  call_maybe()?;

  // will return an opaque 404 on None or error
  call_fallible().context(StatusCode::NOT_FOUND)?;
  call_maybe().context(StatusCode::NOT_FOUND)?;

  // will return the following 420 response:
  //  {
  //    "code": "whoa_there",
  //    "title": "Whoa there that is 10",
  //    "meta": { "that": 10 }
  //  }
  // preserves the original error information as source (does not return from the API)
  // useful for logs and inspection
  call_fallible().with_context(|_| AppError::WhoaThere { that: 30 })?;
  call_fallible().context(AppError::OhNo)?;
  call_maybe().context(AppError::OhNo)?;

  // return ok
  Ok(StatusCode::OK)
}
```

## License

Licensed under either of

Apache License, Version 2.0 (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
MIT license (LICENSE-MIT or https://opensource.org/licenses/MIT)
at your option.

**Contribution**

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
