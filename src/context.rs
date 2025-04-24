use crate::ErrorResponse;

pub trait Context<T> {
    #[inline]
    fn context<C>(self, context: C) -> crate::Result<T>
    where
        Self: Sized,
        C: Into<ErrorResponse>,
    {
        self.with_context(move || context)
    }

    fn with_context<C>(self, context: impl FnOnce() -> C) -> crate::Result<T>
    where
        C: Into<ErrorResponse>;
}

impl<T, E> Context<T> for Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn with_context<C>(self, context: impl FnOnce() -> C) -> crate::Result<T>
    where
        C: Into<ErrorResponse>,
    {
        match self {
            Ok(value) => Ok(value),
            Err(error) => Err(context().into().with_source(error.into())),
        }
    }
}

impl<T> Context<T> for Option<T> {
    fn with_context<C>(self, context: impl FnOnce() -> C) -> crate::Result<T>
    where
        C: Into<ErrorResponse>,
    {
        match self {
            Some(value) => Ok(value),
            None => Err(context().into()),
        }
    }
}
