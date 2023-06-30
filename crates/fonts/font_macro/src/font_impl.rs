use proc_macro::TokenStream;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use proc_macro2::Span;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{AttributeArgs, Ident, ItemEnum, LitStr, Path};

use attr::Attr;

pub fn parse(args: &AttributeArgs, item: &ItemEnum) -> TokenStream {
    // Parse parent args
    let (family, font_typ) = {
        let mut family = None;
        let mut font_typ = None;

        for attr in Attr::parse_nested_meta(args) {
            match attr {
                Attr::Family(f) => family = Some(f),
                Attr::FontTyp(t) => font_typ = Some(t),
                _ => {}
            }
        }

        (
            family.expect("font family name required"),
            font_typ.unwrap_or_else(|| util::str_to_path("::iced::Font")),
        )
    };

    // Collect font for each variant
    let (font_map, variant_map) = collect_fonts(item, &family);

    // Define everything
    let fonts_def = define_fonts(
        &font_typ,
        font_map.keys(),
        font_map.values().map(|v| &v.name),
        font_map.values().map(|v| &v.source),
    );
    let enum_def = define_enum(item);
    let convert_def = define_conversions(item, &font_typ, variant_map.values(), variant_map.keys());

    // Finally
    quote! {
        #fonts_def
        #enum_def
        #convert_def
    }
    .into()
}

/// Just re-defines the enum without our custom attributes.
fn define_enum(item: &ItemEnum) -> TokenStream2 {
    let mut item = item.clone();

    // Filter `font` attributes.
    for variant in &mut item.variants {
        variant.attrs = variant
            .attrs
            .clone()
            .into_iter()
            .filter(|attr| !attr.path.is_ident("font"))
            .collect();
    }

    quote! {
        #item
    }
}

/// Define static global fonts.
fn define_fonts<'a>(
    font: &'a Path,
    hashes: impl Iterator<Item = &'a Ident>,
    names: impl Iterator<Item = &'a String>,
    sources: impl Iterator<Item = &'a String>,
) -> TokenStream2 {
    quote! {
        #(
            #[doc(hidden)]
            static #hashes: #font = #font::External {
                name: #names,
                bytes: ::std::include_bytes!(#sources),
            };
        )*
    }
}

/// Define conversions between the font wrapper type and iced fonts.
fn define_conversions<'a>(
    item: &'a ItemEnum,
    font: &'a Path,
    hashes: impl Iterator<Item = &'a Ident>,
    variants: impl Iterator<Item = &'a &'a Ident>,
) -> TokenStream2 {
    let name = &item.ident;

    quote! {
        impl From<#name> for #font {
            fn from(thing: #name) -> #font {
                match thing {
                    #(#name::#variants => #hashes),*
                }
            }
        }
    }
}

#[derive(Debug, Hash)]
struct FontVariant {
    name: String,
    source: String,
}

/// Go through each variant of the enum and create a `FontVariant` for each uniquely defined
/// font. These `FontVariant`s will be statically defined at the call-site.
fn collect_fonts<'a>(
    item: &'a ItemEnum,
    family: &'a LitStr,
) -> (HashMap<Ident, FontVariant>, HashMap<&'a Ident, Ident>) {
    let this_run: usize = rand::random();

    let mut font_map = HashMap::new();
    let mut variant_map = HashMap::new();

    for variant in &item.variants {
        let (name, source) = {
            let mut name = None;
            let mut source = None;

            for attr in Attr::parse_attrs(&variant.attrs) {
                match attr {
                    Attr::Variant(f) => name = Some(f),
                    Attr::Source(t) => source = Some(t),
                    _ => {}
                }
            }

            (
                name.expect("expected variant name"),
                source.expect("expected source"),
            )
        };

        let font_variant = FontVariant {
            name: format!("{} {}", family.value(), name.value()),
            source: source.value(),
        };

        let hash = {
            let mut hasher = DefaultHasher::new();
            this_run.hash(&mut hasher);
            font_variant.hash(&mut hasher);
            let string = format!("__{:x}", hasher.finish());
            Ident::new(&string, Span::call_site())
        };

        font_map.insert(hash.clone(), font_variant);
        variant_map.insert(&variant.ident, hash);
    }

    (font_map, variant_map)
}

mod attr {
    use proc_macro2::{Ident, Span};
    use quote::ToTokens;
    use syn::parse::{Parse, ParseStream, Parser};
    use syn::punctuated::Punctuated;
    use syn::{Attribute, LitStr, NestedMeta, Path, Token};

    use super::util;

    #[derive(Debug)]
    pub enum Attr {
        Family(LitStr),
        FontTyp(Path),
        Variant(LitStr),
        Source(LitStr),
    }

    impl Attr {
        pub fn parse_nested_meta(nested: &[NestedMeta]) -> Vec<Self> {
            let parser = Self::parse;

            nested
                .iter()
                .map(|meta| {
                    let tokens = meta.to_token_stream();
                    parser.parse2(tokens).unwrap()
                })
                .collect()
        }

        pub fn parse_attrs(attrs: &[Attribute]) -> Vec<Self> {
            attrs
                .iter()
                .filter(|attr| attr.path.is_ident("font"))
                .flat_map(|attr| {
                    attr.parse_args_with(Punctuated::<Self, Token![,]>::parse_terminated)
                        .unwrap()
                })
                .collect()
        }
    }

    impl Parse for Attr {
        fn parse(input: ParseStream) -> syn::Result<Self> {
            let name = input.parse::<Ident>()?.to_string();

            match name.as_str() {
                "family" => {
                    let _assign_token = input.parse::<Token![=]>()?;
                    let lit = input.parse::<LitStr>()?;
                    Ok(Self::Family(lit))
                }
                "font_typ" => {
                    let _assign_token = input.parse::<Token![=]>()?;
                    let path_str = input.parse::<LitStr>()?;
                    Ok(Self::FontTyp(util::lit_str_to_path(&path_str)))
                }
                "variant" => {
                    let _assign_token = input.parse::<Token![=]>()?;
                    let lit = input.parse::<LitStr>()?;
                    Ok(Self::Variant(lit))
                }
                "source" => {
                    let _assign_token = input.parse::<Token![=]>()?;
                    let lit = input.parse::<LitStr>()?;
                    Ok(Self::Source(lit))
                }
                _ => Err(syn::Error::new(
                    Span::call_site(),
                    format!("unknown argument `{name}`"),
                )),
            }
        }
    }
}

mod util {
    use proc_macro2::{Ident, Span};
    use syn::{punctuated::Punctuated, LitStr, Path, Token};

    pub fn lit_str_to_path(lit_str: &LitStr) -> Path {
        str_to_path(&lit_str.value())
    }

    pub fn str_to_path(value: &str) -> Path {
        let mut split = value.split("::").peekable();

        let leading_colon = if let Some(&"") = split.peek() {
            split.next().unwrap();
            Some(Token![::](Span::call_site()))
        } else {
            None
        };

        let mut path = Path {
            leading_colon,
            segments: Punctuated::new(),
        };

        for ident in split {
            let ident = Ident::new(ident, Span::call_site());
            path.segments.push(ident.into());
        }

        path
    }
}
