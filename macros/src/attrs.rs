use http::StatusCode;
use syn::{Attribute, LitInt, Token};

pub struct Attributes {
    pub status: StatusCode,
}

impl Attributes {
    pub fn parse(input: &[Attribute]) -> syn::Result<Self> {
        let mut status = StatusCode::INTERNAL_SERVER_ERROR;

        for attr in input {
            if attr.path().is_ident("response") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("status") {
                        // response(status = _)
                        meta.input.parse::<Token![=]>()?;

                        // can be a number (code)
                        let code: LitInt = meta.input.parse()?;
                        let code: u16 = code.base10_parse()?;

                        // must parse to a valid HTTP status code
                        status = StatusCode::try_from(code).map_err(|_| {
                            syn::Error::new_spanned(meta.path, "invalid value for status")
                        })?;

                        Ok(())
                    } else {
                        Err(syn::Error::new_spanned(meta.path, "unexpected attribute"))
                    }
                })?;
            }
        }

        Ok(Self { status })
    }
}
