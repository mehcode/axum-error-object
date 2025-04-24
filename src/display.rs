use std::fmt::Display;

pub trait DisplayKind {
    fn axum_error_object_title(&self) -> Option<String>;
}


// if the type implements Display
impl<T: Display> DisplayKind for T {
    fn axum_error_object_title(&self) -> Option<String> {
        Some(self.to_string())
    }
}

pub trait NoDisplayKind {
    #[inline]
    fn axum_error_object_title(&self) -> Option<String> {
        None
    }
}

// if the type does not implements Display
impl<T> NoDisplayKind for &T {}
