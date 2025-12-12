//! # Units Derive
//!
//! Procedural macro derives for creating unit types in the `units-core` system.
//!
//! ## Usage
//!
//! ```rust,ignore
//! use units_core::{Dimension, Quantity};
//! use units_derive::Unit;
//!
//! pub enum Length {}
//! impl Dimension for Length {}
//!
//! #[derive(Unit)]
//! #[unit(symbol = "m", dimension = Length, ratio = 1.0)]
//! pub struct Meter;
//!
//! pub type Meters = Quantity<Meter>;
//! ```
//!
//! ## Attributes
//!
//! The `#[unit(...)]` attribute supports the following fields:
//!
//! - `symbol` (required): The display symbol for the unit (e.g., "m", "km", "s")
//! - `dimension` (required): The dimension type this unit belongs to
//! - `ratio` (required): The conversion ratio to the canonical unit of this dimension
//!
//! ## Future Extensions
//!
//! The macro is designed to easily support future attributes:
//! - `long_name`: Full name of the unit ("meter", "kilometer")
//! - `plural`: Plural form ("meters", "kilometers")
//! - `system`: Unit system ("SI", "Imperial")
//! - `base_unit`: Whether this is the base unit for the dimension
//! - `aliases`: Alternative symbols

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, Attribute, DeriveInput, Expr, Ident, LitStr, Token,
};

/// Derive macro for creating unit types.
///
/// This macro generates:
/// - A zero-sized enum type (if input is a struct, converts to enum)
/// - `Unit` trait implementation
/// - `Display` implementation for `Quantity<TheUnit>`
///
/// # Example
///
/// ```rust,ignore
/// #[derive(Unit)]
/// #[unit(symbol = "m", dimension = Length, ratio = 1.0)]
/// pub struct Meter;
/// ```
///
/// Expands to:
///
/// ```rust,ignore
/// #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
/// pub enum Meter {}
///
/// impl Unit for Meter {
///     const RATIO: f64 = 1.0;
///     type Dim = Length;
///     const SYMBOL: &'static str = "m";
/// }
///
/// impl core::fmt::Display for Quantity<Meter> {
///     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
///         write!(f, "{} {}", self.value(), <Meter as Unit>::SYMBOL)
///     }
/// }
/// ```
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
    
    // Generate the output - Note: we don't generate the type itself since derive macros
    // are additive. The struct/enum is already defined by the user.
    // We only implement the Unit trait and Display for Quantity<T>.
    //
    // We use `crate::` path because all unit definitions live in units-core,
    // which re-exports this derive macro. External users should use the
    // `siderust-units` crate which re-exports everything.
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
        
        let symbol = symbol.ok_or_else(|| {
            syn::Error::new(input.span(), "missing required attribute `symbol`")
        })?;
        let dimension = dimension.ok_or_else(|| {
            syn::Error::new(input.span(), "missing required attribute `dimension`")
        })?;
        let ratio = ratio.ok_or_else(|| {
            syn::Error::new(input.span(), "missing required attribute `ratio`")
        })?;
        
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
    // Proc macro tests need to be in a separate test crate
    // that depends on the macro. We'll test via integration
    // tests in the main units crate.
}
