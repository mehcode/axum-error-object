use proc_macro::TokenStream;
use quote::quote;
use syn::punctuated::Punctuated;
use syn::token::Comma;
use syn::{Data, DataEnum, DataStruct, DeriveInput, Variant};

use crate::attrs::Attributes;

pub fn expand(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Enum(DataEnum { variants, .. }) => expand_enum(&input, &variants),
        Data::Struct(DataStruct { .. }) => expand_struct(&input),
        Data::Union(_) => Err(syn::Error::new_spanned(input, "unions are not supported")),
    }
}

fn expand_enum(
    input: &DeriveInput,
    variants: &Punctuated<Variant, Comma>,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;

    let mut status_arms = Vec::new();

    for v in variants {
        let variant = &v.ident;
        let attrs = Attributes::parse(&v.attrs)?;
        let status = attrs.status.as_u16();

        // TODO: make this error compile time after https://github.com/hyperium/http/pull/761
        status_arms.push(
            quote!(#ident :: #variant { .. } => ::axum::http::StatusCode::from_u16(#status).unwrap()),
        )
    }

    Ok(quote! {
        impl From<#ident> for ::axum_error_object::ErrorResponse {
            fn from(value: #ident) -> Self {
                use ::axum::response::IntoResponse;
                use ::axum_error_object::__private::{DisplayKind, NoDisplayKind};

                // pull out a title from Display::fmt if available
                let title = (&value).axum_error_object_title();

                let status = match value {
                    #(#status_arms),*
                };

                let object = ::axum_error_object::ErrorObject {
                    title,
                    status,
                    error: value,
                };

                object.into()
            }
        }
    }
    .into())
}

fn expand_struct(input: &DeriveInput) -> syn::Result<TokenStream> {
    let attrs = Attributes::parse(&input.attrs)?;

    let ident = &input.ident;
    let status: u16 = attrs.status.as_u16();

    Ok(quote! {
        impl From<#ident> for ::axum_error_object::ErrorResponse {
            fn from(value: #ident) -> Self {
                use ::axum::response::IntoResponse;
                use ::axum_error_object::__private::{DisplayKind, NoDisplayKind};

                // pull out a title from Display::fmt if available
                let title = (&value).axum_error_object_title();

                // TODO: make this error compile time after https://github.com/hyperium/http/pull/761
                let status = ::axum::http::StatusCode::from_u16(#status).unwrap();

                let object = ::axum_error_object::ErrorObject {
                    title,
                    status,
                    error: value,
                };

                object.into()
            }
        }
    }
    .into())
}
