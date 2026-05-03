// SPDX-License-Identifier: BSD-3-Clause
// Copyright (C) 2026 Vallés Puig, Ramon

//! Derive macro implementation used by `qtty-core`.
//!
//! `qtty-derive` is an implementation detail of this workspace. By default the
//! `Unit` derive expands in terms of `crate::Unit` and `crate::Quantity`, which
//! matches `qtty-core`. Downstream crates can target the public `qtty` facade by
//! adding `crate = qtty` to the helper attribute.
//!
//! Most users should depend on `qtty` instead and use the predefined units.
//!
//! # Generated impls
//!
//! For a unit marker type `MyUnit`, the derive implements:
//!
//! - `crate::Unit for MyUnit`
//! - when targeting the defining crate itself, formatting impls for
//!   `crate::Quantity<MyUnit, S>` (`Display`, `LowerExp`, `UpperExp`)
//!
//! # Attributes
//!
//! The derive reads a required `#[unit(...)]` attribute:
//!
//! - `symbol = "m"`: displayed unit symbol
//! - `dimension = SomeDim`: dimension marker type
//! - `ratio = 1000.0`: conversion ratio to the canonical unit of the dimension
//! - `crate = qtty`: optional crate path when deriving from a downstream crate

#![deny(missing_docs)]
#![forbid(unsafe_code)]

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    ext::IdentExt,
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, DeriveInput, Expr, Ident, LitStr, Path, Token,
};

/// Derive `crate::Unit` and a `Display` impl for `crate::Quantity<ThisUnit>`.
///
/// The derive must be paired with a `#[unit(...)]` attribute providing
/// `symbol`, `dimension`, and `ratio`. Downstream crates using the public
/// `qtty` facade should also set `crate = qtty`.
///
/// Note that downstream crates only receive the `Unit` impl. Formatting impls
/// for `qtty::Quantity<CustomUnit, S>` would violate Rust's orphan rules.
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
    let emit_quantity_formatting = unit_attr
        .crate_path
        .as_ref()
        .is_none_or(|path| path.is_ident("crate"));
    let crate_path = unit_attr
        .crate_path
        .as_ref()
        .map(|path| quote!(#path))
        .unwrap_or_else(|| quote!(crate));

    let expanded = quote! {
        impl #crate_path::Unit for #name {
            const RATIO: f64 = #ratio;
            type Dim = #dimension;
            const SYMBOL: &'static str = #symbol;
        }

    };

    let formatting = if emit_quantity_formatting {
        quote! {
            impl<S: #crate_path::Scalar + ::core::fmt::Display> ::core::fmt::Display for #crate_path::Quantity<#name, S> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    // Forward all format flags (precision, width, fill, …) to the
                    // inner scalar so that e.g. `format!("{:.9}", my_au)` works.
                    ::core::fmt::Display::fmt(&self.value(), f)?;
                    write!(f, " {}", <#name as #crate_path::Unit>::SYMBOL)
                }
            }

            impl<S: #crate_path::Scalar + ::core::fmt::LowerExp> ::core::fmt::LowerExp for #crate_path::Quantity<#name, S> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::LowerExp::fmt(&self.value(), f)?;
                    write!(f, " {}", <#name as #crate_path::Unit>::SYMBOL)
                }
            }

            impl<S: #crate_path::Scalar + ::core::fmt::UpperExp> ::core::fmt::UpperExp for #crate_path::Quantity<#name, S> {
                fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                    ::core::fmt::UpperExp::fmt(&self.value(), f)?;
                    write!(f, " {}", <#name as #crate_path::Unit>::SYMBOL)
                }
            }
        }
    } else {
        TokenStream2::new()
    };

    Ok(quote! {
        #expanded
        #formatting
    })
}

/// Parsed contents of the `#[unit(...)]` attribute.
struct UnitAttribute {
    symbol: LitStr,
    dimension: Expr,
    ratio: Expr,
    crate_path: Option<Path>,
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
        let mut crate_path: Option<Path> = None;

        while !input.is_empty() {
            let ident = Ident::parse_any(input)?;
            input.parse::<Token![=]>()?;

            match ident.to_string().as_str() {
                "crate" => {
                    crate_path = Some(input.parse()?);
                }
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
            crate_path,
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
        assert!(attr.crate_path.is_none());
    }

    #[test]
    fn test_parse_unit_attribute_with_crate_path() {
        let input: DeriveInput = parse_quote! {
            #[unit(crate = qtty, symbol = "m", dimension = qtty::Length, ratio = 1.0)]
            pub enum Meter {}
        };

        let attr = parse_unit_attribute(&input.attrs).unwrap();
        assert_eq!(attr.symbol.value(), "m");
        let crate_path = attr.crate_path.as_ref().unwrap();
        assert_eq!(quote!(#crate_path).to_string(), "qtty");
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
    fn test_derive_unit_impl_with_downstream_crate_path() {
        let input: DeriveInput = parse_quote! {
            #[unit(crate = qtty, symbol = "smoot", dimension = qtty::Length, ratio = 1.7018)]
            pub enum Smoot {}
        };

        let result = derive_unit_impl(input);
        assert!(result.is_ok());
        let tokens = result.unwrap();
        let code = tokens.to_string();
        assert!(code.contains("impl qtty :: Unit for Smoot"));
        assert!(!code.contains("for qtty :: Quantity < Smoot , S >"));
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
