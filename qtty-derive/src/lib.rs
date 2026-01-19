//! Derive macro implementation used by `qtty-core`.
//!
//! `qtty-derive` is an implementation detail of this workspace. The `Unit` derive expands in terms of `crate::Unit`
//! and `crate::Quantity`, so it is intended to be used by `qtty-core` (or by crates that expose an identical
//! crate-root API).
//!
//! Most users should depend on `qtty` instead and use the predefined units.
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
/// This macro is intended for use by `qtty-core`.
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

        impl<S: crate::scalar::Real> ::core::fmt::Display for crate::Quantity<#name, S> {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{} {}", self.value().to_f64(), <#name as crate::Unit>::SYMBOL)
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
mod tests {
    use super::*;
    use quote::quote;
    use syn::parse_quote;

    #[test]
    fn test_parse_unit_attribute_complete() {
        let input: DeriveInput = parse_quote! {
            #[unit(symbol = "m", dimension = Length, ratio = 1.0)]
            pub enum Meter {}
        };

        let attr = parse_unit_attribute(&input.attrs).unwrap();
        assert_eq!(attr.symbol.value(), "m");
    }

    #[test]
    fn test_parse_unit_attribute_missing() {
        let input: DeriveInput = parse_quote! {
            pub enum Meter {}
        };

        let result = parse_unit_attribute(&input.attrs);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let err_msg = err.to_string();
        assert!(err_msg.contains("missing #[unit(...)] attribute"));
    }

    #[test]
    fn test_parse_unit_attribute_missing_symbol() {
        let input: DeriveInput = parse_quote! {
            #[unit(dimension = Length, ratio = 1.0)]
            pub enum Meter {}
        };

        let result = parse_unit_attribute(&input.attrs);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let err_msg = err.to_string();
        assert!(err_msg.contains("missing required attribute `symbol`"));
    }

    #[test]
    fn test_parse_unit_attribute_missing_dimension() {
        let input: DeriveInput = parse_quote! {
            #[unit(symbol = "m", ratio = 1.0)]
            pub enum Meter {}
        };

        let result = parse_unit_attribute(&input.attrs);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let err_msg = err.to_string();
        assert!(err_msg.contains("missing required attribute `dimension`"));
    }

    #[test]
    fn test_parse_unit_attribute_missing_ratio() {
        let input: DeriveInput = parse_quote! {
            #[unit(symbol = "m", dimension = Length)]
            pub enum Meter {}
        };

        let result = parse_unit_attribute(&input.attrs);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let err_msg = err.to_string();
        assert!(err_msg.contains("missing required attribute `ratio`"));
    }

    #[test]
    fn test_parse_unit_attribute_unknown_field() {
        let input: DeriveInput = parse_quote! {
            #[unit(symbol = "m", dimension = Length, ratio = 1.0, unknown = "value")]
            pub enum Meter {}
        };

        let result = parse_unit_attribute(&input.attrs);
        assert!(result.is_err());
        let err = result.err().unwrap();
        let err_msg = err.to_string();
        assert!(err_msg.contains("unknown attribute"));
    }

    #[test]
    fn test_derive_unit_impl_basic() {
        let input: DeriveInput = parse_quote! {
            #[unit(symbol = "m", dimension = Length, ratio = 1.0)]
            pub enum Meter {}
        };

        let result = derive_unit_impl(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let code = tokens.to_string();
        assert!(code.contains("impl crate :: Unit for Meter"));
        assert!(code.contains("const RATIO : f64 = 1.0"));
        assert!(code.contains("const SYMBOL : & 'static str = \"m\""));
        assert!(code.contains("type Dim = Length"));
    }

    #[test]
    fn test_derive_unit_impl_with_expression_ratio() {
        let input: DeriveInput = parse_quote! {
            #[unit(symbol = "km", dimension = Length, ratio = 1000.0)]
            pub enum Kilometer {}
        };

        let result = derive_unit_impl(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let code = tokens.to_string();
        assert!(code.contains("const RATIO : f64 = 1000.0"));
    }

    #[test]
    fn test_unit_attribute_parse_with_trailing_comma() {
        let tokens = quote! {
            symbol = "m", dimension = Length, ratio = 1.0,
        };
        let attr: UnitAttribute = syn::parse2(tokens).unwrap();
        assert_eq!(attr.symbol.value(), "m");
    }

    #[test]
    fn test_unit_attribute_parse_no_trailing_comma() {
        let tokens = quote! {
            symbol = "m", dimension = Length, ratio = 1.0
        };
        let attr: UnitAttribute = syn::parse2(tokens).unwrap();
        assert_eq!(attr.symbol.value(), "m");
    }

    #[test]
    fn test_unit_attribute_parse_duplicate_symbol() {
        // Parser accepts duplicates - last one wins
        let tokens = quote! {
            symbol = "m", symbol = "km", dimension = Length, ratio = 1.0
        };
        let attr: UnitAttribute = syn::parse2(tokens).unwrap();
        assert_eq!(attr.symbol.value(), "km");
    }

    #[test]
    fn test_parse_empty_attribute() {
        let tokens = quote! {};
        let result: syn::Result<UnitAttribute> = syn::parse2(tokens);
        assert!(result.is_err());
    }

    #[test]
    fn test_derive_unit_impl_error_path() {
        // Test error handling in derive_unit_impl
        let input: DeriveInput = parse_quote! {
            pub enum Meter {}
        };
        let result = derive_unit_impl(input);
        assert!(result.is_err());
        // The error should contain information about missing attribute
        let err = result.err().unwrap();
        let err_tokens = err.to_compile_error();
        let code = err_tokens.to_string();
        assert!(code.contains("compile_error"));
    }
}
