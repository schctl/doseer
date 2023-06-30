//! Proc macro implementations for sleet.

use proc_macro::TokenStream;

use syn::{parse_macro_input, AttributeArgs, ItemEnum};

mod font_impl;

/// Generate a font family.
///
/// # Examples
///
/// ```rust,ignore
/// use sleet_font_macro::font;
///
/// #[font(family = "Roboto")]
/// pub enum Roboto {
///     #[font(variant = "Regular")]
///     #[font(source = "res/fonts/Roboto-Regular.ttf")]
///     Regular
/// }
/// ```
#[proc_macro_attribute]
pub fn font(args: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as AttributeArgs);
    let item = parse_macro_input!(item as ItemEnum);

    font_impl::parse(&args, &item)
}
