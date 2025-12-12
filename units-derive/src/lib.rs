//! Derive macro implementation used by `unit-core`.
//!
//! `unit-derive` is an implementation detail of this workspace. The `Unit` derive expands in terms of `crate::Unit`
//! and `crate::Quantity`, so it is intended to be used by `unit-core` (or by crates that expose an identical
//! crate-root API).
//!
//! Most users should depend on `unit` instead and use the predefined units.
//!
//! # Generated impls
//!
//! For a unit marker type `MyUnit`, the derive implements:
//!
//! - `crate::Unit for MyUnit`
//! - `core::fmt::Display for crate::Quantity<MyUnit>` (formats as `<value> <symbol>`)
//!
//! # Attributes
//!
//! The derive reads a required `#[unit(...)]` attribute:
//!
//! - `symbol = "m"`: displayed unit symbol
//! - `dimension = SomeDim`: dimension marker type
//! - `ratio = 1000.0`: conversion ratio to the canonical unit of the dimension

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, DeriveInput, Expr, Ident, LitStr, Token,
};

/// Derive `crate::Unit` and a `Display` impl for `crate::Quantity<ThisUnit>`.
///
/// The derive must be paired with a `#[unit(...)]` attribute providing `symbol`, `dimension`, and `ratio`.
///
/// This macro is intended for use by `unit-core`.
#[proc_macro_derive(Unit, attributes(unit))]
pub fn derive_unit(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match derive_unit_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.to_compile_error().into(),
    }
}

fn derive_unit_impl(input: DeriveInput) -> syn::Result<TokenStream2> {
    let name = &input.ident;

    // Parse the #[unit(...)] attribute
    let unit_attr = parse_unit_attribute(&input.attrs)?;

    let symbol = &unit_attr.symbol;
    let dimension = &unit_attr.dimension;
    let ratio = &unit_attr.ratio;

    let expanded = quote! {
        impl crate::Unit for #name {
            const RATIO: f64 = #ratio;
            type Dim = #dimension;
            const SYMBOL: &'static str = #symbol;
        }

        impl ::core::fmt::Display for crate::Quantity<#name> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{} {}", self.value(), <#name as crate::Unit>::SYMBOL)
            }
        }
    };

    Ok(expanded)
}

/// Parsed contents of the `#[unit(...)]` attribute.
struct UnitAttribute {
    symbol: LitStr,
    dimension: Expr,
    ratio: Expr,
    // Future extensions:
    // long_name: Option<LitStr>,
    // plural: Option<LitStr>,
    // system: Option<LitStr>,
    // base_unit: Option<bool>,
    // aliases: Option<Vec<LitStr>>,
}

impl Parse for UnitAttribute {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut symbol: Option<LitStr> = None;
        let mut dimension: Option<Expr> = None;
        let mut ratio: Option<Expr> = None;

        while !input.is_empty() {
            let ident: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "symbol" => {
                    symbol = Some(input.parse()?);
                }
                "dimension" => {
                    dimension = Some(input.parse()?);
                }
                "ratio" => {
                    ratio = Some(input.parse()?);
                }
                // Future extensions would be handled here:
                // "long_name" => { ... }
                // "plural" => { ... }
                // "system" => { ... }
                // "base_unit" => { ... }
                // "aliases" => { ... }
                other => {
                    return Err(syn::Error::new(
                        ident.span(),
                        format!("unknown attribute `{}`", other),
                    ));
                }
            }

            // Consume trailing comma if present
            if input.peek(Token![,]) {
                input.parse::<Token![,]>()?;
            }
        }

        let symbol = symbol
            .ok_or_else(|| syn::Error::new(input.span(), "missing required attribute `symbol`"))?;
        let dimension = dimension.ok_or_else(|| {
            syn::Error::new(input.span(), "missing required attribute `dimension`")
        })?;
        let ratio = ratio
            .ok_or_else(|| syn::Error::new(input.span(), "missing required attribute `ratio`"))?;

        Ok(UnitAttribute {
            symbol,
            dimension,
            ratio,
        })
    }
}

fn parse_unit_attribute(attrs: &[Attribute]) -> syn::Result<UnitAttribute> {
    for attr in attrs {
        if attr.path().is_ident("unit") {
            return attr.parse_args::<UnitAttribute>();
        }
    }

    Err(syn::Error::new(
        proc_macro2::Span::call_site(),
        "missing #[unit(...)] attribute",
    ))
}

#[cfg(test)]
mod tests {}
